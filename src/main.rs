/*
 * main.rs - Console program to test tree_view.
 *
 * (C) 2020 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

#![feature(allocator_api)]

mod tree_view;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TestMap {
    pub key: String,
    pub value: Vec<TestMap>,
}

impl tree_view::ToNode for TestMap {
    fn to_node(&self) -> tree_view::Node {
        tree_view::Node {
            node: self.key.clone(),
            children: self.value.iter().map(|v| v.to_node()).collect(),
        }
    }
}

fn main() {
    let mut tree0: TestMap = TestMap {
        key: String::from("Root"),
        value: Vec::new(),
    };
    tree0.value.push(TestMap {
        key: String::from("Leaf1"),
        value: Vec::new(),
    });
    tree0.value.push(TestMap {
        key: String::from("Node1"),
        value: Vec::from([
            TestMap {
                key: String::from("Leaf2"),
                value: Vec::new(),
            },
            TestMap {
                key: String::from("Node2"),
                value: Vec::from([
                    TestMap {
                        key: String::from("Leaf3"),
                        value: Vec::new(),
                    },
                    TestMap {
                        key: String::from("Leaf4"),
                        value: Vec::new(),
                    },
                ]),
            },
        ]),
    });
    tree0.value.push(TestMap {
        key: String::from("Node3"),
        value: Vec::from([TestMap {
            key: String::from("Leaf5"),
            value: Vec::new(),
        }]),
    });
    tree0.value.push(TestMap {
        key: String::from("Node4"),
        value: Vec::from([TestMap {
            key: String::from("Leaf6"),
            value: Vec::new(),
        }]),
    });
    let tree1: TestMap = TestMap {
        key: String::from("Root"),
        value: Vec::new(),
    };
    let tree2: TestMap = TestMap {
        key: String::from("Root"),
        value: Vec::from([
            TestMap {
                key: String::from("Leaf"),
                value: Vec::new(),
            },
            TestMap {
                key: String::from("Leaf"),
                value: Vec::new(),
            },
            TestMap {
                key: String::from("Leaf"),
                value: Vec::new(),
            },
        ]),
    };

    let view0: tree_view::TreeView<TestMap> = tree_view::TreeView::new(tree0.clone());
    let view1: tree_view::TreeView<TestMap> = tree_view::TreeView::new(tree1.clone());
    let view2: tree_view::TreeView<TestMap> = tree_view::TreeView::new(tree2.clone());
    println!("{:?}\n", tree0);
    println!("{}", view0);
    println!("{}", view1.print());
    println!("{}", view2.print());
}
