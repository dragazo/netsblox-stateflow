use core::ops::{BitAnd, BitOr, Not};
use core::fmt;

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::collections::BTreeSet;

use netsblox_ast::CompactString;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    Const(bool),
    Atom(CompactString),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl BitAnd for Condition {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Condition::And(Box::new(self), Box::new(rhs))
    }
}
impl BitOr for Condition {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Condition::Or(Box::new(self), Box::new(rhs))
    }
}
impl Not for Condition {
    type Output = Self;
    fn not(self) -> Self::Output {
        Condition::Not(Box::new(self))
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Const(x) => write!(f, "{x}"),
            Condition::Atom(x) => write!(f, "{x}"),
            Condition::And(a, b) => {
                fn single(f: &mut fmt::Formatter<'_>, v: &Condition) -> fmt::Result {
                    match v {
                        Condition::Or(_, _) => write!(f, "({v})"),
                        _ => write!(f, "{v}"),
                    }
                }
                single(f, a)?;
                write!(f, " & ")?;
                single(f, b)
            }
            Condition::Or(a, b) => {
                fn single(f: &mut fmt::Formatter<'_>, v: &Condition) -> fmt::Result {
                    match v {
                        Condition::And(_, _) => write!(f, "({v})"),
                        _ => write!(f, "{v}"),
                    }
                }
                single(f, a)?;
                write!(f, " | ")?;
                single(f, b)
            }
            Condition::Not(x) => {
                let inside = x.to_string();
                if inside.chars().all(char::is_alphanumeric) {
                    write!(f, "~{inside}")
                } else {
                    write!(f, "~({inside})")
                }
            }
        }
    }
}

#[test]
fn test_condition() {
    let a = Condition::Atom("a".into());
    let b = Condition::Atom("b".into());
    let c = Condition::Atom("c".into());
    let d = Condition::Atom("d".into());
    let e = Condition::Atom("x < 10".into());
    let f = Condition::Atom("y == x + 10".into());

    let bt = Condition::Const(true);
    let bf = Condition::Const(false);

    assert_eq!(a.to_string(), "a");
    assert_eq!(b.to_string(), "b");
    assert_eq!(c.to_string(), "c");
    assert_eq!(d.to_string(), "d");
    assert_eq!(e.to_string(), "x < 10");
    assert_eq!(f.to_string(), "y == x + 10");
    assert_eq!(bt.to_string(), "true");
    assert_eq!(bf.to_string(), "false");

    assert_eq!((a.clone() & b.clone()).to_string(), "a & b");
    assert_eq!((a.clone() & b.clone() & c.clone()).to_string(), "a & b & c");
    assert_eq!((a.clone() & b.clone() & c.clone() & d.clone()).to_string(), "a & b & c & d");

    assert_eq!((a.clone() | b.clone()).to_string(), "a | b");
    assert_eq!((a.clone() | b.clone() | c.clone()).to_string(), "a | b | c");
    assert_eq!((a.clone() | b.clone() | c.clone() | d.clone()).to_string(), "a | b | c | d");

    assert_eq!((a.clone() & b.clone() & a.clone()).to_string(), "a & b & a");
    assert_eq!(((a.clone() & b.clone()) & a.clone()).to_string(), "a & b & a");
    assert_eq!((a.clone() & (b.clone() & a.clone())).to_string(), "a & b & a");

    assert_eq!(((a.clone() & b.clone()) | c.clone()).to_string(), "(a & b) | c");
    assert_eq!((c.clone() | (a.clone() & b.clone())).to_string(), "c | (a & b)");
    assert_eq!(((c.clone() & a.clone()) | (a.clone() & b.clone())).to_string(), "(c & a) | (a & b)");

    assert_eq!(((a.clone() | b.clone()) & c.clone()).to_string(), "(a | b) & c");
    assert_eq!((c.clone() & (a.clone() | b.clone())).to_string(), "c & (a | b)");
    assert_eq!(((c.clone() | a.clone()) & (a.clone() | b.clone())).to_string(), "(c | a) & (a | b)");

    assert_eq!(((a.clone() & (c.clone() & b.clone())) & a.clone()).to_string(), "a & c & b & a");
    assert_eq!(((a.clone() & (c.clone() & b.clone())) & (a.clone() & a.clone() & c.clone())).to_string(), "a & c & b & a & a & c");

    assert_eq!((!a.clone()).to_string(), "~a");
    assert_eq!((!c.clone()).to_string(), "~c");
    assert_eq!((!e.clone()).to_string(), "~(x < 10)");
    assert_eq!((!f.clone()).to_string(), "~(y == x + 10)");

    assert_eq!((!!a.clone()).to_string(), "~(~a)");
    assert_eq!((!!c.clone()).to_string(), "~(~c)");
    assert_eq!((!!e.clone()).to_string(), "~(~(x < 10))");
    assert_eq!((!!f.clone()).to_string(), "~(~(y == x + 10))");

    assert_eq!((bt.clone() & a.clone()).to_string(), "true & a");
    assert_eq!((a.clone() & bt.clone()).to_string(), "a & true");

    assert_eq!((bf.clone() | a.clone()).to_string(), "false | a");
    assert_eq!((a.clone() | bf.clone()).to_string(), "a | false");

    assert_eq!((bf.clone() & a.clone()).to_string(), "false & a");
    assert_eq!((a.clone() & bf.clone()).to_string(), "a & false");

    assert_eq!((bt.clone() | a.clone()).to_string(), "true | a");
    assert_eq!((a.clone() | bt.clone()).to_string(), "a | true");

    assert_eq!((a.clone() | !a.clone()).to_string(), "a | ~a");
}

