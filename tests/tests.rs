use std::collections::{BTreeSet, BTreeMap};

use itertools::Itertools;

use netsblox_stateflow::*;

fn aggregate_atoms<'a>(src: &'a RawCondition, dest: &mut BTreeSet<&'a str>) {
    match src {
        RawCondition::Const(_) => (),
        RawCondition::Atom(x) => { dest.insert(x.as_str()); }
        RawCondition::And(a, b) | RawCondition::Or(a, b) => {
            aggregate_atoms(a, dest);
            aggregate_atoms(b, dest);
        }
        RawCondition::Not(a) => aggregate_atoms(a, dest),
    }
}

fn eval(cond: &RawCondition, assignments: &BTreeMap<&str, bool>) -> bool {
    match cond {
        RawCondition::Const(x) => *x,
        RawCondition::Atom(x) => assignments[x.as_str()],
        RawCondition::And(a, b) => eval(a, assignments) && eval(b, assignments),
        RawCondition::Or(a, b) => eval(a, assignments) || eval(b, assignments),
        RawCondition::Not(a) => !eval(a, assignments),
    }
}

fn assert_complete(proj: &Project) {
    for (state_machine_name, state_machine) in proj.state_machines.iter() {
        for (state_name, state) in state_machine.states.iter() {
            match state.transitions.back() {
                Some(t) => if t.ordered_condition != Condition::constant(true) { panic!("{state_machine_name:?} :: {state_name:?} > transitions not in normal form") },
                None => panic!("{state_machine_name:?} :: {state_name:?} > no transitions"),
            }

            let mut variables = BTreeSet::new();
            for transition in state.transitions.iter() {
                aggregate_atoms(transition.ordered_condition.raw(), &mut variables);
                aggregate_atoms(transition.unordered_condition.raw(), &mut variables);
            }

            let mut chosen_transitions = BTreeMap::new();
            for values in vec![[false, true]; variables.len()].into_iter().multi_cartesian_product() {
                assert_eq!(variables.len(), values.len());
                let assignments = variables.iter().copied().zip(values.iter().copied()).collect::<BTreeMap<_,_>>();

                match state.transitions.iter().enumerate().find(|x| eval(x.1.ordered_condition.raw(), &assignments)).map(|x| x.0) {
                    Some(i) => { chosen_transitions.insert(assignments, i); }
                    None => panic!("{state_machine_name:?} :: {state_name:?} > no ordered transition for {assignments:?}"),
                }
            }
            assert_eq!(chosen_transitions.len(), 1 << variables.len());

            for (assignments, chosen_transition) in chosen_transitions.iter() {
                let activations = state.transitions.iter().map(|t| eval(t.unordered_condition.raw(), assignments)).collect::<Vec<_>>();
                match activations.iter().filter(|x| **x).count() {
                    0 => panic!("{state_machine_name:?} :: {state_name:?} > no unordered transition for {assignments:?}"),
                    1 => match activations.iter().enumerate().find(|x| *x.1).unwrap().0 {
                        x if x == *chosen_transition => (),
                        x => panic!("{state_machine_name:?} :: {state_name:?} > wrong unordered transition (got {x} expected {chosen_transition}) for {assignments:?}"),
                    }
                    _ => panic!("{state_machine_name:?} :: {state_name:?} > multiple unordered transitions (got {x:?} expected {chosen_transition}) for {assignments:?}", x = activations.iter().enumerate().filter_map(|t| t.1.then(|| t.0)).collect::<Vec<_>>()),
                }
            }
        }
    }
}

#[test]
fn test_empty_project() {
    let proj = Project::compile(include_str!("projects/empty-project.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [].into_iter().collect(),
    });
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {

}
    "#.trim());
    assert_complete(&proj);
}

