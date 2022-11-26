use std::{collections::HashMap, path, vec};

#[derive(Clone, PartialEq)]
struct Node {
    name: &'static str,
    weight: i32,

    start: bool,
    goal: bool,
    fix: bool,
}

#[derive(Clone, PartialEq)]
struct Link {
    a: Node,
    b: Node,
    weight: i32,
}

#[derive(Clone, PartialEq)]
struct Path {
    nodes: Vec<Node>,
    weight: i32,
}

impl Node {
    pub fn new(name: &'static str) -> Self {
        Node {
            name,
            weight: 0,
            fix: false,
            start: false,
            goal: false,
        }
    }
}

impl Link {
    pub fn new(a: Node, b: Node, weight: i32) -> Self {
        Link { a, b, weight }
    }

    pub fn opposite_node(&self, x: &Node) -> Option<&Node> {
        let x = x.clone();

        if self.a == x {
            return Some(&self.b);
        }

        if self.b == x {
            return Some(&self.a);
        }

        return None;
    }

    pub fn contain_node(&self, x: &Node) -> bool {
        let x = x.clone();
        x == self.a || x == self.b
    }
}

fn create_nodes() -> Vec<Node> {
    vec!["a", "b", "c", "d", "e", "f"]
        .iter()
        .map(|&x| Node::new(x))
        .collect()
}

fn mark_node_as_fix<'a>(mut nodes: Vec<Node>, name: &str) -> Vec<Node> {
    let mut new_nodes = Vec::<Node>::new();
    let mut ok = false;
    while !nodes.is_empty() {
        let mut node = nodes.remove(0);
        if node.name == name {
            node.fix = true;
        }
        new_nodes.push(node);
    }
    if !ok {
        panic!("Could not mark as fix!");
    }
    new_nodes
}

fn mark_node_as_start<'a>(mut nodes: Vec<Node>, name: &str) -> Vec<Node> {
    let mut new_nodes = Vec::<Node>::new();
    let mut ok = false;
    while !nodes.is_empty() {
        let mut node = nodes.remove(0);
        if node.name == name {
            node.start = true;
            node.fix = true;
            ok = true;
        }
        new_nodes.push(node);
    }
    if !ok {
        panic!("Could not mark as start!");
    }
    new_nodes
}

fn mark_node_as_goal<'a>(mut nodes: Vec<Node>, name: &str) -> Vec<Node> {
    let mut new_nodes = Vec::<Node>::new();
    let mut ok = false;
    while !nodes.is_empty() {
        let mut node = nodes.remove(0);
        if node.name == name {
            node.goal = true;
            node.fix = true;
            ok = true;
        }
        new_nodes.push(node);
    }
    if !ok {
        panic!("Could not mark as goal!");
    }
    new_nodes
}

fn find_node_by_name<'a>(nodes: &'a Vec<Node>, name: &str) -> Option<&'a Node> {
    nodes.iter().find(|&x| x.name == name)
}

fn find_links_by_nodes<'a>(links: &'a Vec<Link>, node: &'a Node) -> Vec<&'a Link> {
    links.iter().filter(|&x| x.contain_node(node)).collect()
}

fn create_links(nodes: &Vec<Node>) -> Vec<Link> {
    vec![
        ("a", "b", 5),
        ("a", "c", 4),
        ("a", "d", 2),
        ("b", "c", 2),
        ("b", "e", 6),
        ("c", "d", 3),
        ("c", "f", 2),
        ("d", "f", 6),
        ("e", "f", 4),
    ]
    .into_iter()
    .map(|(a, b, weight)| {
        Link::new(
            find_node_by_name(nodes, a).unwrap().clone(),
            find_node_by_name(nodes, b).unwrap().clone(),
            weight,
        )
    })
    .collect()
}

fn main() {
    let start = "a";
    let goal = "f";
    let nodes = mark_node_as_goal(mark_node_as_start(create_nodes(), start), goal);
    let links = create_links(&nodes);
}

#[test]
fn creates() {
    let nodes: Vec<Node> = create_nodes();
    create_links(&nodes);
}
