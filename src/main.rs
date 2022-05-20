use serde::Deserialize;
use std::{collections::HashMap, io::Read};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Node {
    Num(i32),
    Var(String),
    Sum {
        #[serde(rename = "+")]
        sum: (Box<Node>, Box<Node>),
    },
    Sub {
        #[serde(rename = "-")]
        sub: (Box<Node>, Box<Node>),
    },
    Mul {
        #[serde(rename = "*")]
        mul: (Box<Node>, Box<Node>),
    },
    Div {
        #[serde(rename = "/")]
        div: (Box<Node>, Box<Node>),
    },
    Assign {
        #[serde(rename = "=")]
        assign: (String, Box<Node>),
    },
    Fn {
        #[serde(rename = "fn")]
        func: Box<Node>,
    },
    Call {
        call: Box<Node>,
        #[serde(default)]
        pars: HashMap<String, Node>,
    },
    Print {
        print: Box<Node>,
    },
    Block(Vec<Node>),
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
            Num(_) | Undefined => node,
            Var(name) => self.get_variable(name),
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
            Assign {
                assign: (name, rhs),
            } => {
                let result = self.eval(*rhs);
                self.vars.last_mut().unwrap().insert(name, result);
                Undefined
            }
            Fn { func } => *func,
            Call { call, pars } => {
                self.vars.push(pars);
                let result = match self.eval(*call) {
                    Fn { func } => self.eval(*func),
                    node => self.eval(node),
                };
                self.vars.pop();
                result
            }
            Print { print } => {
                match self.eval(*print) {
                    Num(n) => println!("{n}"),
                    _ => println!("undefined"),
                };

                Undefined
            }
            Block(nodes) => nodes
                .into_iter()
                .map(|node| self.eval(node))
                .last()
                .unwrap_or(Undefined),
        }
    }
}
