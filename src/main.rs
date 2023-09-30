use {
    serde::Deserialize,
    std::{
        collections::{BTreeMap as Map, HashMap},
        io,
    },
};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(untagged)]
enum Node {
    #[default]
    Nil,
    Num(i64),
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
}

#[allow(clippy::enum_glob_use)]
use Node::*;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("read");
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

    fn get_variable(&self, name: &str) -> Node {
        self.vars
            .iter()
            .find_map(|map| map.get(name))
            .cloned()
            .unwrap_or_default()
    }

    fn eval(&mut self, node: Node) -> Node {
        match node {
            Num(_) | Str { .. } | Nil => node,
            Var(name) => self.get_variable(&name),
            Let { l } => {
                for (name, node) in l {
                    let result = self.eval(node);
                    self.vars.last_mut().expect("last").insert(name, result);
                }

                Nil
            }
            Print { print } => {
                match self.eval(*print) {
                    Num(n) => println!("{n}"),
                    Str { str } => println!("{str}"),
                    _ => println!("nil"),
                }

                Nil
            }
            Sum { sum: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a.wrapping_add(b)),
                _ => Nil,
            },
            Sub { sub: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a.wrapping_sub(b)),
                _ => Nil,
            },
            Mul { mul: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(a), Num(b)) => Num(a.wrapping_mul(b)),
                _ => Nil,
            },
            Div { div: (a, b) } => match (self.eval(*a), self.eval(*b)) {
                (Num(_), Num(0)) => Nil,
                (Num(a), Num(b)) => Num(a.wrapping_div(b)),
                _ => Nil,
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
                Num(0) | Nil => self.eval(*e),
                Str { str } if str.is_empty() => self.eval(*e),
                Num(_) | Str { .. } => self.eval(*t),
                _ => Nil,
            },
            Block(nodes) => nodes
                .into_iter()
                .map(|node| self.eval(node))
                .last()
                .unwrap_or_default(),
        }
    }
}
