use serde::Deserialize;
use std::{
    collections::{BTreeMap as Map, HashMap},
    io::Read,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Node {
    Num(i32),
    Str {
        str: String,
    },
    Var(String),
    Let {
        #[serde(rename = "let")]
        l: Map<String, Self>,
    },
    Print {
        print: Box<Self>,
    },
    Sum {
        #[serde(rename = "+")]
        sum: (Box<Self>, Box<Self>),
    },
    Sub {
        #[serde(rename = "-")]
        sub: (Box<Self>, Box<Self>),
    },
    Mul {
        #[serde(rename = "*")]
        mul: (Box<Self>, Box<Self>),
    },
    Div {
        #[serde(rename = "/")]
        div: (Box<Self>, Box<Self>),
    },
    Eq {
        #[serde(rename = "==")]
        eq: (Box<Self>, Box<Self>),
    },
    Fn {
        #[serde(rename = "fn")]
        f: Box<Self>,
    },
    Call {
        call: Box<Self>,
        #[serde(default)]
        pars: Map<String, Self>,
    },
    If {
        #[serde(rename = "if")]
        i: Box<Self>,
        #[serde(rename = "then")]
        t: Box<Self>,
        #[serde(rename = "else")]
        e: Box<Self>,
    },
    Block(Vec<Self>),
    Undefined,
}

use Node::*;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("read");

    let node = serde_json::from_str(&input).expect("parse");
    let _ = Runner::new().eval(node);
}

struct Runner {
    vars: Vec<HashMap<String, Node>>,
}

impl Runner {
    fn new() -> Self {
        Self {
            vars: vec![HashMap::default()],
        }
    }

    fn get_variable(&self, name: String) -> Node {
        self.vars
            .iter()
            .find_map(|map| map.get(&name))
            .cloned()
            .unwrap_or(Undefined)
    }

    fn eval(&mut self, node: Node) -> Node {
        match node {
            Num(_) | Str { .. } | Undefined => node,
            Var(name) => self.get_variable(name),
            Let { l } => {
                for (name, node) in l {
                    let result = self.eval(node);
                    self.vars.last_mut().expect("last").insert(name, result);
                }

                Undefined
            }
            Print { print } => {
                match self.eval(*print) {
                    Num(n) => println!("{n}"),
                    Str { str } => println!("{str}"),
                    _ => println!("undefined"),
                }

                Undefined
            }
            Sum { sum: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a.wrapping_add(b)),
                _ => Undefined,
            },
            Sub { sub: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a.wrapping_sub(b)),
                _ => Undefined,
            },
            Mul { mul: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a.wrapping_mul(b)),
                _ => Undefined,
            },
            Div { div: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(_), Num(0)) => Undefined,
                (Num(a), Num(b)) => Num(a.wrapping_div(b)),
                _ => Undefined,
            },
            Eq { eq: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) if a == b => Num(1),
                _ => Num(0),
            },
            Fn { f } => *f,
            Call { call, pars } => {
                self.vars.push(HashMap::default());
                let _ = self.eval(Let { l: pars });
                let result = match self.eval(*call) {
                    Fn { f } => self.eval(*f),
                    node => self.eval(node),
                };
                self.vars.pop();
                result
            }
            If { i, t, e } => match self.eval(*i) {
                Num(0) | Undefined => self.eval(*e),
                Num(_) => self.eval(*t),
                Str { str } if str.is_empty() => self.eval(*e),
                Str { .. } => self.eval(*t),
                _ => Undefined,
            },
            Block(nodes) => nodes
                .into_iter()
                .map(|node| self.eval(node))
                .last()
                .unwrap_or(Undefined),
        }
    }
}
