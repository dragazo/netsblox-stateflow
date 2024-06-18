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
                        transitions: vec![
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ],
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: vec![
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ],
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
                        transitions: vec![
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ],
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: vec![
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 3".into(),
                            },
                        ],
                    }),
                    ("thing 3".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: vec![],
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
                        transitions: vec![
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ],
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: vec![
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ],
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
                        transitions: vec![
                            Transition {
                                condition: Some("t > 10".into()),
                                actions: [].into_iter().collect(),
                                new_state: "thing 2".into(),
                            },
                        ],
                    }),
                    ("thing 2".into(), State {
                        actions: [].into_iter().collect(),
                        transitions: vec![
                            Transition {
                                condition: None,
                                actions: [].into_iter().collect(),
                                new_state: "thing 1".into(),
                            },
                        ],
                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}