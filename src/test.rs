use crate::*;

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
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 1" -> "something thing 2" [label=" 1 "]
    "something thing 2" -> "something thing 1" [label=" 1 "]
  }
}
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
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 3"[label="thing 3"]
    "something thing 1" -> "something thing 2" [label=" 1 "]
    "something thing 2" -> "something thing 3" [label=" 1 "]
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
    let proj = Project::compile(include_str!("projects/if-timer-reset-1.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/if-timer-reset-2.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/if-timer-reset-3.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/no-transitions-1.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/no-transitions-2.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/if-chain-1.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/if-chain-2.xml"), None, Settings::default()).unwrap();
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
  subgraph "something" {
    "something thing 1"[label="thing 1"]
    "something thing 2"[label="thing 2"]
    "something thing 3"[label="thing 3"]
    "something thing 4"[label="thing 4"]
    "something thing 1" -> "something thing 4" [label=" 1: t > 8 "]
    "something thing 1" -> "something thing 3" [label=" 2: t > 9 "]
    "something thing 1" -> "something thing 2" [label=" 3: t > 10 "]
    "something thing 2" -> "something thing 1" [label=" 1 "]
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
    let proj = Project::compile(include_str!("projects/nested-if-2.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/nested-if-3.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/nested-if-4.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/nested-if-5.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/nested-if-6.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/variables-1.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/variables-2.xml"), None, Settings::default()).unwrap();
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
    let err = Project::compile(include_str!("projects/variables-3.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::VariableOverlap { state_machines: ("another".into(), "something".into()), variable: "foo".into() });
}

#[test]
fn test_variables_4() {
    let err = Project::compile(include_str!("projects/variables-4.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::VariableOverlap { state_machines: ("something".into(), "another".into()), variable: "another".into() });
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
    let proj = Project::compile(include_str!("projects/if-else-6.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/if-else-7.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/if-else-8.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/operators.xml"), None, Settings::default()).unwrap();
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
    let proj = Project::compile(include_str!("projects/actions-1.xml"), None, Settings::default()).unwrap();
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
    let err = Project::compile(include_str!("projects/actions-2.xml"), None, Settings::default()).unwrap_err();
    assert_eq!(err, CompileError::ActionsOutsideTransition { state_machine: "state".into(), state: "state 1".into() });
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
                        transitions: [
                            Transition {
                                unordered_condition: Some("t < 10".into()),
                                ordered_condition: Some("t < 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "bar".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t < 10)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "baz".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("bar".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("baz".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
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
                        transitions: [
                            Transition {
                                unordered_condition: Some("t < 10 & t < 9".into()),
                                ordered_condition: Some("t < 10 & t < 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "bar1".into(),
                            },
                            Transition {
                                unordered_condition: Some("t < 10 & ~(t < 9)".into()),
                                ordered_condition: Some("t < 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "bar2".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t < 10) & t < 8".into()),
                                ordered_condition: Some("t < 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "baz1".into(),
                            },
                            Transition {
                                unordered_condition: Some("~(t < 10) & ~(t < 8)".into()),
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "baz2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("bar1".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("bar2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("baz1".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                    ("baz2".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
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
                    "another_var".into(),
                    "some_var".into(),
                ].into_iter().collect(),
                states: [
                    ("first state".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [
                                    "some_var = (some_var * another_var)".into(),
                                ].into_iter().collect(),
                                new_state: "second state".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("second state".into(), State {
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
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
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("foo 4".into()),
            }),
        ].into_iter().collect(),
    });
    assert_eq!(graphviz_rust::print(proj.to_graphviz(), &mut Default::default()), r#"
digraph "untitled" {
  subgraph "something" {
    "something"[style=invis]
    "something" -> "something foo 4"
    "something barb"[label="barb"]
    "something foo 3"[label="foo 3"]
    "something foo 4"[label="foo 4"]
    "something barb" -> "something foo 3" [label=" 1 "]
    "something foo 3" -> "something foo 4" [label=" 1 "]
    "something foo 4" -> "something barb" [label=" 1 "]
  }
}
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
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
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
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("foo 3".into()),
            }),
        ].into_iter().collect(),
    });
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
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("foo 4".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "barb".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("barb".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "foo 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("barb".into()),
            }),
        ].into_iter().collect(),
    });
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
                    "derp".into(),
                    "merp".into(),
                ].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
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
            }),
        ].into_iter().collect(),
    });
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
                    "derp".into(),
                    "merp".into(),
                ].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
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
            }),
        ].into_iter().collect(),
    });
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
                    "merp".into(),
                ].into_iter().collect(),
                states: [
                    ("foo".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
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
            }),
        ].into_iter().collect(),
    });
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
                        transitions: [
                            Transition {
                                unordered_condition: Some("?".into()),
                                ordered_condition: Some("?".into()),
                                actions: [].into_iter().collect(),
                                new_state: "me go".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("me go".into(), State {
                        transitions: [
                            Transition {
                                unordered_condition: None,
                                ordered_condition: None,
                                actions: [
                                    "?".into(),
                                ].into_iter().collect(),
                                new_state: "me stop".into(),
                            },
                        ].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: Some("me stop".into()),
            }),
        ].into_iter().collect(),
    });
}
