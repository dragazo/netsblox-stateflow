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
                        transitions: vec![
                            Transition {
                                condition: None,
                                new_state: "thing 2".into(),
                            },
                        ],
                    }),
                    ("thing 2".into(), State {
                        transitions: vec![
                            Transition {
                                condition: None,
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
