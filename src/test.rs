use crate::*;

#[test]
fn test_empty_project() {
    let proj = Project::compile(include_str!("projects/empty-project.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [].into_iter().collect(),
    });
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {

}
    "#.trim());
}

#[test]
fn test_simple() {
    let proj = Project::compile(include_str!("projects/simple.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "cluster something" {
    graph[label="something",style="rounded"]
    "thing 1"
    "thing 2"
    "thing 1" -> "thing 2" [label=" 1 "]
    "thing 2" -> "thing 1" [label=" 1 "]
  }
}
    "#.trim());
}

#[test]
fn test_simple_no_handler() {
    let proj = Project::compile(include_str!("projects/simple-no-handler.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "cluster something" {
    graph[label="something",style="rounded"]
    "thing 1"
    "thing 2"
    "thing 3"
    "thing 1" -> "thing 2" [label=" 1 "]
    "thing 2" -> "thing 3" [label=" 1 "]
  }
}
    "#.trim());
}

#[test]
fn test_simple_if_timer() {
    let proj = Project::compile(include_str!("projects/simple-if-timer.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_timer_reset_1() {
    let proj = Project::compile(include_str!("projects/if-timer-reset-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_timer_reset_2() {
    let proj = Project::compile(include_str!("projects/if-timer-reset-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_timer_reset_3() {
    let proj = Project::compile(include_str!("projects/if-timer-reset-3.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_no_transitions_1() {
    let proj = Project::compile(include_str!("projects/no-transitions-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_no_transitions_2() {
    let proj = Project::compile(include_str!("projects/no-transitions-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_chain_1() {
    let proj = Project::compile(include_str!("projects/if-chain-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 9".into()),
                                ordered_condition: Some("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 9) & t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_chain_2() {
    let proj = Project::compile(include_str!("projects/if-chain-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 8".into()),
                                ordered_condition: Some("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 8) & t > 9".into()),
                                ordered_condition: Some("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 8) & ~(t > 9) & t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                ordered_condition: None,
                                unordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "cluster something" {
    graph[label="something",style="rounded"]
    "thing 1"
    "thing 2"
    "thing 3"
    "thing 4"
    "thing 1" -> "thing 4" [label=" 1: t > 8 "]
    "thing 1" -> "thing 3" [label=" 2: t > 9 "]
    "thing 1" -> "thing 2" [label=" 3: t > 10 "]
    "thing 2" -> "thing 1" [label=" 1 "]
  }
}
    "#.trim());
}

#[test]
fn test_nested_if_1() {
    let proj = Project::compile(include_str!("projects/nested-if-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_nested_if_2() {
    let proj = Project::compile(include_str!("projects/nested-if-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9".into()),
                                ordered_condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_nested_if_3() {
    let proj = Project::compile(include_str!("projects/nested-if-3.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9".into()),
                                ordered_condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                ordered_condition: Some("t > 10 & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_nested_if_4() {
    let proj = Project::compile(include_str!("projects/nested-if-4.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                ordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                ordered_condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                ordered_condition: Some("t > 10 & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_nested_if_5() {
    let proj = Project::compile(include_str!("projects/nested-if-5.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                ordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                ordered_condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                ordered_condition: Some("t > 10 & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & ~(t > 8)".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_nested_if_6() {
    let proj = Project::compile(include_str!("projects/nested-if-6.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                ordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [
                                    "foo = 234".into(),
                                    "foo = 652".into(),
                                ].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                ordered_condition: Some("t > 10 & t > 9".into()),
                                actions: [
                                    "foo = 123".into(),
                                    "foo = 453".into(),
                                ].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                ordered_condition: Some("t > 10 & t > 8".into()),
                                actions: [
                                    "foo = 546".into(),
                                    "foo = 876".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & ~(t > 8)".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [
                                    "foo = 431".into(),
                                    "foo = 197".into(),
                                ].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [
                                    "foo = 856".into(),
                                    "foo = 465".into(),
                                ].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_variables_1() {
    let proj = Project::compile(include_str!("projects/variables-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [
                                    "foo = 14".into(),
                                    "foo = 21".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
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
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_variables_2() {
    let proj = Project::compile(include_str!("projects/variables-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [
                                    "foo = 14".into(),
                                    "foo = 21".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
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
            }),
            ("another".into(), StateMachine {
                variables: [
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("test 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [
                                    "bar = 231".into(),
                                    "bar = 453".into(),
                                ].into_iter().collect(),
                                new_state: "test 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("test 2".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
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
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_variables_3() {
    let err = Project::compile(include_str!("projects/variables-3.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::VariableOverlap { state_machines: ("another".into(), "something".into()), variable: "foo".into() });
}

#[test]
fn test_variables_4() {
    let err = Project::compile(include_str!("projects/variables-4.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::VariableOverlap { state_machines: ("something".into(), "another".into()), variable: "another".into() });
}

#[test]
fn test_if_else_1() {
    let proj = Project::compile(include_str!("projects/if-else-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("foo == bar".into()),
                                ordered_condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo == bar)".into()),
                                ordered_condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_else_2() {
    let err = Project::compile(include_str!("projects/if-else-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_3() {
    let err = Project::compile(include_str!("projects/if-else-3.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_4() {
    let err = Project::compile(include_str!("projects/if-else-4.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_5() {
    let proj = Project::compile(include_str!("projects/if-else-5.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("foo == bar".into()),
                                ordered_condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo == bar)".into()),
                                ordered_condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("false".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_else_6() {
    let proj = Project::compile(include_str!("projects/if-else-6.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("foo == bar".into()),
                                ordered_condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo == bar)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_else_7() {
    let proj = Project::compile(include_str!("projects/if-else-7.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("~(foo == bar)".into()),
                                ordered_condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("foo == bar".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_else_8() {
    let proj = Project::compile(include_str!("projects/if-else-8.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_else_9() {
    let err = Project::compile(include_str!("projects/if-else-9.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_10() {
    let err = Project::compile(include_str!("projects/if-else-10.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_11() {
    let err = Project::compile(include_str!("projects/if-else-11.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_12() {
    let err = Project::compile(include_str!("projects/if-else-12.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_fall_through_1() {
    let err = Project::compile(include_str!("projects/if-fall-through-1.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_fall_through_2() {
    let err = Project::compile(include_str!("projects/if-fall-through-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_simple_fall_through_1() {
    let err = Project::compile(include_str!("projects/simple-fall-through-1.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_simple_fall_through_2() {
    let err = Project::compile(include_str!("projects/simple-fall-through-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_multiple_handlers() {
    let err = Project::compile(include_str!("projects/multiple-handlers.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::MultipleHandlers { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_transition_1() {
    let err = Project::compile(include_str!("projects/complex-transition-1.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ComplexTransitionName { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_transition_2() {
    let err = Project::compile(include_str!("projects/complex-transition-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ComplexTransitionName { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_tail_actions_1() {
    let proj = Project::compile(include_str!("projects/tail-actions-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("foo > bar".into()),
                                ordered_condition: Some("foo > bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo > bar)".into()),
                                ordered_condition: None,
                                actions: [
                                    "foo = (2 * foo * 2)".into(),
                                    "bar = (3 * bar)".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_operators() {
    let proj = Project::compile(include_str!("projects/operators.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [
                    "foo".into(),
                    "bar".into(),
                ].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("foo < bar".into()),
                                ordered_condition: Some("foo < bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & foo <= bar".into()),
                                ordered_condition: Some("foo <= bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & foo > bar".into()),
                                ordered_condition: Some("foo > bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & foo >= bar".into()),
                                ordered_condition: Some("foo >= bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & ~(foo >= bar) & foo == bar".into()),
                                ordered_condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 6".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & ~(foo >= bar) & ~(foo == bar) & foo ~= bar".into()),
                                ordered_condition: Some("foo ~= bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 7".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & ~(foo >= bar) & ~(foo == bar) & ~(foo ~= bar) & foo < bar & false".into()),
                                ordered_condition: Some("foo < bar & false".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 8".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & ~(foo >= bar) & ~(foo == bar) & ~(foo ~= bar) & ~(foo < bar & false) & (true | foo > bar)".into()),
                                ordered_condition: Some("(true | foo > bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 9".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & ~(foo >= bar) & ~(foo == bar) & ~(foo ~= bar) & ~(foo < bar & false) & ~((true | foo > bar)) & ~(foo == bar)".into()),
                                ordered_condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 10".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(foo < bar) & ~(foo <= bar) & ~(foo > bar) & ~(foo >= bar) & ~(foo == bar) & ~(foo ~= bar) & ~(foo < bar & false) & ~((true | foo > bar)) & ~(~(foo == bar))".into()),
                                ordered_condition: None,
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
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 6".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 7".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 8".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 9".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 10".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_actions_1() {
    let proj = Project::compile(include_str!("projects/actions-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("state".into(), StateMachine {
                variables: [
                    "foo".into(),
                ].into_iter().collect(),
                states: [
                    ("state 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [
                                    "foo = 456".into()
                                ].into_iter().collect(),
                                new_state: "state 1".into(),
                            }
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_actions_2() {
    let err = Project::compile(include_str!("projects/actions-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "state".into(), state: "state 1".into() });
}

#[test]
fn test_ext_blocks_1() {
    let proj = Project::compile(include_str!("projects/ext-blocks-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                ordered_condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                ordered_condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                ordered_condition: Some("t > 10 & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                unordered_condition: Some("t > 10 & ~(t > 9) & ~(t > 8)".into()),
                                ordered_condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t > 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}
