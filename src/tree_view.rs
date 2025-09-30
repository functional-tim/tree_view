#![feature(allocator_api)]

use std::alloc::{Allocator, Global};
use std::collections::{BTreeMap};
use std::fmt::Display;

const free: &str = "│   ";
const node: &str = "├── ";
const end: &str = "└── ";
const empty: &str = "    ";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Node {
    pub node: String,
    pub children: Vec<Node>,
}

pub trait ToNode {
    fn to_node(&self) -> Node;
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Graph<T>
where
    T: ToNode
{
    pub original: T,
    pub root: Node,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TreeView<T>
where
    T: ToNode
{
    pub graph: Graph<T>,
//    view: String, // TODO Caching
//    tree_changed: bool, // TODO Caching
}

impl Node {
    pub fn new() -> Self {
        Node {
            node: String::from(""),
            children: Vec::new(),
        }
    }

    pub fn from(n: Node) -> Self {
        Node {
            node: n.node,
            children: n.children,
        }
    }
    
    pub fn insert(mut self, n: Node) {
        self.children.push(n);
    }

    pub fn print_node(self, depth: u32, pre: &str) -> String {
        let node_var: &str = self.node.as_str();
        let mut output: String = format!("{pre}{node_var}\n");
        let n = self.children.len();

        for (i, c) in self.children.into_iter().enumerate() {
            if i < n - 1 && depth == 0 {
                output = output + &c.print_node(depth + 1, &format!("{pre}{node}"));
            } else if i < n - 1 && depth > 0 {
                //output = output + &c.print_node(depth + 1, gen_prefix(depth));
            } else {
                output = output + &c.print_node(depth + 1, &format!("{pre}{end}"));
            }
        }

        output
    }
}

impl<T> Graph<T>
where
    T: Clone + Ord + ToNode,
{
    pub fn new(t: T) -> Self {
        Graph {
            original: t.clone(),
            root: t.to_node(),
        }
    }

    pub fn print(self) -> String {
        self.root.print_node(0, "")
    }
}

impl<T> TreeView<T>
where
    T: Clone + Ord + ToNode,
{
    pub fn new(t: T) -> Self {
        TreeView {
            graph: Graph::new(t),
        }
    }

    pub fn print(self) -> String {
        self.graph.print()
    }
}

