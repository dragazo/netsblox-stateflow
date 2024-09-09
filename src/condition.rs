use core::ops::{BitAnd, BitOr, Not};
use core::fmt;

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::collections::BTreeSet;

use netsblox_ast::CompactString;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawCondition {
    Const(bool),
    Atom(CompactString),
    Not(Box<RawCondition>),
    And(Box<RawCondition>, Box<RawCondition>),
    Or(Box<RawCondition>, Box<RawCondition>),
}

impl fmt::Display for RawCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawCondition::Const(x) => write!(f, "{x}"),
            RawCondition::Atom(x) => write!(f, "{x}"),
            RawCondition::And(a, b) => {
                fn single(f: &mut fmt::Formatter<'_>, v: &RawCondition) -> fmt::Result {
                    match v {
                        RawCondition::Or(_, _) => write!(f, "({v})"),
                        _ => write!(f, "{v}"),
                    }
                }
                single(f, a)?;
                write!(f, " & ")?;
                single(f, b)
            }
            RawCondition::Or(a, b) => {
                fn single(f: &mut fmt::Formatter<'_>, v: &RawCondition) -> fmt::Result {
                    match v {
                        RawCondition::And(_, _) => write!(f, "({v})"),
                        _ => write!(f, "{v}"),
                    }
                }
                single(f, a)?;
                write!(f, " | ")?;
                single(f, b)
            }
            RawCondition::Not(x) => {
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

impl BitAnd for RawCondition {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        RawCondition::And(Box::new(self), Box::new(rhs))
    }
}
impl BitOr for RawCondition {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        RawCondition::Or(Box::new(self), Box::new(rhs))
    }
}
impl Not for RawCondition {
    type Output = Self;
    fn not(self) -> Self::Output {
        RawCondition::Not(Box::new(self))
    }
}

#[test]
fn test_condition() {
    let a = RawCondition::Atom("a".into());
    let b = RawCondition::Atom("b".into());
    let c = RawCondition::Atom("c".into());
    let d = RawCondition::Atom("d".into());
    let e = RawCondition::Atom("x < 10".into());
    let f = RawCondition::Atom("y == x + 10".into());

    let bt = RawCondition::Const(true);
    let bf = RawCondition::Const(false);

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

    assert_eq!((!bt.clone()).to_string(), "~true");
    assert_eq!((!!bt.clone()).to_string(), "~(~true)");
    assert_eq!((!!!bt.clone()).to_string(), "~(~(~true))");
    assert_eq!((!!!!bt.clone()).to_string(), "~(~(~(~true)))");

    assert_eq!((!bf.clone()).to_string(), "~false");
    assert_eq!((!!bf.clone()).to_string(), "~(~false)");
    assert_eq!((!!!bf.clone()).to_string(), "~(~(~false))");
    assert_eq!((!!!!bf.clone()).to_string(), "~(~(~(~false)))");
}

impl RawCondition {
    fn visit_and<F: FnMut(&RawCondition)>(&self, f: &mut F) {
        match self {
            RawCondition::And(x, y) => {
                x.visit_and(f);
                y.visit_and(f);
            }
            x => f(x),
        }
    }
    fn visit_or<F: FnMut(&RawCondition)>(&self, f: &mut F) {
        match self {
            RawCondition::Or(x, y) => {
                x.visit_or(f);
                y.visit_or(f);
            }
            x => f(x),
        }
    }
    fn simpl(&self) -> Self {
        macro_rules! subset_simpl {
            ($terms:ident : $kind:ident : $visitor:ident) => {{
                let groups = $terms.iter().filter(|t| if let RawCondition::$kind(_, _) = t { true } else { false }).map(|t| {
                    let mut sub_terms: BTreeSet<RawCondition> = Default::default();
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
            RawCondition::And(_, _) => {
                let mut terms: BTreeSet<RawCondition> = Default::default();
                self.visit_and(&mut |x| { terms.insert(x.simpl()); });

                subset_simpl!(terms : Or : visit_or);
                terms.remove(&RawCondition::Const(true));
                if terms.contains(&RawCondition::Const(false)) || terms.iter().any(|t| if let RawCondition::Not(t) = t { terms.contains(t) } else { false }) {
                    return RawCondition::Const(false);
                }

                terms.into_iter().reduce(|a, b| a & b).unwrap_or(RawCondition::Const(true))
            }
            RawCondition::Or(_, _) => {
                let mut terms: BTreeSet<RawCondition> = Default::default();
                self.visit_or(&mut |x| { terms.insert(x.simpl()); });

                subset_simpl!(terms : And : visit_and);
                terms.remove(&RawCondition::Const(false));
                if terms.contains(&RawCondition::Const(true)) || terms.iter().any(|t| if let RawCondition::Not(t) = t { terms.contains(t) } else { false }) {
                    return RawCondition::Const(true);
                }

                terms.into_iter().reduce(|a, b| a | b).unwrap_or(RawCondition::Const(false))
            }
            RawCondition::Not(x) => match &**x {
                RawCondition::Const(x) => RawCondition::Const(!x),
                RawCondition::Not(x) => x.simpl(),
                x => RawCondition::Not(Box::new(x.simpl())),
            }
            x => x.clone(),
        }
    }
}

#[test]
fn test_simpl() {
    let a = RawCondition::Atom("a".into());
    let b = RawCondition::Atom("b".into());
    let c = RawCondition::Atom("c".into());
    let d = RawCondition::Atom("d".into());
    let e = RawCondition::Atom("x < 10".into());
    let f = RawCondition::Atom("y == x + 10".into());

    let bt = RawCondition::Const(true);
    let bf = RawCondition::Const(false);

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

    assert_eq!((!bt.clone()).simpl().to_string(), "false");
    assert_eq!((!!bt.clone()).simpl().to_string(), "true");
    assert_eq!((!!!bt.clone()).simpl().to_string(), "false");
    assert_eq!((!!!!bt.clone()).simpl().to_string(), "true");

    assert_eq!((!bf.clone()).simpl().to_string(), "true");
    assert_eq!((!!bf.clone()).simpl().to_string(), "false");
    assert_eq!((!!!bf.clone()).simpl().to_string(), "true");
    assert_eq!((!!!!bf.clone()).simpl().to_string(), "false");
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Condition(RawCondition);
impl Condition {
    pub fn raw(&self) -> &RawCondition {
        &self.0
    }
    pub fn atom(v: CompactString) -> Self {
        debug_assert!(v != "true" && v != "false" && v != "");
        Condition(RawCondition::Atom(v))
    }
    pub fn constant(v: bool) -> Self {
        Condition(RawCondition::Const(v))
    }
}
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl BitAnd for Condition {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let res = (self.0 & rhs.0).simpl();
        debug_assert_eq!(res, res.simpl());
        Condition(res)
    }
}
impl BitOr for Condition {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let res = (self.0 | rhs.0).simpl();
        debug_assert_eq!(res, res.simpl());
        Condition(res)
    }
}
impl Not for Condition {
    type Output = Self;
    fn not(self) -> Self::Output {
        let res = (!self.0).simpl();
        debug_assert_eq!(res, res.simpl());
        Condition(res)
    }
}
