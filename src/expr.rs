use std::fmt;

use crate::list::List;

#[derive(Debug)]
pub enum Atom {
    Builtin(String),
    Boolean(bool),
    Char(u8),
    Number(f64),
    Str(String),
    Symbol(String),
}

#[derive(Debug)]
pub enum SExpr {
    Atom(Atom),
    SExpr(List<SExpr>),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Char(c)       => write!(f, "#{}", c),
            Atom::Number(n)     => write!(f, "{}", n),
            Atom::Str(s)        => write!(f, "\"{}\"", s),
            x@_                 => write!(f, "{}", x),
        }
    }
}
impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SExpr::Atom(a)      => write!(f, "{}", a),
            SExpr::SExpr(lst)   => {
                write!(f, "(")?;
                for node in lst.iter() {
                    node.fmt(f)?; 
                    write!(f, " ")?;
                }
                write!(f, ")")
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct() {
        let st = SExpr::Atom(Atom::Str(String::from("hello, world")));
        let one = SExpr::Atom(Atom::Number(1.));
        let two = SExpr::Atom(Atom::Number(2.));
        let three = SExpr::Atom(Atom::Number(3.));
        let lst = SExpr::SExpr(List::new().cons(three).cons(two).cons(one).cons(st));
        assert_eq!(format!("{}", lst).to_string(), 
            String::from("(\"hello, world\" 1 2 3 )"));
    }
}
