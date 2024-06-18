use netsblox_ast as ast;
use netsblox_ast::compact_str::{CompactString, format_compact};

use std::collections::{VecDeque, BTreeMap};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum CompileError {
    ParseError(Box<ast::Error>),
    RoleCount { count: usize },
    UnknownRole { name: CompactString },
    UnsupportedBlock { info: CompactString },
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

pub fn translate_expr(expr: &ast::Expr) -> Result<CompactString, CompileError> {
    Ok(match &expr.kind {
        ast::ExprKind::Value(ast::Value::String(x)) => x.clone(),
        ast::ExprKind::Greater { left, right } => format_compact!("{} > {}", translate_expr(&left)?, translate_expr(&right)?),
        ast::ExprKind::Timer => "t".into(),
        x => return Err(CompileError::UnsupportedBlock { info: format_compact!("{x:?}") }),
    })
}
pub fn parse_stmts(state_machine_name: &str, stmts: &[ast::Stmt]) -> Result<(Vec<CompactString>, Vec<Transition>), CompileError> {
    let mut actions = vec![];
    let mut transitions = vec![];

    let mut stmts = stmts.iter().rev().fuse();
    while let Some(stmt) = stmts.next() {
        match &stmt.kind {
            ast::StmtKind::Assign { var, value } if var.name == state_machine_name => match &value.kind {
                ast::ExprKind::Value(ast::Value::String(x)) => transitions.push(Transition { condition: None, actions: Default::default(), new_state: x.clone() }),
                _ => (),
            }
            ast::StmtKind::If { condition, then } => {
                let condition = translate_expr(condition)?;
                let (sub_actions, mut sub_transitions) = parse_stmts(state_machine_name, &then)?;
                for sub_transition in sub_transitions.iter_mut() {
                    sub_transition.condition = Some(sub_transition.condition.take().map(|x| format_compact!("{condition} and {x}")).unwrap_or_else(|| condition.clone()));
                    for sub_action in sub_actions.iter().rev() {
                        sub_transition.actions.push_front(sub_action.clone());
                    }
                }
                transitions.extend(sub_transitions.into_iter());
            }
            _ => (),
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

            let (actions, transitions) = parse_stmts(&state_machine_name, &script.stmts)?;
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
