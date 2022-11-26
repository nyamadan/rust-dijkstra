use std::vec;

#[derive(Clone, PartialEq)]
struct Node {
    name: &'static str,
    from: Option<&'static str>,
    weight: Option<i32>,
    start: bool,
    goal: bool,
    done: bool,
}

#[derive(Clone, PartialEq)]
struct Link {
    a_name: &'static str,
    b_name: &'static str,
    weight: i32,
}

impl Node {
    pub fn new(name: &'static str) -> Self {
        Node {
            name,
            from: None,
            weight: None,
            done: false,
            start: false,
            goal: false,
        }
    }
}

impl Link {
    pub fn new(a_name: &'static str, b_name: &'static str, weight: i32) -> Self {
        Link {
            a_name,
            b_name,
            weight,
        }
    }

    pub fn opposite_node_name(&self, name: &'static str) -> Option<&'static str> {
        if self.a_name == name {
            return Some(&self.b_name);
        }

        if self.b_name == name {
            return Some(&self.a_name);
        }

        return None;
    }

    pub fn contain_node_name(&self, name: &'static str) -> bool {
        name == self.a_name || name == self.b_name
    }
}

fn create_nodes() -> Vec<Node> {
    vec!["a", "b", "c", "d", "e", "f"]
        .iter()
        .map(|&x| Node::new(x))
        .collect()
}

fn mark_node_as_fix(mut nodes: Vec<Node>, name: &str) -> Vec<Node> {
    let index = nodes.iter().position(|x| x.name == name).unwrap();
    let mut node = nodes.get(index).unwrap().clone();
    node.done = true;
    nodes.push(node);
    nodes.swap_remove(index);
    nodes
}

fn mark_node_as_start(mut nodes: Vec<Node>, name: &str) -> Vec<Node> {
    let index = nodes.iter().position(|x| x.name == name).unwrap();
    let mut node = nodes.get(index).unwrap().clone();
    node.start = true;
    nodes.push(node);
    nodes.swap_remove(index);
    nodes
}

fn mark_node_as_goal(mut nodes: Vec<Node>, name: &str) -> Vec<Node> {
    let index = nodes.iter().position(|x| x.name == name).unwrap();
    let mut node = nodes.get(index).unwrap().clone();
    node.goal = true;
    nodes.push(node);
    nodes.swap_remove(index);
    nodes
}

fn swap_node(mut nodes: Vec<Node>, index: usize, weight: i32, from: &'static str) -> Vec<Node> {
    let mut node = nodes.get(index).unwrap().clone();
    node.weight = Some(weight);
    node.from = Some(from);
    nodes.push(node);
    nodes.swap_remove(index);
    nodes
}

fn update_nodes(links: &Vec<Link>, mut nodes: Vec<Node>) -> Vec<Node> {
    let from = if nodes.iter().find(|x| x.start).unwrap().done {
        nodes
            .iter()
            .filter(|x| !x.done && x.weight.is_some())
            .min_by_key(|x| x.weight)
    } else {
        nodes.iter().find(|x| x.start)
    }
    .unwrap();
    let from_name = from.name;
    let from_weight = from.weight;
    let connected: Vec<_> = links
        .iter()
        .filter(|&x| x.contain_node_name(from_name))
        .collect();
    nodes = mark_node_as_fix(nodes, from_name);
    for link in connected.into_iter() {
        let to_name = link.opposite_node_name(from_name).unwrap();
        let to_index = nodes.iter().position(|x| x.name == to_name).unwrap();
        let to = nodes.get(to_index).unwrap();
        let to_fix = to.done;
        if to_fix {
            continue;
        }

        if let Some(from_weight) = from_weight {
            let new_weight = from_weight + link.weight;
            if let Some(to_weight) = to.weight {
                if new_weight < to_weight {
                    nodes = swap_node(nodes, to_index, new_weight, from_name);
                }
            } else {
                nodes = swap_node(nodes, to_index, new_weight, from_name);
            }
        } else {
            let new_weight = link.weight;
            if let Some(to_weight) = to.weight {
                if new_weight < to_weight {
                    nodes = swap_node(nodes, to_index, new_weight, from_name);
                }
            } else {
                nodes = swap_node(nodes, to_index, new_weight, from_name);
            }
        }
    }
    return nodes;
}

fn create_links() -> Vec<Link> {
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
    .map(|(a_name, b_name, weight)| Link::new(a_name, b_name, weight))
    .collect()
}

fn main() {
    let start = "a";
    let goal = "e";

    let mut nodes = mark_node_as_goal(mark_node_as_start(create_nodes(), start), goal);
    let links = create_links();
    while !nodes.iter().find(|x| x.name == goal).unwrap().done {
        nodes = update_nodes(&links, nodes);
    }

    let mut node = nodes.iter().find(|x| x.name == goal).unwrap();
    let mut results = vec![format!("{}({})", node.name, node.weight.unwrap())];
    while node.from.is_some() {
        node = nodes.iter().find(|x| x.name == node.from.unwrap()).unwrap();
        results.push(format!(
            "{}({})",
            node.name,
            if node.weight.is_some() {
                node.weight.unwrap()
            } else {
                0
            }
        ));
    }
    results.reverse();
    println!("Result: {}", results.join(" → "));
}
