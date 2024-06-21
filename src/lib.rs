use netsblox_ast as ast;
use netsblox_ast::compact_str::{CompactString, format_compact};

use std::collections::{VecDeque, BTreeMap, BTreeSet};

#[cfg(test)]
mod test;

macro_rules! count_exprs {
    () => { 0usize };
    ($head:expr $(,$tail:expr)* $(,)?) => { 1usize + count_exprs!($($tail),*) };
}
macro_rules! deque {
    ($($values:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut res = VecDeque::with_capacity(count_exprs!($($values),*));
        $(res.push_back($values);)*
        res
    }}
}

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
    MultipleHandlers { state_machine: CompactString, state: CompactString },
    ComplexTransitionName { state_machine: CompactString, state: CompactString },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    pub name: CompactString,
    pub role: CompactString,
    pub state_machines: BTreeMap<CompactString, StateMachine>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct StateMachine {
    pub variables: BTreeSet<CompactString>,
    pub states: BTreeMap<CompactString, State>,
    pub initial_state: Option<CompactString>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub actions: VecDeque<CompactString>,
    pub transitions: VecDeque<Transition>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Transition {
    pub condition: Option<CompactString>,
    pub actions: VecDeque<CompactString>,
    pub new_state: CompactString,
}

#[derive(Default)]
struct Context {
    variables: Vec<ast::VariableRef>,
}

fn translate_expr(state_machine: &str, state: &str, expr: &ast::Expr, context: &mut Context) -> Result<CompactString, CompileError> {
    Ok(match &expr.kind {
        ast::ExprKind::Variable { var } => {
            context.variables.push(var.clone());
            var.trans_name.clone()
        }
        ast::ExprKind::Value(ast::Value::String(x)) => x.clone(),
        ast::ExprKind::Eq { left, right } => format_compact!("{} == {}", translate_expr(state_machine, state, &left, context)?, translate_expr(state_machine, state, &right, context)?),
        ast::ExprKind::Greater { left, right } => format_compact!("{} > {}", translate_expr(state_machine, state, &left, context)?, translate_expr(state_machine, state, &right, context)?),
        ast::ExprKind::Timer => "t".into(),
        x => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
    })
}
fn parse_actions(state_machine: &str, state: &str, stmt: &ast::Stmt, context: &mut Context) -> Result<Vec<CompactString>, CompileError> {
    Ok(match &stmt.kind {
        ast::StmtKind::Assign { var, value } => {
            context.variables.push(var.clone());
            vec![format_compact!("{} = {}", var.trans_name, translate_expr(state_machine, state, value, context)?)]
        }
        ast::StmtKind::ResetTimer => vec!["t = 0".into()],
        x => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
    })
}
fn parse_transitions(state_machine: &str, state: &str, stmt: &ast::Stmt, terminal: bool, context: &mut Context) -> Result<Option<(VecDeque<CompactString>, VecDeque<Transition>, Option<CompactString>, bool)>, CompileError> {
    Ok(match &stmt.kind {
        ast::StmtKind::Assign { var, value } if var.name == state_machine => match &value.kind {
            ast::ExprKind::Value(ast::Value::String(x)) => match terminal {
                true => Some((deque![], deque![Transition { condition: None, actions: <_>::default(), new_state: x.clone() }], None, true)),
                false => return Err(CompileError::NonTerminalTransition { state_machine: state_machine.into(), state: state.into() }),
            }
            _ => return Err(CompileError::ComplexTransitionName { state_machine: state_machine.into(), state: state.into() }),
        }
        ast::StmtKind::If { condition, then } => {
            let condition = translate_expr(state_machine, state, condition, context)?;
            let (actions, mut transitions, body_terminal) = parse_stmts(state_machine, state, &then, terminal, context)?;

            if !actions.is_empty() && !body_terminal {
                return Err(CompileError::NonTerminalTransition { state_machine: state_machine.into(), state: state.into() });
            }

            for transition in transitions.iter_mut() {
                transition.condition = Some(transition.condition.take().map(|x| format_compact!("{condition} & {x}")).unwrap_or_else(|| condition.clone()));
                transition.actions.extend_front(actions.iter().cloned());
            }

            Some((deque![], transitions, Some(format_compact!("~({condition})")), terminal))
        }
        ast::StmtKind::IfElse { condition, then, otherwise } => {
            let condition = translate_expr(state_machine, state, condition, context)?;

            let (mut actions_1, mut transitions_1, body_terminal_1) = parse_stmts(state_machine, state, &then, terminal, context)?;
            let (mut actions_2, mut transitions_2, body_terminal_2) = parse_stmts(state_machine, state, &otherwise, terminal, context)?;

            let res_actions = match actions_1 == actions_2 {
                true => {
                    actions_2.clear();
                    core::mem::take(&mut actions_1)
                }
                false => deque![],
            };

            if (!actions_1.is_empty() && !body_terminal_1) || (!actions_2.is_empty() && !body_terminal_2) {
                return Err(CompileError::NonTerminalTransition { state_machine: state_machine.into(), state: state.into() });
            }

            for transition in transitions_1.iter_mut() {
                transition.condition = Some(transition.condition.take().map(|x| format_compact!("{condition} & {x}")).unwrap_or_else(|| condition.clone()));
                transition.actions.extend_front(actions_1.iter().cloned());
            }
            for transition in transitions_2.iter_mut() {
                transition.condition = Some(transition.condition.take().map(|x| format_compact!("~({condition}) & {x}")).unwrap_or_else(|| format_compact!("~({condition})")));
                transition.actions.extend_front(actions_2.iter().cloned());
            }

            let tail_condition = match (body_terminal_1, body_terminal_2) {
                (true, true) => Some("false".into()),
                (true, false) => Some(format_compact!("~({condition})")),
                (false, true) => Some(condition.clone()),
                (false, false) => None,
            };

            transitions_1.extend(transitions_2.into_iter());
            Some((res_actions, transitions_1, tail_condition, terminal || (body_terminal_1 && body_terminal_2)))
        }
        _ => None,
    })
}
fn parse_stmts(state_machine: &str, state: &str, stmts: &[ast::Stmt], mut terminal: bool, context: &mut Context) -> Result<(VecDeque<CompactString>, VecDeque<Transition>, bool), CompileError> {
    let mut actions: VecDeque<CompactString> = <_>::default();
    let mut transitions: VecDeque<Transition> = <_>::default();

    let mut stmts = stmts.iter().rev().peekable();
    while let Some(stmt) = stmts.peek() {
        match &stmt.kind {
            ast::StmtKind::Return { value: _ } => (),
            _ => break,
        }
        stmts.next();
        terminal = true;
    }

    let mut last = true;
    for stmt in stmts {
        match parse_transitions(state_machine, state, stmt, terminal && last, context)? {
            Some((sub_actions, sub_transitions, tail_condition, sub_terminal)) => {
                terminal |= sub_terminal;
                for transition in transitions.iter_mut() {
                    if let Some(tail_condition) = tail_condition.as_ref() {
                        transition.condition = Some(transition.condition.take().map(|x| format_compact!("{tail_condition} & {x}")).unwrap_or_else(|| tail_condition.clone()));
                    }
                    transition.actions.extend_front(actions.iter().cloned());
                }
                actions = sub_actions;
                transitions.extend_front(sub_transitions.into_iter());
            }
            None => actions.extend_front(parse_actions(state_machine, state, stmt, context)?.into_iter()),
        }
        last = false;
    }

    Ok((actions, transitions, terminal))
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

    let mut state_machines: BTreeMap<_, StateMachine> = <_>::default();
    let mut visited_handlers: BTreeSet<(CompactString, CompactString)> = <_>::default();
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

            if !visited_handlers.insert((state_machine_name.clone(), state_name.clone())) {
                return Err(CompileError::MultipleHandlers { state_machine: state_machine_name.clone(), state: state_name.clone() });
            }

            let state_machine = state_machines.entry(state_machine_name.clone()).or_insert_with(|| StateMachine { variables: <_>::default(), states: <_>::default(), initial_state: <_>::default() });
            let state = state_machine.states.entry(state_name.clone()).or_insert_with(|| State { actions: <_>::default(), transitions: <_>::default() });
            debug_assert_eq!(state.transitions.len(), 0);
            debug_assert_eq!(state.actions.len(), 0);

            let mut context = Context::default();
            let (actions, transitions, _) = parse_stmts(&state_machine_name, &state_name, &script.stmts, true, &mut context)?;
            let target_states = transitions.iter().map(|x| x.new_state.clone()).collect::<Vec<_>>();
            state.transitions.extend_front(transitions.into_iter());
            state.actions.extend_front(actions.into_iter());
            for target_state in target_states {
                state_machine.states.entry(target_state).or_insert_with(|| State { actions: <_>::default(), transitions: <_>::default() });
            }
            for variable in context.variables {
                state_machine.variables.insert(variable.trans_name);
            }
        }
    }

    Ok(Project { name: proj.name, role: role.name.clone(), state_machines })
}
