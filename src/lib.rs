#![forbid(unsafe_code)]
#![no_std]

#[macro_use]
extern crate alloc;

use netsblox_ast as ast;
use netsblox_ast::compact_str::{CompactString, ToCompactString, format_compact};

use graphviz_rust::dot_structures as dot;

use alloc::collections::{VecDeque, BTreeMap};
use alloc::fmt::Write as _;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::{ToString, String};

pub use graphviz_rust as graphviz;

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

mod condition;
pub use condition::*;

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

fn punctuate<'a, I: Iterator<Item = &'a str>>(mut values: I, sep: &str) -> Option<(CompactString, usize)> {
    let mut res = CompactString::new(values.next()?);
    let mut separators = 0;
    for x in values {
        res.push_str(sep);
        res.push_str(x);
        separators += 1;
    }
    Some((res, separators))
}

fn common_suffix<T: PartialEq, I: Iterator<Item = J>, J: Iterator<Item = T> + DoubleEndedIterator>(mut sequences: I) -> Vec<T> {
    let mut suffix: Vec<T> = match sequences.next() {
        None => return <_>::default(),
        Some(x) => x.collect(),
    };
    for sequence in sequences {
        let common = suffix.iter().rev().zip(sequence.rev()).take_while(|(a, b)| *a == b).count();
        suffix.drain(..suffix.len() - common);
    }
    suffix
}
#[test]
fn test_common_suffix() {
    assert_eq!(common_suffix(Vec::<alloc::vec::IntoIter<i32>>::new().into_iter()), &[]);
    assert_eq!(common_suffix([vec![1, 2, 3].into_iter()].into_iter()), &[1, 2, 3]);
    assert_eq!(common_suffix([vec![1, 2, 3].into_iter(), vec![2, 1, 3].into_iter()].into_iter()), &[3]);
    assert_eq!(common_suffix([vec![1, 2, 3].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[2, 3]);
    assert_eq!(common_suffix([vec![1, 2, 3].into_iter(), vec![2, 3].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[2, 3]);
    assert_eq!(common_suffix([vec![1, 2, 3].into_iter(), vec![3, 3].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[3]);
    assert_eq!(common_suffix([vec![1, 2, 3].into_iter(), vec![3, 4].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[]);
    assert_eq!(common_suffix([vec![2, 2, 3].into_iter(), vec![2, 2, 3].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[2, 2, 3]);
    assert_eq!(common_suffix([vec![2, 2, 3].into_iter(), vec![2, 2, 4].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[]);
    assert_eq!(common_suffix([vec![2, 2, 3].into_iter(), vec![2, 1, 3].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[3]);
    assert_eq!(common_suffix([vec![2, 2, 3].into_iter(), vec![1, 2, 3].into_iter(), vec![2, 2, 3].into_iter()].into_iter()), &[2, 3]);
}

struct RenamePool<F: for<'a> FnMut(&'a str) -> Result<CompactString, ()>> {
    forward: BTreeMap<CompactString, CompactString>,
    backward: BTreeMap<CompactString, CompactString>,
    f: F,
}
impl<F: for<'a> FnMut(&'a str) -> Result<CompactString, ()>> RenamePool<F> {
    fn new(f: F) -> Self {
        Self { forward: Default::default(), backward: Default::default(), f }
    }
    fn rename(&mut self, x: &str) -> Result<CompactString, CompileError> {
        if let Some(res) = self.forward.get(x) {
            return Ok(res.clone());
        }

        let y = (self.f)(x).map_err(|()| CompileError::RenameFailure { before: x.into() })?;
        assert!(self.forward.insert(x.into(), y.clone()).is_none());

        if let Some(prev) = self.backward.insert(y.clone(), x.into()) {
            return Err(CompileError::RenameConflict { before: (x.into(), prev.clone()), after: y });
        }

        Ok(y)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompileError {
    ParseError(Box<ast::Error>),

    RoleCount { count: usize },
    UnknownRole { name: CompactString },

    RenameFailure { before: CompactString },
    RenameConflict { before: (CompactString, CompactString), after: CompactString },

    TransitionEmptyTarget { state_machine: CompactString, state: CompactString },
    UnsupportedBlock { state_machine: CompactString, state: CompactString, info: CompactString },
    NonTerminalTransition { state_machine: CompactString, state: CompactString },
    MultipleHandlers { state_machine: CompactString, state: CompactString },
    ComplexTransitionName { state_machine: CompactString, state: CompactString },
    VariadicBlocks { state_machine: CompactString, state: CompactString },
    ActionsOutsideTransition { state_machine: CompactString, state: CompactString },
    VariableOverlap { state_machines: (CompactString, CompactString), variable: CompactString },
    TransitionForeignMachine { state_machine: CompactString, state: CompactString, foreign_machine: CompactString },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableKind {
    Local, Input, Output,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    pub name: CompactString,
    pub role: CompactString,
    pub state_machines: BTreeMap<CompactString, StateMachine>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct StateMachine {
    pub variables: BTreeMap<CompactString, Variable>,
    pub states: BTreeMap<CompactString, State>,
    pub initial_state: Option<CompactString>,
    pub current_state: Option<CompactString>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Variable {
    pub init: CompactString,
    pub kind: VariableKind,
}
#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub parent: Option<CompactString>,
    pub transitions: VecDeque<Transition>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Transition {
    pub ordered_condition: Condition,
    pub unordered_condition: Condition,
    pub actions: VecDeque<CompactString>,
    pub new_state: Option<CompactString>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub omit_unknown_blocks: bool,
}
struct Context {
    variables: Vec<ast::VariableRef>,
    junctions: Vec<(CompactString, State)>,
    settings: Settings,
}

fn prune_unreachable(transitions: &mut VecDeque<Transition>) {
    transitions.retain(|t| t.ordered_condition != Condition::constant(false) && t.unordered_condition != Condition::constant(false));
}

fn translate_value(state_machine: &str, state: &str, value: &ast::Value) -> Result<CompactString, CompileError> {
    Ok(match value {
        ast::Value::String(x) => x.clone(),
        ast::Value::Number(x) => x.to_compact_string(),
        ast::Value::Bool(x) => if *x { "true" } else { "false" }.into(),
        ast::Value::Constant(x) => match x {
            ast::Constant::E => core::f64::consts::E.to_compact_string(),
            ast::Constant::Pi => core::f64::consts::PI.to_compact_string(),
        }
        x => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
    })
}
fn translate_expr(state_machine: &str, state: &str, expr: &ast::Expr, context: &mut Context) -> Result<CompactString, CompileError> {
    fn extract_fixed_variadic(state_machine: &str, state: &str, values: &ast::Expr, context: &mut Context) -> Result<Vec<CompactString>, CompileError> {
        match &values.kind {
            ast::ExprKind::MakeList { values } => Ok(values.iter().map(|x| translate_expr(state_machine, state, x, context)).collect::<Result<_,_>>()?),
            ast::ExprKind::Value(ast::Value::List(values, _)) => Ok(values.iter().map(|x| translate_value(state_machine, state, x)).collect::<Result<_,_>>()?),
            _ => Err(CompileError::VariadicBlocks { state_machine: state_machine.into(), state: state.into() }),
        }
    }

    Ok(match &expr.kind {
        ast::ExprKind::Value(x) => translate_value(state_machine, state, x)?,
        ast::ExprKind::Variable { var } => {
            context.variables.push(var.clone());
            var.trans_name.clone()
        }
        ast::ExprKind::Sin { value } => format_compact!("sind({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Cos { value } => format_compact!("cosd({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Tan { value } => format_compact!("tand({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Asin { value } => format_compact!("asind({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Acos { value } => format_compact!("acosd({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Atan { value } => format_compact!("atand({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Sqrt { value } => format_compact!("sqrt({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Floor { value } => format_compact!("floor({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Ceil { value } => format_compact!("ceil({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Round { value } => format_compact!("round({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Sign { value } => format_compact!("sign({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Neg { value } => format_compact!("-{}", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Abs { value } => format_compact!("abs({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Sub { left, right } => format_compact!("({} - {})", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Div { left, right } => format_compact!("({} / {})", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Mod { left, right } => format_compact!("mod({}, {})", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Log { value, base } => format_compact!("(log({}) / log({}))", translate_expr(state_machine, state, value, context)?, translate_expr(state_machine, state, base, context)?),
        ast::ExprKind::Atan2 { y, x } => format_compact!("atan2d({}, {})", translate_expr(state_machine, state, y, context)?, translate_expr(state_machine, state, x, context)?),
        ast::ExprKind::Add { values } => punctuate(extract_fixed_variadic(state_machine,state, values, context)?.iter().map(|x| x.as_str()), " + ").map(|x| format_compact!("({})", x.0)).unwrap_or_else(|| "0".into()),
        ast::ExprKind::Mul { values } => punctuate(extract_fixed_variadic(state_machine,state, values, context)?.iter().map(|x| x.as_str()), " * ").map(|x| format_compact!("({})", x.0)).unwrap_or_else(|| "1".into()),
        ast::ExprKind::Pow { base, power } => format_compact!("({} ^ {})", translate_expr(state_machine, state, base, context)?, translate_expr(state_machine, state, power, context)?),
        ast::ExprKind::Eq { left, right } => format_compact!("{} == {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Neq { left, right } => format_compact!("{} ~= {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Greater { left, right } => format_compact!("{} > {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::GreaterEq { left, right } => format_compact!("{} >= {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Less { left, right } => format_compact!("{} < {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::LessEq { left, right } => format_compact!("{} <= {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::And { left, right } => format_compact!("{} & {}", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Or { left, right } => format_compact!("({} | {})", translate_expr(state_machine, state, left, context)?, translate_expr(state_machine, state, right, context)?),
        ast::ExprKind::Not { value } => format_compact!("~({})", translate_expr(state_machine, state, value, context)?),
        ast::ExprKind::Timer => "t".into(),
        ast::ExprKind::Random { a, b } => match (translate_expr(state_machine, state, a, context)?.as_str(), translate_expr(state_machine, state, b, context)?.as_str()) {
            ("1", b) => format_compact!("randi({b})"),
            (a, b) => format_compact!("randi([{a}, {b}])"),
        }
        x => match context.settings.omit_unknown_blocks {
            true => "?".into(),
            false => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
        }
    })
}
fn translate_condition(state_machine: &str, state: &str, expr: &ast::Expr, context: &mut Context) -> Result<Condition, CompileError> {
    Ok(match &expr.kind {
        ast::ExprKind::And { left, right } => translate_condition(state_machine, state, left, context)? & translate_condition(state_machine, state, right, context)?,
        ast::ExprKind::Or { left, right } => translate_condition(state_machine, state, left, context)? | translate_condition(state_machine, state, right, context)?,
        ast::ExprKind::Not { value } => !translate_condition(state_machine, state, value, context)?,
        ast::ExprKind::Value(ast::Value::Bool(x)) => Condition::constant(*x),
        ast::ExprKind::Value(ast::Value::String(x)) if x.is_empty() => Condition::constant(true),
        _ => Condition::atom(translate_expr(state_machine, state, expr, context)?),
    })
}
fn parse_actions(state_machine: &str, state: &str, stmt: &ast::Stmt, context: &mut Context) -> Result<Vec<CompactString>, CompileError> {
    Ok(match &stmt.kind {
        ast::StmtKind::Assign { var, value } => {
            context.variables.push(var.clone());
            vec![format_compact!("{} = {}", var.trans_name, translate_expr(state_machine, state, value, context)?)]
        }
        ast::StmtKind::AddAssign { var, value } => {
            context.variables.push(var.clone());
            vec![format_compact!("{} = {} + {}", var.trans_name, var.trans_name, translate_expr(state_machine, state, value, context)?)]
        }
        ast::StmtKind::ResetTimer => vec!["t = 0".into()],
        x => match context.settings.omit_unknown_blocks {
            true => vec!["?".into()],
            false => return Err(CompileError::UnsupportedBlock { state_machine: state_machine.into(), state: state.into(), info: format_compact!("{x:?}") }),
        }
    })
}
fn parse_transitions(state_machine: &str, state: &str, stmt: &ast::Stmt, terminal: bool, context: &mut Context) -> Result<Option<(VecDeque<Transition>, Condition, bool)>, CompileError> {
    fn parse_transition_target(state_machine: &str, state: &str, expr: &ast::Expr, context: &mut Context) -> Result<VecDeque<Transition>, CompileError> {
        Ok(match &expr.kind {
            ast::ExprKind::Value(ast::Value::String(x)) => match x.as_str() {
                "" => return Err(CompileError::TransitionEmptyTarget { state_machine: state_machine.into(), state: state.into() }),
                _ => deque![Transition { ordered_condition: Condition::constant(true), unordered_condition: Condition::constant(true), actions: <_>::default(), new_state: Some(x.clone()) }],
            }
            ast::ExprKind::Conditional { condition, then, otherwise } => {
                let condition = translate_condition(state_machine, state, condition, context)?;
                let mut then_transitions = parse_transition_target(state_machine, state, then, context)?;
                let mut otherwise_transitions = parse_transition_target(state_machine, state, otherwise, context)?;

                for transition in then_transitions.iter_mut() {
                    for target in [&mut transition.unordered_condition, &mut transition.ordered_condition] {
                        *target = condition.clone() & target.clone();
                    }
                }
                for transition in otherwise_transitions.iter_mut() {
                    transition.unordered_condition = !condition.clone() & transition.unordered_condition.clone();
                }

                then_transitions.extend(otherwise_transitions);
                then_transitions
            }
            _ => return Err(CompileError::ComplexTransitionName { state_machine: state_machine.into(), state: state.into() }),
        })
    }

    Ok(match &stmt.kind {
        ast::StmtKind::UnknownBlock { name, args } => match (name.as_str(), args.as_slice()) {
            ("smTransition", [var, value]) => match &var.kind {
                ast::ExprKind::Value(ast::Value::String(var)) => match *var == state_machine {
                    true => Some((parse_transition_target(state_machine, state, value, context)?, Condition::constant(false), true)),
                    false => return Err(CompileError::TransitionForeignMachine { state_machine: state_machine.into(), state: state.into(), foreign_machine: var.clone() }),
                }
                _ => None,
            }
            _ => None,
        }
        ast::StmtKind::Assign { var, value } if var.name == state_machine => match terminal {
            true => Some((parse_transition_target(state_machine, state, value, context)?, Condition::constant(false), true)),
            false => return Err(CompileError::NonTerminalTransition { state_machine: state_machine.into(), state: state.into() }),
        }
        ast::StmtKind::If { condition, then } => {
            let condition = translate_condition(state_machine, state, condition, context)?;
            let (mut transitions, body_terminal, volatile) = parse_stmts(state_machine, state, then, terminal, context, false)?;

            if volatile {
                make_junction(state, &mut <_>::default(), &mut transitions, context);
                debug_assert_eq!(transitions.len(), 1);
            }

            let tail_condition = match body_terminal {
                true => !condition.clone(),
                false => transitions.iter().map(|t| t.ordered_condition.clone()).reduce(|a, b| a | b).map(|c| !(condition.clone() & c)).unwrap_or(Condition::constant(true)),
            };

            for transition in transitions.iter_mut() {
                for target in [&mut transition.unordered_condition, &mut transition.ordered_condition] {
                    *target = condition.clone() & target.clone();
                }
            }

            Some((transitions, tail_condition, false))
        }
        ast::StmtKind::IfElse { condition, then, otherwise } => {
            let condition = translate_condition(state_machine, state, condition, context)?;

            let (mut transitions_1, body_terminal_1, volatile_1) = parse_stmts(state_machine, state, then, terminal, context, false)?;
            let (mut transitions_2, body_terminal_2, volatile_2) = parse_stmts(state_machine, state, otherwise, terminal, context, false)?;

            if volatile_1 {
                make_junction(state, &mut <_>::default(), &mut transitions_1, context);
                debug_assert_eq!(transitions_1.len(), 1);
            }
            if volatile_2 {
                make_junction(state, &mut <_>::default(), &mut transitions_2, context);
                debug_assert_eq!(transitions_2.len(), 1);
            }

            let cond_1 = transitions_1.back().map(|t| t.unordered_condition.clone()).unwrap_or(Condition::constant(false));
            let cond_2 = transitions_2.back().map(|t| t.unordered_condition.clone()).unwrap_or(Condition::constant(false));

            let tail_condition = match (body_terminal_1, body_terminal_2) {
                (true, true) => Condition::constant(false),
                (true, false) => !condition.clone() & !cond_2,
                (false, true) => condition.clone() & !cond_1,
                (false, false) => !(condition.clone() & cond_1) & !(!condition.clone() & cond_2),
            };

            for transition in transitions_1.iter_mut() {
                for target in [&mut transition.unordered_condition, &mut transition.ordered_condition] {
                    *target = condition.clone() & target.clone();
                }
            }
            for transition in transitions_2.iter_mut() {
                let targets = match body_terminal_1 {
                    true => [Some(&mut transition.unordered_condition), None],
                    false => [Some(&mut transition.unordered_condition), Some(&mut transition.ordered_condition)],
                };
                for target in targets.into_iter().flatten() {
                    *target = !condition.clone() & target.clone();
                }
            }

            transitions_1.extend(transitions_2);
            Some((transitions_1, tail_condition, body_terminal_1 && body_terminal_2))
        }
        _ => None,
    })
}
fn make_junction<'a>(state: &str, actions: &mut VecDeque<CompactString>, transitions: &mut VecDeque<Transition>, context: &'a mut Context) {
    prune_unreachable(transitions);

    let junction = format_compact!("::junction-{}::", context.junctions.len());
    let mut junction_state = State { parent: Some(state.into()), transitions: core::mem::take(transitions) };

    if junction_state.transitions.back().map(|t| t.ordered_condition != Condition::constant(true)).unwrap_or(true) {
        let return_condition: Condition = junction_state.transitions.iter().map(|t| t.unordered_condition.clone()).fold(Condition::constant(true), |a, b| a & !b);
        junction_state.transitions.push_back(Transition {
            unordered_condition: return_condition,
            ordered_condition: Condition::constant(true),
            actions: deque![],
            new_state: Some(state.into()),
        });
    }

    transitions.push_front(Transition { ordered_condition: Condition::constant(true), unordered_condition: Condition::constant(true), actions: core::mem::take(actions), new_state: Some(junction.clone()) });
    context.junctions.push((junction, junction_state));
}
fn handle_actions(state_machine: &str, state: &str, actions: &mut VecDeque<CompactString>, transitions: &mut VecDeque<Transition>, terminal: bool, volatile: &mut bool, context: &mut Context) -> Result<(), CompileError> {
    prune_unreachable(transitions);

    if !actions.is_empty() {
        if terminal && transitions.is_empty() {
            transitions.push_front(Transition { ordered_condition: Condition::constant(true), unordered_condition: Condition::constant(true), actions: core::mem::take(actions), new_state: Some(state.into()) });
        } else if transitions.len() == 1 && transitions[0].unordered_condition == Condition::constant(true) {
            transitions[0].actions.extend_front(core::mem::take(actions).into_iter());
        } else if terminal {
            make_junction(state, actions, transitions, context);
            debug_assert_eq!(transitions.len(), 1);
            *volatile = false;
        } else {
            return Err(CompileError::ActionsOutsideTransition { state_machine: state_machine.into(), state: state.into() });
        }
    }

    debug_assert_eq!(actions.len(), 0);
    Ok(())
}
fn parse_stmts(state_machine: &str, state: &str, stmts: &[ast::Stmt], script_terminal: bool, context: &mut Context, top_level: bool) -> Result<(VecDeque<Transition>, bool, bool), CompileError> {
    let mut actions: VecDeque<CompactString> = <_>::default();
    let mut transitions: VecDeque<Transition> = <_>::default();
    let mut body_terminal = false;
    let mut volatile = false;

    if top_level {
        transitions.push_back(Transition { unordered_condition: Condition::constant(true), ordered_condition: Condition::constant(true), actions: <_>::default(), new_state: Some(state.into()) });
    }

    let mut stmts = stmts.iter().rev().peekable();
    while let Some(stmt) = stmts.peek() {
        match &stmt.kind {
            ast::StmtKind::Return { value: _ } => (),
            _ => break,
        }
        stmts.next();
        body_terminal = true;
    }

    let mut last = true;
    for stmt in stmts {
        match parse_transitions(state_machine, state, stmt, (script_terminal || body_terminal) && last, context)? {
            Some((sub_transitions, tail_condition, sub_body_terminal)) => {
                handle_actions(state_machine, state, &mut actions, &mut transitions, script_terminal || body_terminal, &mut volatile, context)?;
                debug_assert_eq!(actions.len(), 0);

                if volatile {
                    make_junction(state, &mut actions, &mut transitions, context);
                    debug_assert_eq!(actions.len(), 0);
                    debug_assert_eq!(transitions.len(), 1);
                    volatile = false;
                }

                body_terminal |= sub_body_terminal;
                for transition in transitions.iter_mut() {
                    transition.unordered_condition = tail_condition.clone() & transition.unordered_condition.clone();
                }
                transitions.extend_front(sub_transitions.into_iter());
            }
            None => match &stmt.kind {
                ast::StmtKind::Sleep { seconds } => {
                    handle_actions(state_machine, state, &mut actions, &mut transitions, script_terminal || body_terminal, &mut volatile, context)?;
                    debug_assert_eq!(actions.len(), 0);

                    match transitions.as_slices() {
                        ([t], []) if t.unordered_condition == Condition::constant(true) => (),
                        _ => {
                            make_junction(state, &mut actions, &mut transitions, context);
                            debug_assert_eq!(actions.len(), 0);
                            debug_assert_eq!(transitions.len(), 1);
                        }
                    };

                    let condition = Condition::atom(format_compact!("after({}, sec)", translate_expr(state_machine, state, seconds, context)?));
                    for transition in transitions.iter_mut() {
                        for target in [&mut transition.unordered_condition, &mut transition.ordered_condition] {
                            *target = target.clone() & condition.clone();
                        }
                    }

                    transitions.push_back(Transition {
                        unordered_condition: !condition,
                        ordered_condition: Condition::constant(true),
                        actions: <_>::default(),
                        new_state: None,
                    });

                    volatile = true;
                }
                _ => actions.extend_front(parse_actions(state_machine, state, stmt, context)?.into_iter()),
            }
        }
        last = false;
    }

    handle_actions(state_machine, state, &mut actions, &mut transitions, script_terminal || body_terminal, &mut volatile, context)?;
    debug_assert_eq!(actions.len(), 0);

    Ok((transitions, body_terminal, volatile))
}

fn dot_id(name: &str) -> dot::Id {
    dot::Id::Escaped(format!("{name:?}"))
}

impl Project {
    pub fn compile(xml: &str, role: Option<&str>, settings: Settings) -> Result<Project, CompileError> {
        let parser = ast::Parser {
            name_transformer: Box::new(ast::util::c_ident),
            ..Default::default()
        };
        let proj = parser.parse(xml).map_err(CompileError::ParseError)?;
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

        let mut state_machines: BTreeMap<CompactString, (StateMachine, Context)> = <_>::default();
        for entity in role.entities.iter() {
            for script in entity.scripts.iter() {
                let (state_machine_name, state_name) = match script.hat.as_ref().map(|x| &x.kind) {
                    Some(ast::HatKind::When { condition }) => match &condition.kind {
                        ast::ExprKind::Eq { left, right } => match (&left.kind, &right.kind) {
                            (ast::ExprKind::Variable { var }, ast::ExprKind::Value(ast::Value::String(val))) => (&var.name, val),
                            (ast::ExprKind::Value(ast::Value::String(val)), ast::ExprKind::Variable { var }) => (&var.name, val),
                            _ => continue,
                        }
                        ast::ExprKind::UnknownBlock { name, args } => match (name.as_str(), args.as_slice()) {
                            ("smInState", [var, val]) => match (&var.kind, &val.kind) {
                                (ast::ExprKind::Value(ast::Value::String(var)), ast::ExprKind::Value(ast::Value::String(val))) => (var, val),
                                _ => continue,
                            }
                            _ => continue,
                        }
                        _ => continue,
                    }
                    _ => continue,
                };

                let (state_machine, context) = state_machines.entry(state_machine_name.clone()).or_insert_with(|| {
                    (StateMachine { variables: <_>::default(), states: <_>::default(), initial_state: None, current_state: None }, Context { variables: vec![], junctions: vec![], settings })
                });
                if state_machine.states.contains_key(state_name.as_str()) {
                    return Err(CompileError::MultipleHandlers { state_machine: state_machine_name.clone(), state: state_name.clone() });
                }

                let (transitions, _, _) = parse_stmts(state_machine_name, state_name, &script.stmts, true, context, true)?;
                assert!(state_machine.states.insert(state_name.clone(), State { parent: None, transitions }).is_none());
            }
        }

        for (state_machine, _) in state_machines.values_mut() {
            for state in state_machine.states.values_mut() {
                prune_unreachable(&mut state.transitions);
                if let Some(last) = state.transitions.back_mut() {
                    last.ordered_condition = Condition::constant(true);
                }
            }
        }

        let mut state_machines = state_machines.into_iter().map(|(state_machine_name, (mut state_machine, context))| {
            for (name, junction) in context.junctions {
                assert!(state_machine.states.insert(name, junction).is_none());
            }
            for variable in context.variables {
                state_machine.variables.insert(variable.trans_name, Variable { init: "0".into(), kind: VariableKind::Local });
            }
            (state_machine_name, state_machine)
        }).collect::<BTreeMap<_,_>>();

        for state_machine in state_machines.values_mut() {
            let target_states: Vec<_> = state_machine.states.values().flat_map(|s| s.transitions.iter().flat_map(|t| t.new_state.clone())).collect();
            for target_state in target_states {
                state_machine.states.entry(target_state.clone()).or_insert_with(|| State {
                    parent: None,
                    transitions: deque![Transition { unordered_condition: Condition::constant(true), ordered_condition: Condition::constant(true), actions: <_>::default(), new_state: Some(target_state) }]
                });
            }
        }

        let mut var_inits: BTreeMap<&CompactString, &ast::Expr> = BTreeMap::new();
        let mut var_kinds: BTreeMap<&CompactString, VariableKind> = BTreeMap::new();
        for entity in role.entities.iter() {
            for script in entity.scripts.iter() {
                if let Some(ast::HatKind::OnFlag) = script.hat.as_ref().map(|x| &x.kind) {
                    for stmt in script.stmts.iter() {
                        match &stmt.kind {
                            ast::StmtKind::Assign { var, value } => match state_machines.get_mut(&var.name) {
                                Some(state_machine) => if let ast::ExprKind::Value(ast::Value::String(value)) = &value.kind {
                                    if state_machine.states.contains_key(value) { state_machine.initial_state = Some(value.clone()); }
                                }
                                None => { var_inits.insert(&var.trans_name, value); }
                            }
                            ast::StmtKind::UnknownBlock { name, args } => match (name.as_str(), args.as_slice()) {
                                ("smTransition", [var, value]) => if let (ast::ExprKind::Value(ast::Value::String(var)), ast::ExprKind::Value(ast::Value::String(value))) = (&var.kind, &value.kind) {
                                    if let  Some(state_machine) = state_machines.get_mut(var) {
                                        if state_machine.states.contains_key(value) { state_machine.initial_state = Some(value.clone()); }
                                    }
                                }
                                ("smMarkVar", [var, kind]) => if let (ast::ExprKind::Value(ast::Value::String(var)), ast::ExprKind::Value(ast::Value::String(kind))) = (&var.kind, &kind.kind) {
                                    let kind = match kind.as_str() {
                                        "local" => VariableKind::Local,
                                        "input" => VariableKind::Input,
                                        "output" => VariableKind::Output,
                                        _ => continue,
                                    };
                                    var_kinds.insert(var, kind);
                                }
                                _ => (),
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        let mut var_inits_context = Context { variables: vec![], junctions: vec![], settings };
        for (state_machine_name, state_machine) in state_machines.iter_mut() {
            if let Some(ast::Value::String(init)) = role.globals.iter().find(|g| g.def.name == state_machine_name).map(|g| &g.init) {
                if state_machine.states.contains_key(init) {
                    state_machine.current_state = Some(init.clone());
                }
            }

            for (var, info) in state_machine.variables.iter_mut() {
                if let Some(&init) = var_inits.get(var) {
                    info.init = translate_expr(state_machine_name, "<init>", init, &mut var_inits_context)?;
                }
                if let Some(&kind) = var_kinds.get(var) {
                    info.kind = kind;
                }
            }
        }
        debug_assert_eq!(var_inits_context.variables.len(), 0);
        debug_assert_eq!(var_inits_context.junctions.len(), 0);
        drop(var_inits_context);

        let mut machines = state_machines.iter();
        while let Some(machine_1) = machines.next() {
            if let Some((machine_2, var)) = machines.clone().find_map(|machine_2| machine_1.1.variables.keys().find(|&k| machine_2.1.variables.contains_key(k)).map(|x| (machine_2, x))) {
                return Err(CompileError::VariableOverlap { state_machines: (machine_1.0.clone(), machine_2.0.clone()), variable: var.clone() });
            }
            if let Some(var) = machine_1.1.variables.keys().find(|&x| state_machines.contains_key(x)) {
                return Err(CompileError::VariableOverlap { state_machines: (machine_1.0.clone(), var.clone()), variable: var.clone() });
            }
        }

        for machine in state_machines.values_mut() {
            for (state_name, state) in machine.states.iter_mut() {
                for transition in state.transitions.iter_mut() {
                    if matches!(&transition.new_state, Some(x) if x == state_name) {
                        transition.new_state = None;
                    }
                }
            }
        }

        Ok(Project { name: proj.name, role: role.name.clone(), state_machines })
    }
    pub fn to_graphviz(&self) -> dot::Graph {
        let stmts = self.state_machines.iter().map(|(name, state_machine)| {
            let node_id = |state: &str| dot::NodeId(if !state.is_empty() { dot_id(&format!("{name} {state}")) } else { dot_id(name) }, None);

            let mut stmts = vec![];
            if let Some(init) = state_machine.initial_state.as_ref() {
                let attributes = vec![
                    dot::Attribute(dot::Id::Plain("shape".into()), dot::Id::Plain("point".into())),
                    dot::Attribute(dot::Id::Plain("width".into()), dot::Id::Plain("0.1".into())),
                ];
                stmts.push(dot::Stmt::Node(dot::Node { id: node_id(""), attributes }));
                stmts.push(dot::Stmt::Edge(dot::Edge { ty: dot::EdgeTy::Pair(dot::Vertex::N(node_id("")), dot::Vertex::N(node_id(init))), attributes: vec![] }));
            }
            for (state_name, state) in state_machine.states.iter() {
                let mut attributes = vec![];

                if state.parent.is_none() {
                    attributes.push(dot::Attribute(dot::Id::Plain("label".into()), dot_id(state_name)));
                } else {
                    attributes.push(dot::Attribute(dot::Id::Plain("label".into()), dot_id("")));
                    attributes.push(dot::Attribute(dot::Id::Plain("shape".into()), dot::Id::Plain("circle".into())));
                    attributes.push(dot::Attribute(dot::Id::Plain("width".into()), dot::Id::Plain("0.1".into())));
                }

                if state_machine.current_state.as_ref().map(|x| x == state_name).unwrap_or(false) {
                    attributes.push(dot::Attribute(dot::Id::Plain("style".into()), dot::Id::Plain("filled".into())));
                }

                stmts.push(dot::Stmt::Node(dot::Node { id: node_id(state_name), attributes }));
            }
            for (state_name, state) in state_machine.states.iter() {
                let included_transitions = state.transitions.iter().filter(|t| t.new_state.as_ref().unwrap_or(state_name) != state_name || !t.actions.is_empty() || t.ordered_condition != Condition::constant(true)).collect::<Vec<_>>();

                let labeler: fn (usize, Option<String>) -> dot::Id = match included_transitions.len() {
                    1 => |_, t| t.map(|t| dot_id(&format!(" {t} "))).unwrap_or_else(|| dot_id("")),
                    _ => |i, t| t.map(|t| dot_id(&format!(" {}: {t} ", i + 1))).unwrap_or_else(|| dot_id(&format!(" {} ", i + 1))),
                };
                for (i, transition) in included_transitions.iter().enumerate() {
                    stmts.push(dot::Stmt::Edge(dot::Edge { ty: dot::EdgeTy::Pair(dot::Vertex::N(node_id(state_name)), dot::Vertex::N(node_id(transition.new_state.as_ref().unwrap_or(state_name)))), attributes: vec![
                        dot::Attribute(dot::Id::Plain("label".into()), labeler(i, if transition.ordered_condition != Condition::constant(true) { Some(transition.ordered_condition.to_string()) } else { None })),
                    ] }));
                }
            }
            dot::Stmt::Subgraph(dot::Subgraph { id: dot_id(name), stmts })
        }).collect();
        dot::Graph::DiGraph { id: dot_id(&self.name), strict: false, stmts }
    }
    pub fn to_stateflow(&self) -> Result<CompactString, CompileError> {
        let mut rename_pool = RenamePool::new(ast::util::c_ident);
        let mut rename = move |x| rename_pool.rename(x);
        let model_name = rename(&self.name)?;

        let state_size = (100, 100);
        let junction_size = (100, 20);
        let padding = (100, 100);

        fn stateflow_escape(full: &str) -> String {
            let mut res = String::new();
            for line in full.lines() {
                if !res.is_empty() {
                    res.push_str(" + newline + ");
                }
                write!(res, "{line:?}").unwrap();
            }
            res
        }

        let mut res = CompactString::default();
        writeln!(res, "sfnew {model_name}").unwrap();
        for (state_machine_idx, (state_machine_name, state_machine)) in self.state_machines.iter().enumerate() {
            let state_numbers: BTreeMap<&str, usize> = state_machine.states.iter().enumerate().map(|x| (x.1.0.as_str(), x.0)).collect();
            let parent_state_numbers: BTreeMap<&str, usize> = state_machine.states.iter().filter(|x| x.1.parent.is_none()).enumerate().map(|x| (x.1.0.as_str(), x.0)).collect();

            if state_machine_idx == 0 {
                writeln!(res, "chart = find(sfroot, \"-isa\", \"Stateflow.Chart\")").unwrap();
                writeln!(res, "chart.Name = {state_machine_name:?}").unwrap();
            } else {
                writeln!(res, "chart = add_block(\"sflib/Chart\", {:?})", format!("{model_name}/{state_machine_name}")).unwrap();
            }

            let included_transitions = state_machine.states.iter().map(|(state_name, state)| {
                (state as *const State, state.transitions.iter().filter(|t| t.new_state.as_ref().unwrap_or(state_name) != state_name || !t.actions.is_empty() || t.ordered_condition != Condition::constant(true)).collect::<Vec<_>>())
            }).collect::<BTreeMap<_,_>>();

            let entry_actions = state_machine.states.iter().filter(|s| s.1.parent.is_none()).map(|(state_name, _)| {
                let actions = match state_machine.initial_state.as_ref().map(|i| i != state_name).unwrap_or(true) {
                    true => common_suffix(state_machine.states.iter().flat_map(|(n, s)| included_transitions[&(s as _)].iter().filter(|t| t.new_state.as_ref().unwrap_or(n) == state_name)).map(|t| t.actions.iter())),
                    false => <_>::default(),
                };
                (state_name, actions)
            }).collect::<BTreeMap<_,_>>();
            let exit_actions = state_machine.states.iter().filter(|s| s.1.parent.is_none()).map(|(state_name, state)| {
                let actions = common_suffix(included_transitions[&(state as _)].iter().map(|t| t.actions.iter().take(t.actions.len() - entry_actions.get(t.new_state.as_ref().unwrap_or(state_name)).map(|x| x.len()).unwrap_or(0))));
                (state_name, actions)
            }).collect::<BTreeMap<_,_>>();

            let mut child_counts: BTreeMap<&str, usize> = Default::default();
            for (state_idx, (state_name, state)) in state_machine.states.iter().enumerate() {
                match state.parent.as_deref() {
                    Some(parent) => {
                        *child_counts.entry(parent).or_default() += 1;
                        writeln!(res, "s{state_idx} = Stateflow.State(chart)").unwrap();
                        writeln!(res, "s{state_idx}.LabelString = \"{}_{}\"", rename(parent)?, child_counts[parent]).unwrap();
                        writeln!(res, "s{state_idx}.Position = [{}, {}, {}, {}]", parent_state_numbers[parent] * (state_size.0 + padding.0) + (state_size.0 - junction_size.0) / 2, state_size.1 + padding.1 * child_counts[parent], junction_size.0, junction_size.1).unwrap();
                    }
                    None => {
                        let mut label = rename(state_name)?;
                        match entry_actions.get(state_name) {
                            Some(actions) if !actions.is_empty() => {
                                label.push_str("\nentry:");
                                for action in actions {
                                    write!(label, " {action};").unwrap();
                                }
                            }
                            _ => (),
                        }
                        match exit_actions.get(state_name) {
                            Some(actions) if !actions.is_empty() => {
                                label.push_str("\nexit:");
                                for action in actions {
                                    write!(label, " {action};").unwrap();
                                }
                            }
                            _ => (),
                        }

                        writeln!(res, "s{state_idx} = Stateflow.State(chart)").unwrap();
                        writeln!(res, "s{state_idx}.LabelString = {}", stateflow_escape(&label)).unwrap();
                        writeln!(res, "s{state_idx}.Position = [{}, {}, {}, {}]", parent_state_numbers[state_name.as_str()] * (state_size.0 + padding.0), 0, state_size.0, state_size.1).unwrap();
                    }
                }
            }
            for (state_idx, (state_name, state)) in state_machine.states.iter().enumerate() {
                for transition in &included_transitions[&(state as _)] {
                    writeln!(res, "t = Stateflow.Transition(chart)").unwrap();
                    writeln!(res, "t.Source = s{state_idx}").unwrap();
                    writeln!(res, "t.Destination = s{}", state_numbers[transition.new_state.as_deref().unwrap_or(state_name)]).unwrap();

                    let mut label = CompactString::default();
                    if transition.unordered_condition != Condition::constant(true) {
                        write!(label, "[{}]", transition.unordered_condition).unwrap();
                    }

                    let entry_action_count = entry_actions.get(transition.new_state.as_ref().unwrap_or(state_name)).map(|x| x.len()).unwrap_or(0);
                    let exit_action_count = exit_actions.get(state_name).map(|x| x.len()).unwrap_or(0);
                    if transition.actions.len() > entry_action_count + exit_action_count {
                        label.push('{');
                        for action in transition.actions.iter().take(transition.actions.len() - (entry_action_count + exit_action_count)) {
                            write!(label, "{action};").unwrap();
                        }
                        label.push('}');
                    }

                    writeln!(res, "t.LabelString = {label:?}").unwrap();
                }
            }
            if let Some(initial_state) = state_machine.initial_state.as_deref() {
                writeln!(res, "t = Stateflow.Transition(chart)").unwrap();
                writeln!(res, "t.Destination = s{}", state_numbers[initial_state]).unwrap();
                writeln!(res, "t.DestinationOClock = 0").unwrap();
                writeln!(res, "t.SourceEndpoint = t.DestinationEndpoint - [0 30]").unwrap();
                writeln!(res, "t.Midpoint = t.DestinationEndpoint - [0 15]").unwrap();
            }
            for (var, info) in state_machine.variables.iter() {
                writeln!(res, "d = Stateflow.Data(chart)").unwrap();
                writeln!(res, "d.Name = {var:?}").unwrap();
                writeln!(res, "d.Props.InitialValue = {:?}", info.init).unwrap();
                writeln!(res, "d.Scope = \"{:?}\"", info.kind).unwrap();
            }
        }
        debug_assert_eq!(res.chars().next_back(), Some('\n'));
        res.pop();
        Ok(res)
    }
}