#[test]
fn test_simple() {
    let proj = Project::compile(include_str!("projects/simple.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 1" -> "something thing 2" [label=""]
    "something thing 2" -> "something thing 1" [label=""]
  }
}
    "#.trim());
    assert_eq!(proj.to_stateflow().unwrap(), r#"
sfnew untitled
chart = find(sfroot, "-isa", "Stateflow.Chart", Path = "untitled/Chart")
chart.Name = "something"
s0 = Stateflow.State(chart)
s0.Name = "thing_1"
s0.Position = [0, 0, 100, 100]
s1 = Stateflow.State(chart)
s1.Name = "thing_2"
s1.Position = [200, 0, 100, 100]
t = Stateflow.Transition(chart)
t.Source = s0
t.Destination = s1
t.LabelString = "[]{}"
t = Stateflow.Transition(chart)
t.Source = s1
t.Destination = s0
t.LabelString = "[]{}"
    "#.trim());
}

#[test]
fn test_simple_no_handler() {
    let proj = Project::compile(include_str!("projects/simple-no-handler.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 3"[label="thing 3"]
    "something thing 1" -> "something thing 2" [label=""]
    "something thing 2" -> "something thing 3" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_single_transition() {
    let proj = Project::compile(include_str!("projects/single-transition.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            }
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 1" -> "something thing 2" [label=" t > 10 "]
    "something thing 2" -> "something thing 1" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_multiple_machines_1() {
    let proj = Project::compile(include_str!("projects/multiple-machines-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("machine 1".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "bar".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("bar".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "buz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("buz".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("foo".into()),
                current_state: None,
            }),
            ("machine 2".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("bar".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "baz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("baz".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "buzz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("buzz".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "bar".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("bar".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "machine 1" {
    "machine 1"[shape=point,width=0.1]
    "machine 1" -> "machine 1 foo"
    "machine 1 bar"[label="bar"]
    "machine 1 buz"[label="buz"]
    "machine 1 foo"[label="foo"]
    "machine 1 bar" -> "machine 1 buz" [label=""]
    "machine 1 buz" -> "machine 1 foo" [label=""]
    "machine 1 foo" -> "machine 1 bar" [label=""]
  }
  subgraph "machine 2" {
    "machine 2"[shape=point,width=0.1]
    "machine 2" -> "machine 2 bar"
    "machine 2 bar"[label="bar"]
    "machine 2 baz"[label="baz"]
    "machine 2 buzz"[label="buzz"]
    "machine 2 bar" -> "machine 2 baz" [label=""]
    "machine 2 baz" -> "machine 2 buzz" [label=""]
    "machine 2 buzz" -> "machine 2 bar" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_simple_if_timer() {
    let proj = Project::compile(include_str!("projects/simple-if-timer.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_timer_reset_1() {
    let proj = Project::compile(include_str!("projects/if-timer-reset-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_timer_reset_2() {
    let proj = Project::compile(include_str!("projects/if-timer-reset-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_timer_reset_3() {
    let proj = Project::compile(include_str!("projects/if-timer-reset-3.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_no_transitions_1() {
    let proj = Project::compile(include_str!("projects/no-transitions-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_no_transitions_2() {
    let proj = Project::compile(include_str!("projects/no-transitions-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_chain_1() {
    let proj = Project::compile(include_str!("projects/if-chain-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 9".into()),
                                ordered_condition: Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 9".into()) & Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 9".into()) & !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_chain_2() {
    let proj = Project::compile(include_str!("projects/if-chain-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 8".into()) & Condition::atom("t > 9".into()),
                                ordered_condition: Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 8".into()) & !Condition::atom("t > 9".into()) & Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 8".into()) & !Condition::atom("t > 9".into()) & !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            }
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 3"[label="thing 3"]
    "something thing 4"[label="thing 4"]
    "something thing 1" -> "something thing 4" [label=" 1: t > 8 "]
    "something thing 1" -> "something thing 3" [label=" 2: t > 9 "]
    "something thing 1" -> "something thing 2" [label=" 3: t > 10 "]
    "something thing 2" -> "something thing 1" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_nested_if_1() {
    let proj = Project::compile(include_str!("projects/nested-if-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_nested_if_2() {
    let proj = Project::compile(include_str!("projects/nested-if-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !(Condition::atom("t > 10".into()) & Condition::atom("t > 9".into())),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_nested_if_3() {
    let proj = Project::compile(include_str!("projects/nested-if-3.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: !(Condition::atom("t > 10".into()) & (Condition::atom("t > 9".into()) | Condition::atom("t > 8".into()))),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_nested_if_4() {
    let proj = Project::compile(include_str!("projects/nested-if-4.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & !Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !(Condition::atom("t > 10".into()) & ((Condition::atom("t > 9".into()) & Condition::atom("t > 7".into())) | Condition::atom("t > 9".into()) | Condition::atom("t > 8".into()))),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [Transition {
                            ordered_condition: Condition::constant(true),
                            unordered_condition: Condition::constant(true),
                            actions: [].into_iter().collect(),
                            new_state: "thing 0".into(),
                        },].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_nested_if_5() {
    let proj = Project::compile(include_str!("projects/nested-if-5.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & !Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & !Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_nested_if_6() {
    let proj = Project::compile(include_str!("projects/nested-if-6.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                actions: [
                                    "foo = 234".into(),
                                    "foo = 652".into(),
                                ].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & !Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                actions: [
                                    "foo = 123".into(),
                                    "foo = 453".into(),
                                ].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 8".into()),
                                actions: [
                                    "foo = 546".into(),
                                    "foo = 876".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & !Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [
                                    "foo = 431".into(),
                                    "foo = 197".into(),
                                ].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 856".into(),
                                    "foo = 465".into(),
                                ].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_variables_1() {
    let proj = Project::compile(include_str!("projects/variables-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 14".into(),
                                    "foo = 21".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 76".into(),
                                    "foo = 43".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_variables_2() {
    let proj = Project::compile(include_str!("projects/variables-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 14".into(),
                                    "foo = 21".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 76".into(),
                                    "foo = 43".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
            ("another".into(), StateMachine {
                variables: [
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("test 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "bar = 231".into(),
                                    "bar = 453".into(),
                                ].into_iter().collect(),
                                new_state: "test 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("test 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "bar = 432".into(),
                                    "bar = 646".into(),
                                ].into_iter().collect(),
                                new_state: "test 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_variables_3() {
    let err = Project::compile(include_str!("projects/variables-3.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::VariableOverlap { state_machines: ("another".into(), "something".into()), variable: "foo".into() });
}

#[test]
fn test_variables_4() {
    let err = Project::compile(include_str!("projects/variables-4.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::VariableOverlap { state_machines: ("something".into(), "another".into()), variable: "another".into() });
}

#[test]
fn test_var_inits() {
    let proj = Project::compile(include_str!("projects/var-inits.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "something cool".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy g".into(), StateMachine {
                variables: [
                    ("foo_3".into(), "(7 + 2)".into()),
                    ("bar_5".into(), "(4 * 4)".into()),
                    ("baz_b".into(), "(3 ^ 2)".into()),
                ].into_iter().collect(),
                states: [
                    ("merp derp".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo_3 = (foo_3 * 2)".into(),
                                    "bar_5 = bar_5 + (1 + 1)".into(),
                                    "baz_b = (bar_5 + foo_3)".into(),
                                ].into_iter().collect(),
                                new_state: "derp merp".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("derp merp".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo_3 = (foo_3 * 0.1)".into(),
                                    "bar_5 = bar_5 + (1 + -4)".into(),
                                    "baz_b = (bar_5 - foo_3)".into(),
                                ].into_iter().collect(),
                                new_state: "merp derp".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("merp derp".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(proj.to_stateflow().unwrap(), r#"
sfnew something_cool
chart = find(sfroot, "-isa", "Stateflow.Chart", Path = "something_cool/Chart")
chart.Name = "thingy g"
s0 = Stateflow.State(chart)
s0.Name = "derp_merp"
s0.Position = [0, 0, 100, 100]
s1 = Stateflow.State(chart)
s1.Name = "merp_derp"
s1.Position = [200, 0, 100, 100]
t = Stateflow.Transition(chart)
t.Source = s0
t.Destination = s1
t.LabelString = "[]{foo_3 = (foo_3 * 0.1);bar_5 = bar_5 + (1 + -4);baz_b = (bar_5 - foo_3);}"
t = Stateflow.Transition(chart)
t.Source = s1
t.Destination = s0
t.LabelString = "[]{foo_3 = (foo_3 * 2);bar_5 = bar_5 + (1 + 1);baz_b = (bar_5 + foo_3);}"
t = Stateflow.Transition(chart)
t.Destination = s1
t.DestinationOClock = 0
t.SourceEndpoint = t.DestinationEndpoint - [0 30]
t.Midpoint = t.DestinationEndpoint - [0 15]
d = Stateflow.Data(chart)
d.Name = "bar_5"
d.Props.InitialValue = "(4 * 4)"
d = Stateflow.Data(chart)
d.Name = "baz_b"
d.Props.InitialValue = "(3 ^ 2)"
d = Stateflow.Data(chart)
d.Name = "foo_3"
d.Props.InitialValue = "(7 + 2)"
    "#.trim());
}

#[test]
fn test_if_else_1() {
    let proj = Project::compile(include_str!("projects/if-else-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_2() {
    let err = Project::compile(include_str!("projects/if-else-2.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_3() {
    let err = Project::compile(include_str!("projects/if-else-3.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_4() {
    let err = Project::compile(include_str!("projects/if-else-4.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_5() {
    let proj = Project::compile(include_str!("projects/if-else-5.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_6() {
    let proj = Project::compile(include_str!("projects/if-else-6.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_7() {
    let proj = Project::compile(include_str!("projects/if-else-7.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: !Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_8() {
    let proj = Project::compile(include_str!("projects/if-else-8.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_9() {
    let err = Project::compile(include_str!("projects/if-else-9.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_10() {
    let err = Project::compile(include_str!("projects/if-else-10.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_11() {
    let err = Project::compile(include_str!("projects/if-else-11.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_12() {
    let err = Project::compile(include_str!("projects/if-else-12.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_13() {
    let proj = Project::compile(include_str!("projects/if-else-13.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 6".into()),
                                ordered_condition: Condition::atom("a == 6".into()),
                                actions: [].into_iter().collect(),
                                new_state: "second".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 6".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "first".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("second".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "second".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_14() {
    let proj = Project::compile(include_str!("projects/if-else-14.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: !Condition::atom("a == 6".into()),
                                ordered_condition: !Condition::atom("a == 6".into()),
                                actions: [].into_iter().collect(),
                                new_state: "second".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("a == 6".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "first".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("second".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "second".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_15() {
    let proj = Project::compile(include_str!("projects/if-else-15.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: !Condition::atom("a == 6".into()),
                                ordered_condition: !Condition::atom("a == 6".into()),
                                actions: [].into_iter().collect(),
                                new_state: "fourth".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("a == 6".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "first".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("fourth".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "fourth".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_16() {
    let proj = Project::compile(include_str!("projects/if-else-16.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: !Condition::atom("a == 6".into()),
                                ordered_condition: !Condition::atom("a == 6".into()),
                                actions: [].into_iter().collect(),
                                new_state: "fourth".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("a == 6".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "first".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("fourth".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "fourth".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_17() {
    let proj = Project::compile(include_str!("projects/if-else-17.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "t = 0".into(),
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("thing 1".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_18() {
    let proj = Project::compile(include_str!("projects/if-else-18.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 1".into(),
                                    "foo = 2".into(),
                                    "foo = 3".into(),
                                ].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("thing 1".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_else_19() {
    let proj = Project::compile(include_str!("projects/if-else-19.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 1".into(),
                                    "foo = 2".into(),
                                    "foo = 3".into(),
                                ].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("thing 1".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_if_fall_through_1() {
    let err = Project::compile(include_str!("projects/if-fall-through-1.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_fall_through_2() {
    let err = Project::compile(include_str!("projects/if-fall-through-2.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_simple_fall_through_1() {
    let err = Project::compile(include_str!("projects/simple-fall-through-1.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_simple_fall_through_2() {
    let err = Project::compile(include_str!("projects/simple-fall-through-2.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_multiple_handlers() {
    let err = Project::compile(include_str!("projects/multiple-handlers.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::MultipleHandlers { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_transition_1() {
    let err = Project::compile(include_str!("projects/complex-transition-1.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ComplexTransitionName { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_transition_2() {
    let err = Project::compile(include_str!("projects/complex-transition-2.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ComplexTransitionName { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_tail_actions_1() {
    let proj = Project::compile(include_str!("projects/tail-actions-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo > bar".into()),
                                ordered_condition: Condition::atom("foo > bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo > bar".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = (2 * foo * 2)".into(),
                                    "bar = (3 * bar)".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_operators() {
    let proj = Project::compile(include_str!("projects/operators.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo < bar".into()),
                                ordered_condition: Condition::atom("foo < bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & Condition::atom("foo <= bar".into()),
                                ordered_condition: Condition::atom("foo <= bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & Condition::atom("foo > bar".into()),
                                ordered_condition: Condition::atom("foo > bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & Condition::atom("foo >= bar".into()),
                                ordered_condition: Condition::atom("foo >= bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & !Condition::atom("foo >= bar".into()) & Condition::atom("foo == bar".into()),
                                ordered_condition: Condition::atom("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 6".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & !Condition::atom("foo >= bar".into()) & !Condition::atom("foo == bar".into()) & Condition::atom("foo ~= bar".into()),
                                ordered_condition: Condition::atom("foo ~= bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 7".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & !Condition::atom("foo >= bar".into()) & !Condition::atom("foo == bar".into()) & !Condition::atom("foo ~= bar".into()) & (Condition::atom("foo < 4".into()) & Condition::constant(true)),
                                ordered_condition: Condition::atom("foo < 4".into()) & Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 8".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & !Condition::atom("foo >= bar".into()) & !Condition::atom("foo == bar".into()) & !Condition::atom("foo ~= bar".into()) & !(Condition::atom("foo < 4".into()) & Condition::constant(true)) & (Condition::constant(false) | Condition::atom("foo > 4".into())),
                                ordered_condition: Condition::constant(false) | Condition::atom("foo > 4".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 9".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & !Condition::atom("foo >= bar".into()) & !Condition::atom("foo == bar".into()) & !Condition::atom("foo ~= bar".into()) & !(Condition::atom("foo < 4".into()) & Condition::constant(true)) & !(Condition::constant(false) | Condition::atom("foo > 4".into())) & !Condition::atom("foo == 4".into()),
                                ordered_condition: !Condition::atom("foo == 4".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 10".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < bar".into()) & !Condition::atom("foo <= bar".into()) & !Condition::atom("foo > bar".into()) & !Condition::atom("foo >= bar".into()) & !Condition::atom("foo == bar".into()) & !Condition::atom("foo ~= bar".into()) & !(Condition::atom("foo < 4".into()) & Condition::constant(true)) & !(Condition::constant(false) | Condition::atom("foo > 4".into())) & !!Condition::atom("foo == 4".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = (foo + bar)".into(),
                                    "foo = (foo + 7 + bar)".into(),
                                    "foo = (foo - bar)".into(),
                                    "foo = (foo * bar)".into(),
                                    "foo = (foo * 5 * bar)".into(),
                                    "foo = (bar / foo)".into(),
                                    "foo = (6 ^ foo)".into(),
                                    "foo = mod(bar, foo)".into(),
                                    "foo = round(3.14159)".into(),
                                    "foo = atan2d(foo, bar)".into(),
                                    "foo = abs(3.1415)".into(),
                                    "foo = -3.1415".into(),
                                    "foo = sign(3.1415)".into(),
                                    "foo = ceil(3.1415)".into(),
                                    "foo = floor(3.1415)".into(),
                                    "foo = sqrt(3.1415)".into(),
                                    "foo = sind(3.1415)".into(),
                                    "foo = cosd(3.1415)".into(),
                                    "foo = tand(3.1415)".into(),
                                    "foo = asind(3.1415)".into(),
                                    "foo = acosd(3.1415)".into(),
                                    "foo = atand(3.1415)".into(),
                                    "foo = (log(3.1415) / log(2.718281828459045))".into(),
                                    "foo = (log(3.1415) / log(10.0))".into(),
                                    "foo = (log(3.1415) / log(2.0))".into(),
                                    "foo = (2.718281828459045 ^ 3.1415)".into(),
                                    "foo = (10.0 ^ 3.1415)".into(),
                                    "foo = (2.0 ^ 3.1415)".into(),
                                    "foo = 3.1415".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 6".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 6".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 7".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 7".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 8".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 8".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 9".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 9".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 10".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 10".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_actions_1() {
    let proj = Project::compile(include_str!("projects/actions-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("state".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("state 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = 456".into()
                                ].into_iter().collect(),
                                new_state: "state 1".into(),
                            }
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_actions_2() {
    let proj = Project::compile(include_str!("projects/actions-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("state".into(), StateMachine {
                variables: [
                    ("foo".into(), "0".into()),
                    ("bar".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("state 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = (foo + bar)".into()
                                ].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            }
                        ].into_iter().collect(),
                    }),
                    ("state 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "state 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("state 1".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo < 7".into()),
                                ordered_condition: Condition::atom("foo < 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "state 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo < 7".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "state 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "state" {
    "state ::junction-0::"[label="",shape=circle,width=0.1]
    "state state 1"[label="state 1"]
    "state state 2"[label="state 2"]
    "state ::junction-0::" -> "state state 2" [label=" 1: foo < 7 "]
    "state ::junction-0::" -> "state state 1" [label=" 2 "]
    "state state 1" -> "state ::junction-0::" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_ext_blocks_1() {
    let proj = Project::compile(include_str!("projects/ext-blocks-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & Condition::atom("t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()) & !Condition::atom("t > 7".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()) & Condition::atom("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t > 10".into()) & !Condition::atom("t > 9".into()) & !Condition::atom("t > 8".into()),
                                ordered_condition: Condition::atom("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_ite_1() {
    let proj = Project::compile(include_str!("projects/ite-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thing".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t < 10".into()),
                                ordered_condition: Condition::atom("t < 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "bar".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t < 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "baz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("bar".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "bar".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("baz".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "baz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_ite_2() {
    let proj = Project::compile(include_str!("projects/ite-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thing".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("t < 10".into()) & Condition::atom("t < 9".into()),
                                ordered_condition: Condition::atom("t < 10".into()) & Condition::atom("t < 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "bar1".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("t < 10".into()) & !Condition::atom("t < 9".into()),
                                ordered_condition: Condition::atom("t < 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "bar2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t < 10".into()) & Condition::atom("t < 8".into()),
                                ordered_condition: Condition::atom("t < 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "baz1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("t < 10".into()) & !Condition::atom("t < 8".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "baz2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("bar1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "bar1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("bar2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "bar2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("baz1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "baz1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("baz2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "baz2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_var_names_1() {
    let proj = Project::compile(include_str!("projects/var-names-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("my state".into(), StateMachine {
                variables: [
                    ("another_var".into(), "0".into()),
                    ("some_var".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first state".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "some_var = (some_var * another_var)".into(),
                                ].into_iter().collect(),
                                new_state: "second state".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("second state".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "second state".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_initial_state_1() {
    let proj = Project::compile(include_str!("projects/initial-state-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("foo 4".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "something" {
    "something"[shape=point,width=0.1]
    "something" -> "something foo 4"
    "something barb"[label="barb"]
    "something foo 3"[label="foo 3"]
    "something foo 4"[label="foo 4"]
    "something barb" -> "something foo 3" [label=""]
    "something foo 3" -> "something foo 4" [label=""]
    "something foo 4" -> "something barb" [label=""]
  }
}
    "#.trim());
    assert_eq!(proj.to_stateflow().unwrap(), r#"
sfnew untitled
chart = find(sfroot, "-isa", "Stateflow.Chart", Path = "untitled/Chart")
chart.Name = "something"
s0 = Stateflow.State(chart)
s0.Name = "barb"
s0.Position = [0, 0, 100, 100]
s1 = Stateflow.State(chart)
s1.Name = "foo_3"
s1.Position = [200, 0, 100, 100]
s2 = Stateflow.State(chart)
s2.Name = "foo_4"
s2.Position = [400, 0, 100, 100]
t = Stateflow.Transition(chart)
t.Source = s0
t.Destination = s1
t.LabelString = "[]{}"
t = Stateflow.Transition(chart)
t.Source = s1
t.Destination = s2
t.LabelString = "[]{}"
t = Stateflow.Transition(chart)
t.Source = s2
t.Destination = s0
t.LabelString = "[]{}"
t = Stateflow.Transition(chart)
t.Destination = s2
t.DestinationOClock = 0
t.SourceEndpoint = t.DestinationEndpoint - [0 30]
t.Midpoint = t.DestinationEndpoint - [0 15]
    "#.trim());
}

#[test]
fn test_initial_state_2() {
    let proj = Project::compile(include_str!("projects/initial-state-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_initial_state_3() {
    let proj = Project::compile(include_str!("projects/initial-state-3.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("foo 3".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_initial_state_4() {
    let proj = Project::compile(include_str!("projects/initial-state-4.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("foo 3".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("barb".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_unknown_blocks_1() {
    let err = Project::compile(include_str!("projects/unknown-blocks-1.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::UnsupportedBlock { state_machine: "thing".into(), state: "foo".into(), info: "CallRpc { host: None, service: \"CloudVariables\", rpc: \"deleteVariable\", args: [(\"name\", Expr { kind: Value(String(\"foo\")), info: BlockInfo { comment: None, location: None } }), (\"password\", Expr { kind: Value(String(\"bar\")), info: BlockInfo { comment: None, location: None } })] }".into() });

    let proj = Project::compile(include_str!("projects/unknown-blocks-1.xml"), None, Settings { omit_unknown_blocks: true, ..Settings::default() }).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thing".into(), StateMachine {
                variables: [
                    ("derp".into(), "0".into()),
                    ("merp".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "merp = 10".into(),
                                    "?".into(),
                                    "merp = 20".into(),
                                    "derp = ?".into(),
                                    "merp = 30".into(),
                                ].into_iter().collect(),
                                new_state: "foo".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_unknown_blocks_2() {
    let err = Project::compile(include_str!("projects/unknown-blocks-2.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::UnsupportedBlock { state_machine: "thing".into(), state: "foo".into(), info: "CallRpc { host: None, service: \"CloudVariables\", rpc: \"deleteVariable\", args: [(\"name\", Expr { kind: Value(String(\"foo\")), info: BlockInfo { comment: None, location: None } }), (\"password\", Expr { kind: Value(String(\"bar\")), info: BlockInfo { comment: None, location: None } })] }".into() });

    let proj = Project::compile(include_str!("projects/unknown-blocks-2.xml"), None, Settings { omit_unknown_blocks: true, ..Settings::default() }).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thing".into(), StateMachine {
                variables: [
                    ("derp".into(), "0".into()),
                    ("merp".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "merp = 10".into(),
                                    "merp = 20".into(),
                                    "derp = ?".into(),
                                    "merp = 30".into(),
                                ].into_iter().collect(),
                                new_state: "foo".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_unknown_blocks_3() {
    let err = Project::compile(include_str!("projects/unknown-blocks-3.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::UnsupportedBlock { state_machine: "thing".into(), state: "foo".into(), info: "TurnRight { angle: Expr { kind: Value(String(\"15\")), info: BlockInfo { comment: None, location: None } } }".into() });

    let proj = Project::compile(include_str!("projects/unknown-blocks-3.xml"), None, Settings { omit_unknown_blocks: true, ..Settings::default() }).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thing".into(), StateMachine {
                variables: [
                    ("merp".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "merp = 10".into(),
                                    "?".into(),
                                    "merp = 20".into(),
                                    "merp = 30".into(),
                                ].into_iter().collect(),
                                new_state: "foo".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_unknown_blocks_4() {
    let err = Project::compile(include_str!("projects/unknown-blocks-4.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::UnsupportedBlock { state_machine: "player state".into(), state: "me stop".into(), info: "KeyDown { key: Expr { kind: Value(String(\"space\")), info: BlockInfo { comment: None, location: None } } }".into() });

    let proj = Project::compile(include_str!("projects/unknown-blocks-4.xml"), None, Settings { omit_unknown_blocks: true, ..Settings::default() }).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("player state".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("me stop".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("?".into()),
                                ordered_condition: Condition::atom("?".into()),
                                actions: [].into_iter().collect(),
                                new_state: "me go".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("?".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "me stop".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("me go".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "?".into(),
                                ].into_iter().collect(),
                                new_state: "me stop".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("me stop".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_rand_1() {
    let proj = Project::compile(include_str!("projects/rand-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "state-machine-dice".into(),
        role: "myRole".into(),
        state_machines: [
            ("my state".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("rolling".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "a = randi(6)".into(),
                                    "a = randi(9)".into(),
                                    "a = randi([2, 4])".into(),
                                    "b = randi([(a - b), (a + b)])".into(),
                                ].into_iter().collect(),
                                new_state: "rolling".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("rolling".into()),
                current_state: Some("rolling".into()),
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_current_state_1() {
    let proj = Project::compile(include_str!("projects/current-state-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("a".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "b".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("b".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "a".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("a".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "thingy" {
    "thingy"[shape=point,width=0.1]
    "thingy" -> "thingy a"
    "thingy a"[label="a"]
    "thingy b"[label="b"]
    "thingy a" -> "thingy b" [label=""]
    "thingy b" -> "thingy a" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_current_state_2() {
    let proj = Project::compile(include_str!("projects/current-state-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("a".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "b".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("b".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "a".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("a".into()),
                current_state: Some("a".into()),
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "thingy" {
    "thingy"[shape=point,width=0.1]
    "thingy" -> "thingy a"
    "thingy a"[label="a",style=filled]
    "thingy b"[label="b"]
    "thingy a" -> "thingy b" [label=""]
    "thingy b" -> "thingy a" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_current_state_3() {
    let proj = Project::compile(include_str!("projects/current-state-3.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("a".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "b".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("b".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "a".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("a".into()),
                current_state: Some("b".into()),
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "thingy" {
    "thingy"[shape=point,width=0.1]
    "thingy" -> "thingy a"
    "thingy a"[label="a"]
    "thingy b"[label="b",style=filled]
    "thingy a" -> "thingy b" [label=""]
    "thingy b" -> "thingy a" [label=""]
  }
}
    "#.trim());
}

#[test]
fn test_junctions_1() {
    let proj = Project::compile(include_str!("projects/junctions-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "junctions".into(),
        role: "myRole".into(),
        state_machines: [
            ("my state".into(), StateMachine {
                variables: [
                    ("foo".into(), "43".into()),
                ].into_iter().collect(),
                states: [
                    ("abc".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "foo = (2 * foo)".into(),
                                ].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("abc".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("foo > 1024".into()),
                                ordered_condition: Condition::atom("foo > 1024".into()),
                                actions: [].into_iter().collect(),
                                new_state: "xyz".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("foo > 1024".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "abc".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("xyz".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "xyz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("abc".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_junctions_2() {
    let proj = Project::compile(include_str!("projects/junctions-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("something".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "a = (a + 1)".into(),
                                    "b = (a + b + 2)".into(),
                                ].into_iter().collect(),
                                new_state: "::junction-1::".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("something".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("(a + b) > 10".into()),
                                ordered_condition: Condition::atom("(a + b) > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "x2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("(a + b) > 10".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "a = (a ^ b)".into(),
                                    "b = (a + b)".into(),
                                ].into_iter().collect(),
                                new_state: "something".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-1::".into(), State {
                        parent: Some("something".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("(a * b) > 100".into()),
                                ordered_condition: Condition::atom("(a * b) > 100".into()),
                                actions: [].into_iter().collect(),
                                new_state: "x1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("(a * b) > 100".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "a = (a / b)".into(),
                                    "b = (1 / b)".into(),
                                ].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("x1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "x1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("x2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "x2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("something".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "thingy" {
    "thingy"[shape=point,width=0.1]
    "thingy" -> "thingy something"
    "thingy ::junction-0::"[label="",shape=circle,width=0.1]
    "thingy ::junction-1::"[label="",shape=circle,width=0.1]
    "thingy something"[label="something"]
    "thingy x1"[label="x1"]
    "thingy x2"[label="x2"]
    "thingy ::junction-0::" -> "thingy x2" [label=" 1: (a + b) > 10 "]
    "thingy ::junction-0::" -> "thingy something" [label=" 2 "]
    "thingy ::junction-1::" -> "thingy x1" [label=" 1: (a * b) > 100 "]
    "thingy ::junction-1::" -> "thingy ::junction-0::" [label=" 2 "]
    "thingy something" -> "thingy ::junction-1::" [label=""]
  }
}
    "#.trim());
    assert_eq!(proj.to_stateflow().unwrap(), r#"
sfnew untitled
chart = find(sfroot, "-isa", "Stateflow.Chart", Path = "untitled/Chart")
chart.Name = "thingy"
s0 = Stateflow.Junction(chart)
s0.Position.Center = [50, 200]
s1 = Stateflow.Junction(chart)
s1.Position.Center = [50, 300]
s2 = Stateflow.State(chart)
s2.Name = "something"
s2.Position = [0, 0, 100, 100]
s3 = Stateflow.State(chart)
s3.Name = "x1"
s3.Position = [200, 0, 100, 100]
s4 = Stateflow.State(chart)
s4.Name = "x2"
s4.Position = [400, 0, 100, 100]
t = Stateflow.Transition(chart)
t.Source = s0
t.Destination = s4
t.LabelString = "[(a + b) > 10]{}"
t = Stateflow.Transition(chart)
t.Source = s0
t.Destination = s2
t.LabelString = "[~((a + b) > 10)]{a = (a ^ b);b = (a + b);}"
t = Stateflow.Transition(chart)
t.Source = s1
t.Destination = s3
t.LabelString = "[(a * b) > 100]{}"
t = Stateflow.Transition(chart)
t.Source = s1
t.Destination = s0
t.LabelString = "[~((a * b) > 100)]{a = (a / b);b = (1 / b);}"
t = Stateflow.Transition(chart)
t.Source = s2
t.Destination = s1
t.LabelString = "[]{a = (a + 1);b = (a + b + 2);}"
t = Stateflow.Transition(chart)
t.Destination = s2
t.DestinationOClock = 0
t.SourceEndpoint = t.DestinationEndpoint - [0 30]
t.Midpoint = t.DestinationEndpoint - [0 15]
d = Stateflow.Data(chart)
d.Name = "a"
d.Props.InitialValue = "0"
d = Stateflow.Data(chart)
d.Name = "b"
d.Props.InitialValue = "0"
    "#.trim());
}

#[test]
fn test_double_trans() {
    let err = Project::compile(include_str!("projects/double-trans.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "thingy".into(), state: "first".into() });
}

#[test]
fn test_tail_condition_1() {
    let proj = Project::compile(include_str!("projects/tail-condition-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()),
                                ordered_condition: Condition::atom("a == 1".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_2() {
    let proj = Project::compile(include_str!("projects/tail-condition-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                ordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                            Transition {
                                unordered_condition: !(Condition::atom("a == 1".into()) & Condition::atom("a == 2".into())),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_3() {
    let proj = Project::compile(include_str!("projects/tail-condition-3.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_4() {
    let proj = Project::compile(include_str!("projects/tail-condition-4.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()),
                                ordered_condition: Condition::atom("a == 1".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_5() {
    let proj = Project::compile(include_str!("projects/tail-condition-5.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()),
                                ordered_condition: Condition::atom("a == 1".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                ordered_condition: Condition::atom("a == 2".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()) & !Condition::atom("a == 2".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_6() {
    let proj = Project::compile(include_str!("projects/tail-condition-6.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()),
                                ordered_condition: Condition::atom("a == 1".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_7() {
    let proj = Project::compile(include_str!("projects/tail-condition-7.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                ordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()),
                                ordered_condition: !Condition::atom("a == 1".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()) & !Condition::atom("a == 2".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_8() {
    let proj = Project::compile(include_str!("projects/tail-condition-8.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                ordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()) & Condition::atom("a == 3".into()),
                                ordered_condition: !Condition::atom("a == 1".into()) & Condition::atom("a == 3".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                            Transition {
                                unordered_condition: !(Condition::atom("a == 1".into()) & Condition::atom("a == 2".into())) & !(!Condition::atom("a == 1".into()) & Condition::atom("a == 3".into())),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_9() {
    let proj = Project::compile(include_str!("projects/tail-condition-9.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                ordered_condition: Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !(Condition::atom("a == 1".into()) & Condition::atom("a == 2".into())),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_10() {
    let proj = Project::compile(include_str!("projects/tail-condition-10.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()),
                                ordered_condition: !Condition::atom("a == 1".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("a == 1".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_11() {
    let proj = Project::compile(include_str!("projects/tail-condition-11.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: !Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                ordered_condition: !Condition::atom("a == 1".into()) & Condition::atom("a == 2".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                            Transition {
                                unordered_condition: !(!Condition::atom("a == 1".into()) & Condition::atom("a == 2".into())),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_tail_condition_12() {
    let proj = Project::compile(include_str!("projects/tail-condition-12.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                ordered_condition: Condition::constant(true),
                                unordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_completeness_1() {
    let proj = Project::compile(include_str!("projects/completeness-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_1() {
    let proj = Project::compile(include_str!("projects/prune-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_2() {
    let proj = Project::compile(include_str!("projects/prune-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_3() {
    let proj = Project::compile(include_str!("projects/prune-3.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_4() {
    let proj = Project::compile(include_str!("projects/prune-4.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_5() {
    let proj = Project::compile(include_str!("projects/prune-5.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_6() {
    let proj = Project::compile(include_str!("projects/prune-6.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "first".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_7() {
    let proj = Project::compile(include_str!("projects/prune-7.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == b".into()),
                                ordered_condition: Condition::atom("a == b".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == b".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_8() {
    let proj = Project::compile(include_str!("projects/prune-8.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == b".into()),
                                ordered_condition: Condition::atom("a == b".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == b".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_prune_9() {
    let proj = Project::compile(include_str!("projects/prune-9.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [
                    ("a".into(), "0".into()),
                    ("b".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("a == b".into()) & Condition::atom("b == 4".into()),
                                ordered_condition: Condition::atom("a == b".into()) & Condition::atom("b == 4".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                            Transition {
                                unordered_condition: Condition::atom("a == b".into()) & !Condition::atom("b == 4".into()),
                                ordered_condition: Condition::atom("a == b".into()),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("a == b".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 1".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("mid 2".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "mid 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("last".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "last".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_empty_condition() {
    let proj = Project::compile(include_str!("projects/empty-condition.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("thingy".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("first".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "second".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("second".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "second".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_wait_1() {
    let proj = Project::compile(include_str!("projects/wait-1.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "wait".into(),
        role: "myRole".into(),
        state_machines: [
            ("my state".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("start".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("after(3, sec)".into()),
                                ordered_condition: Condition::atom("after(3, sec)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("after(3, sec)".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "start".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("stop".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "stop".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("start".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "stop".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("start".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}

#[test]
fn test_wait_2() {
    let proj = Project::compile(include_str!("projects/wait-2.xml"), None, Settings::default()).unwrap();
    assert_eq!(proj, Project {
        name: "wait".into(),
        role: "myRole".into(),
        state_machines: [
            ("my state".into(), StateMachine {
                variables: [
                    ("x".into(), "0".into()),
                ].into_iter().collect(),
                states: [
                    ("start".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("after(3, sec)".into()),
                                ordered_condition: Condition::atom("after(3, sec)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "::junction-0::".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("after(3, sec)".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "start".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("stop".into(), State {
                        parent: None,
                        transitions: [
                            Transition {
                                unordered_condition: Condition::constant(true),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "stop".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("::junction-0::".into(), State {
                        parent: Some("start".into()),
                        transitions: [
                            Transition {
                                unordered_condition: Condition::atom("x".into()),
                                ordered_condition: Condition::atom("x".into()),
                                actions: [].into_iter().collect(),
                                new_state: "stop".into(),
                            },
                            Transition {
                                unordered_condition: !Condition::atom("x".into()),
                                ordered_condition: Condition::constant(true),
                                actions: [].into_iter().collect(),
                                new_state: "start".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("start".into()),
                current_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_complete(&proj);
}
