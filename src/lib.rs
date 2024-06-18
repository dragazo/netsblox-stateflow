use netsblox_ast as ast;
use netsblox_ast::compact_str::CompactString;

use std::collections::BTreeMap;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum CompileError {
    ParseError(Box<ast::Error>),
    RoleCount { count: usize },
    UnknownRole { name: CompactString },
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
    pub transitions: Vec<Transition>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Transition {
    pub condition: Option<CompactString>,
    pub new_state: CompactString,
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

            let state_machine = state_machines.entry(state_machine_name.clone()).or_insert_with(|| StateMachine { states: Default::default(), initial_state: None });
            let state = state_machine.states.entry(state_name.clone()).or_insert_with(|| State { transitions: Default::default() });

            let mut target_states = vec![];
            for stmt in script.stmts.iter() {
                match &stmt.kind {
                    ast::StmtKind::Assign { var, value } if var.name == state_machine_name => match &value.kind {
                        ast::ExprKind::Value(ast::Value::String(x)) => {
                            state.transitions.push(Transition { condition: None, new_state: x.clone() });
                            target_states.push(x.clone());
                        }
                        _ => (),
                    }
                    _ => (),
                }
            }
            for target_state in target_states {
                state_machine.states.entry(target_state).or_insert_with(|| State { transitions: Default::default() });
            }
        }
    }

    Ok(Project { name: proj.name, role: role.name.clone(), state_machines })
}
