use crate::*;

#[test]
fn test_empty_project() {
    let proj = compile(include_str!("projects/empty-project.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [].into_iter().collect(),
    });
}

#[test]
fn test_simple() {
    let proj = compile(include_str!("projects/simple.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
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
fn test_simple_no_handler() {
    let proj = compile(include_str!("projects/simple-no-handler.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_simple_if_timer() {
    let proj = compile(include_str!("projects/simple-if-timer.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
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
    let proj = compile(include_str!("projects/if-timer-reset-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [
                            "t = 0".into(),
                        ].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
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
    let proj = compile(include_str!("projects/if-timer-reset-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [
                                    "t = 0".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
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
    let proj = compile(include_str!("projects/if-timer-reset-3.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [
                            "t = 0".into(),
                        ].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
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
fn test_no_transitions_1() {
    let proj = compile(include_str!("projects/no-transitions-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [
                            "t = 0".into(),
                        ].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_no_transitions_2() {
    let proj = compile(include_str!("projects/no-transitions-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/if-chain-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("~(t > 9) & t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/if-chain-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                condition: Some("~(t > 8) & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("~(t > 8) & ~(t > 9) & t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_nested_if_1() {
    let proj = compile(include_str!("projects/nested-if-1.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(t > 10)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/nested-if-2.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(t > 10)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/nested-if-3.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10 & t > 9".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("~(t > 10)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/nested-if-4.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(t > 10)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/nested-if-5.xml"), None).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        role: "myRole".into(),
        state_machines: [
            ("something".into(), StateMachine {
                variables: [].into_iter().collect(),
                states: [
                    ("thing 1".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & ~(t > 9) & ~(t > 8)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                condition: Some("~(t > 10)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/nested-if-6.xml"), None).unwrap();
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
                        actions: [
                            "foo = 12".into(),
                            "foo = 32".into(),
                        ].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("t > 10 & t > 9 & t > 7".into()),
                                actions: [
                                    "foo = 67".into(),
                                    "foo = 54".into(),
                                    "foo = 994".into(),
                                    "foo = 786".into(),
                                    "foo = 234".into(),
                                    "foo = 652".into(),
                                ].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & t > 9 & ~(t > 7)".into()),
                                actions: [
                                    "foo = 67".into(),
                                    "foo = 54".into(),
                                    "foo = 994".into(),
                                    "foo = 786".into(),
                                    "foo = 123".into(),
                                    "foo = 453".into(),
                                ].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & ~(t > 9) & t > 8".into()),
                                actions: [
                                    "foo = 67".into(),
                                    "foo = 54".into(),
                                    "foo = 546".into(),
                                    "foo = 876".into(),
                                ].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("t > 10 & ~(t > 9) & ~(t > 8)".into()),
                                actions: [
                                    "foo = 67".into(),
                                    "foo = 54".into(),
                                    "foo = 431".into(),
                                    "foo = 197".into(),
                                ].into_iter().collect(),
                                new_state: "thing 5".into(),
                            },
                            Transition {
                                condition: Some("~(t > 10)".into()),
                                actions: [
                                    "foo = 856".into(),
                                    "foo = 465".into(),
                                ].into_iter().collect(),
                                new_state: "thing 0".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 0".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 5".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_simple_variables() {
    let proj = compile(include_str!("projects/simple-variables.xml"), None).unwrap();
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
                        actions: [
                            "foo = 14".into(),
                            "foo = 21".into(),
                        ].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [
                            "foo = 76".into(),
                            "foo = 43".into(),
                        ].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
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
fn test_if_else_1() {
    let proj = compile(include_str!("projects/if-else-1.xml"), None).unwrap();
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
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
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
    let err = compile(include_str!("projects/if-else-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_3() {
    let err = compile(include_str!("projects/if-else-3.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_4() {
    let err = compile(include_str!("projects/if-else-4.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_5() {
    let proj = compile(include_str!("projects/if-else-5.xml"), None).unwrap();
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
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                            Transition {
                                condition: Some("false".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/if-else-6.xml"), None).unwrap();
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
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/if-else-7.xml"), None).unwrap();
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
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("~(foo == bar)".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("foo == bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
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
    let proj = compile(include_str!("projects/if-else-8.xml"), None).unwrap();
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
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
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
    let err = compile(include_str!("projects/if-else-9.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_10() {
    let err = compile(include_str!("projects/if-else-10.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_else_11() {
    let proj = compile(include_str!("projects/if-else-11.xml"), None).unwrap();
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
                        actions: [
                            "t = 0".into(),
                        ].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 4".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 4".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}

#[test]
fn test_if_else_12() {
    let err = compile(include_str!("projects/if-else-12.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_fall_through_1() {
    let err = compile(include_str!("projects/if-fall-through-1.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_if_fall_through_2() {
    let err = compile(include_str!("projects/if-fall-through-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_simple_fall_through_1() {
    let err = compile(include_str!("projects/simple-fall-through-1.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_simple_fall_through_2() {
    let err = compile(include_str!("projects/simple-fall-through-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_nonterminal() {
    let err = compile(include_str!("projects/complex-nonterminal.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::NonTerminalTransition { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_multiple_handlers() {
    let err = compile(include_str!("projects/multiple-handlers.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::MultipleHandlers { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_transition_1() {
    let err = compile(include_str!("projects/complex-transition-1.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ComplexTransitionName { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_complex_transition_2() {
    let err = compile(include_str!("projects/complex-transition-2.xml"), None).unwrap_err();
    assert_eq!(err, CompileError::ComplexTransitionName { state_machine: "something".into(), state: "thing 1".into() });
}

#[test]
fn test_tail_actions_1() {
    let proj = compile(include_str!("projects/tail-actions-1.xml"), None).unwrap();
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
                        actions: [].into_iter().collect(),
                        transitions: [
                            Transition {
                                condition: Some("foo > bar".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                            Transition {
                                condition: Some("~(foo > bar)".into()),
                                actions: [
                                    "foo = 2 * foo * 2".into(),
                                    "bar = 3 * bar".into(),
                                ].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ].into_iter().collect(),
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: [].into_iter().collect(),
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}
