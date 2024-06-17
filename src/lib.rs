use netsblox_ast as ast;
use netsblox_ast::compact_str::CompactString;

#[derive(Debug)]
pub enum CompileError {
    ParseError(Box<ast::Error>),
}

pub struct Project {
    pub name: CompactString,
    pub state_machines: Vec<StateMachine>,
}
pub struct StateMachine {
    pub name: CompactString
    pub states: Vec<State>,
}
pub struct State {
    pub name: CompactString,
}

pub fn compile(xml: &str) -> Result<Project, CompileError> {
    let proj = ast::Parser::default().parse(xml).map_err(|e| CompileError::ParseError(e))?;
    proj.name
}