impl Condition {
    fn visit_and<F: FnMut(&Condition)>(&self, f: &mut F) {
        match self {
            Condition::And(x, y) => {
                x.visit_and(f);
                y.visit_and(f);
            }
            x => f(x),
        }
    }
    fn visit_or<F: FnMut(&Condition)>(&self, f: &mut F) {
        match self {
            Condition::Or(x, y) => {
                x.visit_or(f);
                y.visit_or(f);
            }
            x => f(x),
        }
    }
    pub fn simpl(&self) -> Self {
        macro_rules! subset_simpl {
            ($terms:ident : $kind:ident : $visitor:ident) => {{
                let groups = $terms.iter().filter(|t| if let Condition::$kind(_, _) = t { true } else { false }).map(|t| {
                    let mut sub_terms: BTreeSet<Condition> = Default::default();
                    t.$visitor(&mut |x| { sub_terms.insert(x.clone()); });
                    (t.clone(), sub_terms)
                }).collect::<Vec<_>>();
                for (i, (group, sub_terms)) in groups.iter().enumerate() {
                    if sub_terms.iter().any(|t| $terms.contains(t)) || groups[..i].iter().chain(&groups[i + 1..]).any(|t| t.1.is_subset(&sub_terms)) {
                        assert!($terms.remove(group));
                    }
                }
            }}
        }

        match self {
            Condition::And(_, _) => {
                let mut terms: BTreeSet<Condition> = Default::default();
                self.visit_and(&mut |x| { terms.insert(x.simpl()); });

                subset_simpl!(terms : Or : visit_or);
                terms.remove(&Condition::Const(true));
                if terms.contains(&Condition::Const(false)) || terms.iter().any(|t| if let Condition::Not(t) = t { terms.contains(t) } else { false }) {
                    return Condition::Const(false);
                }

                terms.into_iter().reduce(|a, b| a & b).unwrap_or(Condition::Const(true))
            }
            Condition::Or(_, _) => {
                let mut terms: BTreeSet<Condition> = Default::default();
                self.visit_or(&mut |x| { terms.insert(x.simpl()); });

                subset_simpl!(terms : And : visit_and);
                terms.remove(&Condition::Const(false));
                if terms.contains(&Condition::Const(true)) || terms.iter().any(|t| if let Condition::Not(t) = t { terms.contains(t) } else { false }) {
                    return Condition::Const(true);
                }

                terms.into_iter().reduce(|a, b| a | b).unwrap_or(Condition::Const(false))
            }
            Condition::Not(x) => match &**x {
                Condition::Not(x) => x.simpl(),
                x => Condition::Not(Box::new(x.simpl())),
            }
            x => x.clone(),
        }
    }
}

