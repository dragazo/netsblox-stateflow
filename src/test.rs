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
                                condition: Some("~(t > 10 & t > 9)".into()),
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