use std::{collections::HashMap, vec};

#[derive(Clone, PartialEq)]
struct Node {
    name: &'static str,
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
        Node { name }
    }
}

impl Link {
    pub fn new(a: Node, b: Node, weight: i32) -> Self {
        if a.name < b.name {
            Link { a, b, weight }
        } else {
            Link { a: b, b: a, weight }
        }
    }
}

impl Path {
    pub fn new(nodes: Vec<Node>, weight: i32) -> Self {
        Path { nodes, weight }
    }
}

fn create_nodes() -> HashMap<&'static str, Node> {
    vec!["a", "b", "c", "d", "e", "f"]
        .into_iter()
        .map(|x| (x, Node::new(x)))
        .collect()
}

fn find_node_by_name<'a>(nodes: &'a Vec<Node>, name: &str) -> &'a Node {
    nodes.iter().find(|x| x.name == name).unwrap()
}

fn find_links_by_node<'a>(links: &'a Vec<Link>, node: &Node) -> Vec<&'a Link> {
    let node = node.clone();
    links
        .iter()
        .filter(|x| x.a == node || x.b == node)
        .collect()
}

fn find_link_from_nodes<'a>(links: &'a Vec<Link>, a: &Node, b: &Node) -> &'a Link {
    let a = a.clone();
    let b = b.clone();
    links
        .iter()
        .find(|x| (x.a == a && x.b == b) || (x.a == b && x.b == a))
        .unwrap()
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
            find_node_by_name(nodes, a).clone(),
            find_node_by_name(nodes, b).clone(),
            weight,
        )
    })
    .collect()
}

fn main() {
    let nodes: Vec<Node> = create_nodes().values().map(|x| x.clone()).collect();
    let start = find_node_by_name(&nodes, "a");
    let goal = find_node_by_name(&nodes, "e");
    let links = create_links(&nodes);
}

#[test]
fn creates_and_finds() {
    let nodes: Vec<Node> = create_nodes().values().map(|x| x.clone()).collect();
    let b = find_node_by_name(&nodes, "b");
    assert_eq!("b", b.name);

    let links = create_links(&nodes);
    let b_links = find_links_by_node(&links, b);
    assert_eq!(3, b_links.len());
    for iter in b_links.iter() {
        assert!(iter.a.name == "b" || iter.b.name == "b")
    }
}
