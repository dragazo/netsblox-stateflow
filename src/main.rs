use clap::{Parser, ValueEnum};

use netsblox_stateflow::*;

#[derive(ValueEnum, Clone)]
enum Mode {
    Raw, Graphviz, Stateflow,
}

#[derive(Parser)]
struct Args {
    input: String,

    #[clap(short, long)]
    mode: Mode,
}

fn main() {
    let Args { input, mode } = Args::parse();

    let content = std::fs::read_to_string(&input).unwrap();
    let project = Project::compile(&content, None, Settings { omit_unknown_blocks: true }).unwrap();

    match mode {
        Mode::Raw => println!("{project:?}"),
        Mode::Graphviz => println!("{}", graphviz::print(project.to_graphviz(), &mut Default::default())),
        Mode::Stateflow => println!("{}", project.to_stateflow().unwrap()),
    }
}