#[test]
fn test_simpl() {
    let a = Condition::Atom("a".into());
    let b = Condition::Atom("b".into());
    let c = Condition::Atom("c".into());
    let d = Condition::Atom("d".into());
    let e = Condition::Atom("x < 10".into());
    let f = Condition::Atom("y == x + 10".into());

    let bt = Condition::Const(true);
    let bf = Condition::Const(false);

    assert_eq!((a.clone() & a.clone()).simpl().to_string(), "a");
    assert_eq!((a.clone() & c.clone() & a.clone()).simpl().to_string(), "a & c");
    assert_eq!((a.clone() & (c.clone() & a.clone())).simpl().to_string(), "a & c");
    assert_eq!((a.clone() & ((c.clone() | c.clone()) & a.clone())).simpl().to_string(), "a & c");
    assert_eq!((a.clone() & ((c.clone() | c.clone() | c.clone()) & a.clone())).simpl().to_string(), "a & c");

    assert_eq!((a.clone() | a.clone()).simpl().to_string(), "a");
    assert_eq!((a.clone() | c.clone() | a.clone()).simpl().to_string(), "a | c");
    assert_eq!((a.clone() | (c.clone() | a.clone())).simpl().to_string(), "a | c");
    assert_eq!((a.clone() | ((c.clone() & c.clone()) | a.clone())).simpl().to_string(), "a | c");
    assert_eq!((a.clone() | ((c.clone() & c.clone() & c.clone()) | a.clone())).simpl().to_string(), "a | c");

    assert_eq!((!f.clone()).simpl().to_string(), "~(y == x + 10)");
    assert_eq!((!!f.clone()).simpl().to_string(), "y == x + 10");
    assert_eq!((!!!f.clone()).simpl().to_string(), "~(y == x + 10)");
    assert_eq!((!!!!f.clone()).simpl().to_string(), "y == x + 10");

    assert_eq!((!(a.clone() | a.clone())).simpl().to_string(), "~a");
    assert_eq!((!(a.clone() & a.clone())).simpl().to_string(), "~a");

    assert_eq!((bf.clone() & bf.clone()).simpl().to_string(), "false");
    assert_eq!((bf.clone() & bt.clone()).simpl().to_string(), "false");
    assert_eq!((bt.clone() & bf.clone()).simpl().to_string(), "false");
    assert_eq!((bt.clone() & bt.clone()).simpl().to_string(), "true");

    assert_eq!((bf.clone() | bf.clone()).simpl().to_string(), "false");
    assert_eq!((bf.clone() | bt.clone()).simpl().to_string(), "true");
    assert_eq!((bt.clone() | bf.clone()).simpl().to_string(), "true");
    assert_eq!((bt.clone() | bt.clone()).simpl().to_string(), "true");

    assert_eq!((a.clone() & bt.clone()).simpl().to_string(), "a");
    assert_eq!((bt.clone() & a.clone() & bt.clone()).simpl().to_string(), "a");

    assert_eq!((a.clone() | bf.clone()).simpl().to_string(), "a");
    assert_eq!((bf.clone() | a.clone() | bf.clone()).simpl().to_string(), "a");

    assert_eq!((a.clone() & bf.clone()).simpl().to_string(), "false");
    assert_eq!((a.clone() | bt.clone()).simpl().to_string(), "true");

    assert_eq!((a.clone() | (a.clone() & b.clone())).simpl().to_string(), "a");
    assert_eq!(((a.clone() & b.clone()) | (a.clone() & b.clone())).simpl().to_string(), "a & b");
    assert_eq!(((a.clone() & b.clone()) | (a.clone() & c.clone() & b.clone())).simpl().to_string(), "a & b");
    assert_eq!(((a.clone() & c.clone() & b.clone() & c.clone()) | (a.clone() & b.clone())).simpl().to_string(), "a & b");
    assert_eq!(((a.clone() & c.clone() & b.clone() & c.clone()) | b.clone() | (a.clone() & b.clone())).simpl().to_string(), "b");
    assert_eq!(((a.clone() & c.clone() & b.clone() & c.clone()) | b.clone() | (a.clone() & d.clone() & b.clone())).simpl().to_string(), "b");
    assert_eq!(((a.clone() & c.clone() & b.clone() & c.clone()) | (b.clone() & c.clone()) | (a.clone() & d.clone() & b.clone())).simpl().to_string(), "(b & c) | (a & b & d)");
    assert_eq!(((a.clone() & c.clone() & b.clone() & c.clone()) | (b.clone() & c.clone()) | (a.clone() & c.clone() & b.clone())).simpl().to_string(), "b & c");

    assert_eq!((a.clone() & (a.clone() | b.clone())).simpl().to_string(), "a");
    assert_eq!(((a.clone() | b.clone()) & (a.clone() | b.clone())).simpl().to_string(), "a | b");
    assert_eq!(((a.clone() | b.clone()) & (a.clone() | b.clone() | c.clone())).simpl().to_string(), "a | b");
    assert_eq!(((a.clone() | c.clone() | b.clone() | c.clone()) & (a.clone() | b.clone())).simpl().to_string(), "a | b");
    assert_eq!(((a.clone() | c.clone() | b.clone() | c.clone()) & b.clone() & (a.clone() | b.clone())).simpl().to_string(), "b");
    assert_eq!(((a.clone() | c.clone() | b.clone() | c.clone()) & b.clone() & (a.clone() | d.clone() | b.clone())).simpl().to_string(), "b");
    assert_eq!(((a.clone() | c.clone() | b.clone() | c.clone()) & (b.clone() | c.clone()) & (a.clone() | d.clone() | b.clone())).simpl().to_string(), "(b | c) & (a | b | d)");
    assert_eq!(((a.clone() | c.clone() | b.clone() | c.clone()) & (b.clone() | c.clone()) & (a.clone() | c.clone() | b.clone())).simpl().to_string(), "b | c");

    assert_eq!((e.clone() | !e.clone()).simpl().to_string(), "true");
    assert_eq!((!!e.clone() | !e.clone()).simpl().to_string(), "true");
    assert_eq!((!!e.clone() | !!!e.clone()).simpl().to_string(), "true");

    assert_eq!((e.clone() & !e.clone()).simpl().to_string(), "false");
    assert_eq!((!!e.clone() & !e.clone()).simpl().to_string(), "false");
    assert_eq!((!!e.clone() & !!!e.clone()).simpl().to_string(), "false");
}
