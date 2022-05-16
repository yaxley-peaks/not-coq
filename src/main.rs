use crate::Expr::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Peekable;

#[allow(dead_code)]
#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Sym(String),
    Fun(String, Vec<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Sym(name) => write!(f, "{name}"),
            Expr::Fun(name, args) => {
                write!(f, "{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
                // Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    head: Expr,
    body: Expr,
}

fn substitute_bindings(bindings: &Bindings, expr: &Expr) -> Expr {
    match expr {
        Sym(name) => {
            if let Some(value) = bindings.get(name) {
                value.clone()
            } else {
                expr.clone()
            }
        }
        Fun(name, args) => {
            let new_name = match bindings.get(name) {
                Some(Sym(new_name)) => new_name,
                None => name,
                Some(_) => panic!("Invalid value. Burn in fire"),
            };
            let mut new_args = Vec::new();
            for arg in args {
                new_args.push(substitute_bindings(bindings, &arg));
            }
            Fun(new_name.clone(), new_args)
        }
    }
}

impl Rule {
    #[allow(unused)]
    fn apply_all(&self, expr: &Expr) -> Expr {
        if let Some(bindings) = pattern_match(&self.head, &expr) {
            substitute_bindings(&bindings, &self.body)
        } else {
            match expr {
                Sym(_) => expr.clone(),
                Fun(name, args) => {
                    let mut new_args = Vec::new();
                    for arg in args {
                        new_args.push(self.apply_all(arg))
                    }
                    Fun(name.clone(), new_args)
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.head, self.body)
    }
}

type Bindings = HashMap<String, Expr>;

fn pattern_match(pattern: &Expr, value: &Expr) -> Option<Bindings> {
    fn pattern_match_impl(pattern: &Expr, value: &Expr, bindings: &mut Bindings) -> bool {
        match (pattern, value) {
            (Sym(name), _) => {
                if let Some(bound_value) = bindings.get(name) {
                    if bound_value == value {
                        true
                    } else {
                        false
                    }
                } else {
                    bindings.insert(name.clone(), value.clone());
                    true
                }
            }
            (Fun(name1, args1), Fun(name2, args2)) => {
                if name1 == name2 && args1.len() == args2.len() {
                    for i in 0..args1.len() {
                        if !pattern_match_impl(&args1[i], &args2[i], bindings) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    let mut bindings = HashMap::new();
    if pattern_match_impl(pattern, value, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

#[derive(Debug)]
enum TokenKind {
    Sym,
    OpenParen,
    ClosedParen,
    Comma,
    Equals,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    text: String,
}
struct Lexer<Char: Iterator<Item = char>> {
    chars: Peekable<Char>,
}

impl<Char: Iterator<Item = char>> Lexer<Char> {
    fn from_iter(chars: Char) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }
}
impl<Char: Iterator<Item = char>> Iterator for Lexer<Char> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Constructs an `Expr::Sym`.
///
/// ```rs
/// sym!(a) = Expr::Sym("a".to_string())
/// ```
macro_rules! sym {
    ($name: ident) => {
        Sym(stringify!($name).to_string())
    };
}

/// Constructs an `Expr::Fun`
///
/// ```rs
/// fun!(f, b) = Expr::Fun("f".to_string(), vec![b])
/// ```
macro_rules! fun {
    ($name: ident) => {
        Expr::Fun(stringify!($name).to_string(), vec![])
    };
    ($name: ident, $($args: expr),*) => {
        Expr::Fun(stringify!($name).to_string(), vec![$($args),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn rule_apply_all() {
        use super::Expr::*;
        let swap = Rule {
            head: fun!(swap, fun!(pair, sym!(a), sym!(b))),
            body: fun!(pair, sym!(b), sym!(a)),
        };

        let expr = fun!(
            foo,
            fun!(swap, fun!(pair, fun!(f, sym!(a)), fun!(g, sym!(b)))),
            fun!(swap, fun!(pair, fun!(q, sym!(c)), fun!(z, sym!(d))))
        );
        let expected = fun!(
            foo,
            fun!(pair, fun!(g, sym!(b)), fun!(f, sym!(a))),
            fun!(pair, fun!(z, sym!(d)), fun!(q, sym!(c)))
        );
        println!("Rule:         {swap}");
        println!("Expression:   {expr}");
        println!("Expression':  {:?}", swap.apply_all(&expr));

        assert_eq!(swap.apply_all(&expr), expected);
    }
}
macro_rules! fun_args {
    () => {
        vec![]
    };
    ($name: ident) => {
        vec![expr!($name)]
    };
    ($name: ident, $($rest: tt)*) => {
        {
            let mut t = vec![expr!($name)];
            t.append(&mut fun_args!($($rest)*));
            t
        }
    };
    ($name: ident($($args: tt)*)) => {
        vec![expr!($name($($args)*))]
    };
    ($name: ident($($args: tt)*), $($rest: tt)*) => {
        {
            let mut t = vec![expr!($name($($args)*))];
            t.append(&mut fun_args!($($rest)*));
            t
        }
    };
}

macro_rules! expr {
    ($name: ident) => {
        Expr::Sym(stringify!($name).to_string())
    };
    ($name: ident($($args: tt)*)) => {
        Expr::Fun(stringify!($name).to_string(), fun_args!($($args)*))
    };

}

fn main() {
    /*
        pair(a,b)
        swap(pair(a,b)) = pair(b,a)
    */

    // for token in Lexer::from_iter("swap(pair(a,b)) = pair(b,a)".chars()) {
    //     println!("{:?}", token);
    // }
    println!("{}", expr!(a));
    println!("{}", expr!(g(b, a)));
    println!("{}", expr!(f(g(a))));
    println!("{}", expr!(f(g(a), k(q,l(a)))));
}
