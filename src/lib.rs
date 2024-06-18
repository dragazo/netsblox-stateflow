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
            let (state_machine, state) = match script.hat.as_ref().map(|x| &x.kind) {
                Some(ast::HatKind::When { condition }) => match &condition.kind {
                    ast::ExprKind::Eq { left, right } => match (&left.kind, &right.kind) {
                        (ast::ExprKind::Variable { var }, ast::ExprKind::Value(ast::Value::String(val))) => (var.name.clone(), val.clone()),
                        (ast::ExprKind::Value(ast::Value::String(val)), ast::ExprKind::Variable { var }) => (var.name.clone(), val.clone()),
                        _ => continue,
                    }
                    _ => continue,
                }
                _ => continue,
            };
            let state_machine = state_machines.entry(state_machine).or_insert_with(|| StateMachine { states: Default::default(), initial_state: None });
            state_machine.states.entry(state).or_insert_with(|| State { });
        }
    }

    Ok(Project { name: proj.name, role: role.name.clone(), state_machines })
}
