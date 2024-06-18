use crate::*;

#[test]
fn test_empty_project() {
    let proj = compile(include_str!("projects/empty-project.xml")).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        state_machines: [].into_iter().collect(),
    });
}

#[test]
fn test_simple() {
    let proj = compile(include_str!("projects/simple.xml")).unwrap();
    assert_eq!(proj, Project {
        name: "untitled".into(),
        state_machines: [
            ("something".into(), StateMachine {
                states: [
                    ("thing 1".into(), State {

                    }),
                    ("thing 2".into(), State {

                    }),
                ].into_iter().collect(),
                initial_state: None,
            }),
        ].into_iter().collect(),
    });
}
