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
    Mul {
        #[serde(rename = "*")]
        mul: (Box<Node>, Box<Node>),
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
        call: String,
        pars: HashMap<String, Node>,
    },
    Print {
        print: Box<Node>,
    },
    Block(Vec<Node>),
    Eval {
        eval: Box<Node>,
    },
    Undefined,
}

use Node::*;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("read");

    let node = serde_json::from_str(&input).expect("parse");
    match Runner::new().eval(node) {
        Num(n) => println!("{n}"),
        _ => {}
    }
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
                (Num(a), Num(b)) => Num(a + b),
                _ => Undefined,
            },
            Mul { mul: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a * b),
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
                let func = self.get_variable(call);
                self.vars.push(pars);
                let result = self.eval(func);
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
            Eval { eval } => self.eval(*eval),
        }
    }
}
