use crate::Expr::*;
use std::collections::HashMap;
use std::fmt::Display;

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

impl Rule {
    #[allow(unused)]
    fn apply_all(&self, expr: Expr) -> Expr {
        todo!()
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
                //Check if variables don't have the same name
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

fn main() {
    /*
        pair(a,b)
        swap(pair(a,b)) = pair(b,a)
    */
    use Expr::*;
    let _swap = Rule {
        head: Fun(
            "swap".to_string(),
            vec![Fun(
                "pair".to_string(),
                vec![Sym("a".to_string()), Sym("b".to_string())],
            )],
        ),
        body: Fun(
            "pair".to_string(),
            vec![Sym("b".to_string()), Sym("a".to_string())],
        ),
    };

    let expr = Fun(
        "foo".to_string(),
        vec![
            Fun(
                "swap".to_string(),
                vec![Fun(
                    "pair".to_string(),
                    vec![
                        Fun("f".to_string(), vec![Sym("a".to_string())]),
                        Fun("g".to_string(), vec![Sym("b".to_string())]),
                    ],
                )],
            ),
            Fun(
                "swap".to_string(),
                vec![Fun(
                    "pair".to_string(),
                    vec![
                        Fun("q".to_string(), vec![Sym("c".to_string())]),
                        Fun("z".to_string(), vec![Sym("d".to_string())]),
                    ],
                )],
            ),
        ],
    );

    println!("Rule:         {_swap}");
    println!("Expression:   {expr}");
    println!("Expression':  {}", _swap.apply_all(expr));
}
