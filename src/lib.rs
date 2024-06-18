use netsblox_ast as ast;
use netsblox_ast::compact_str::CompactString;

use std::collections::BTreeMap;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum CompileError {
    ParseError(Box<ast::Error>),
    UnknownRole { name: CompactString },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    pub name: CompactString,
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
            _ =>
        }
    }

    let mut state_machines = Default::default();

    Ok(Project { name: proj.name, state_machines })
}
