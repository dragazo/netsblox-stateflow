use netsblox_ast as ast;
use netsblox_ast::compact_str::{CompactString, format_compact};

use std::collections::{VecDeque, BTreeMap};

#[cfg(test)]
mod test;

trait VecDequeUtil<T> {
    fn extend_front<I: Iterator<Item = T> + DoubleEndedIterator>(&mut self, iter: I);
}
impl<T> VecDequeUtil<T> for VecDeque<T> {
    fn extend_front<I: Iterator<Item = T> + DoubleEndedIterator>(&mut self, iter: I) {
        for val in iter.rev() {
            self.push_front(val);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompileError {
    ParseError(Box<ast::Error>),
    RoleCount { count: usize },
    UnknownRole { name: CompactString },
    UnsupportedBlock { state_machine: CompactString, state: CompactString, info: CompactString },
    NonTerminalTransition { state_machine: CompactString, state: CompactString },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    pub name: CompactString,
    pub role: CompactString,
    pub state_machines: BTreeMap<CompactString, StateMachine>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct StateMachine {
    pub states: BTreeMap<CompactString, State>,
    pub initial_state: Option<CompactString>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub actions: VecDeque<CompactString>,
    pub transitions: Vec<Transition>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Transition {
    pub condition: Option<CompactString>,
    pub actions: VecDeque<CompactString>,
    pub new_state: CompactString,
}

pub fn translate_expr(state_machine: &str, state: &str, expr: &ast::Expr) -> Result<CompactString, CompileError> {
    Ok(match &expr.kind {
        ast::ExprKind::Value(ast::Value::String(x)) => x.clone(),
        ast::ExprKind::Greater { left, right } => format_compact!("{} > {}", translate_expr(state_machine, state, &left)?, translate_expr(state_machine, state, &right)?),
        ast::ExprKind::Timer => "t".into(),
        x => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
    })
}
pub fn parse_transitions(state_machine: &str, state: &str, stmt: &ast::Stmt, terminal: bool) -> Result<Option<Vec<Transition>>, CompileError> {
    Ok(match &stmt.kind {
        ast::StmtKind::Assign { var, value } if terminal && var.name == state_machine => match &value.kind {
            ast::ExprKind::Value(ast::Value::String(x)) => Some(vec![Transition { condition: None, actions: Default::default(), new_state: x.clone() }]),
            _ => None,
        }
        ast::StmtKind::If { condition, then } => {
            let condition = translate_expr(state_machine, state, condition)?;
            let (actions, mut transitions) = parse_stmts(state_machine, state, &then, terminal)?;
            for transition in transitions.iter_mut() {
                transition.condition = Some(transition.condition.take().map(|x| format_compact!("{condition} & {x}")).unwrap_or_else(|| condition.clone()));
                transition.actions.extend_front(actions.iter().cloned().rev());
            }
            Some(transitions)
        }
        _ => None,
    })
}
pub fn parse_stmts(state_machine: &str, state: &str, stmts: &[ast::Stmt], terminal: bool) -> Result<(Vec<CompactString>, Vec<Transition>), CompileError> {
    let mut actions = vec![];
    let mut transitions = vec![];

    let mut stmts = stmts.iter().rev().peekable();
    if terminal {
        let mut last = true;
        while let Some(stmt) = stmts.peek() {
            match parse_transitions(state_machine, state, stmt, last)? {
                Some(sub_transitions) => transitions.extend(sub_transitions.into_iter()),
                None => break,
            }
            stmts.next();
            last = false;
        }
    }
    while let Some(stmt) = stmts.next() {
        match parse_transitions(state_machine, state, stmt, true) {
            Ok(Some(_)) => return Err(CompileError::NonTerminalTransition { state_machine: state_machine.into(), state: state.into() }),
            _ => (),
        }
        match &stmt.kind {
            ast::StmtKind::ResetTimer => actions.push("t = 0".into()),
            x => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
        }
    }

    Ok((actions, transitions))
}

pub fn compile(xml: &str, role: Option<&str>) -> Result<Project, CompileError> {
    let proj = ast::Parser::default().parse(xml).map_err(|e| CompileError::ParseError(e))?;
    let role = match role {
        Some(name) => match proj.roles.iter().find(|r| r.name == name) {
            Some(x) => x,
            None => return Err(CompileError::UnknownRole { name: name.into() }),
        }
        None => match proj.roles.as_slice() {
            [x] => x,
            x => return Err(CompileError::RoleCount { count: x.len() }),
        }
    };

    let mut state_machines: BTreeMap<_, StateMachine> = Default::default();
    for entity in role.entities.iter() {
        for script in entity.scripts.iter() {
            let (state_machine_name, state_name) = match script.hat.as_ref().map(|x| &x.kind) {
                Some(ast::HatKind::When { condition }) => match &condition.kind {
                    ast::ExprKind::Eq { left, right } => match (&left.kind, &right.kind) {
                        (ast::ExprKind::Variable { var }, ast::ExprKind::Value(ast::Value::String(val))) => (&var.name, val),
                        (ast::ExprKind::Value(ast::Value::String(val)), ast::ExprKind::Variable { var }) => (&var.name, val),
                        _ => continue,
                    }
                    _ => continue,
                }
                _ => continue,
            };

            let state_machine = state_machines.entry(state_machine_name.clone()).or_insert_with(|| StateMachine { states: Default::default(), initial_state: Default::default() });
            let state = state_machine.states.entry(state_name.clone()).or_insert_with(|| State { actions: Default::default(), transitions: Default::default() });

            let (actions, transitions) = parse_stmts(&state_machine_name, &state_name, &script.stmts, true)?;
            let target_states = transitions.iter().map(|x| x.new_state.clone()).collect::<Vec<_>>();
            state.transitions.extend(transitions.into_iter());
            state.actions.extend(actions.into_iter());
            for target_state in target_states {
                state_machine.states.entry(target_state).or_insert_with(|| State { actions: Default::default(), transitions: Default::default() });
            }
        }
    }

    Ok(Project { name: proj.name, role: role.name.clone(), state_machines })
}
