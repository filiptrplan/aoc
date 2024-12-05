use std::{
    collections::{HashMap, HashSet}, hash::Hash, io
};

#[derive(Debug, Clone)]
struct Node {
    incoming: HashSet<i32>,
    outgoing: HashSet<i32>,
}

type Rule = (i32, i32);

fn topo_sort(mut nodes: HashMap<i32, Node>) -> Vec<i32> {
    let mut sorted = Vec::new();
    while nodes.len() > 0 {
        let mut key_to_remove = Vec::new();
        nodes.iter().for_each(|(key, value)| {
            if value.incoming.len() == 0 {
                key_to_remove.push(key.clone());
            }
        });
        for key in key_to_remove {
            sorted.push(key.clone());
            if let Some(node) = nodes.get(&key) {
                let outgoing: Vec<i32> = node.outgoing.iter().cloned().collect();
                for out in outgoing {
                    if let Some(node_in) = nodes.get_mut(&out) {
                        node_in.incoming.remove(&key);
                    }
                }
            }
            nodes.remove(&key);
        }
    }

    sorted
}

fn construct_nodes(rules: &Vec<Rule>, update: &Vec<i32>) -> HashMap<i32, Node> {
    let mut nodes: HashMap<i32, Node> = HashMap::new();
    let filtered_rules = rules.iter().filter(|(n1, n2)| update.contains(n1) && update.contains(n2));
    for rule in filtered_rules {
        if !nodes.contains_key(&rule.0) {
            nodes.insert(
            rule.0,
            Node {
                incoming: HashSet::new(),
                outgoing: HashSet::new(),
            },
            );
        }
        if !nodes.contains_key(&rule.1) {
            nodes.insert(
            rule.1,
            Node {
                incoming: HashSet::new(),
                outgoing: HashSet::new(),
            },
            );
        }
        if let Some(node) = nodes.get_mut(&rule.0) {
            node.outgoing.insert(rule.1);
        }
        if let Some(node) = nodes.get_mut(&rule.1) {
            node.incoming.insert(rule.0);
        }
    }
    nodes
}

fn process_update(sorted: &Vec<i32>, update: Vec<i32>) -> i32 {
    let mut i = 0;
    for x in update.iter() {
        while *x != sorted[i] {
            if i == sorted.len() - 1 {
                return sorted[sorted.len() / 2];
            }
            i += 1;
        }
    }

    return 0;
}

pub fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut rules: Vec<Rule> = Vec::new();

    loop {
        buf.clear();
        let _ = stdin.read_line(&mut buf);
        if buf == "\n" {
            break;
        }
        let buf_trim = buf.trim_end();
        let rule: Vec<i32> = buf_trim
            .split_terminator("|")
            .map(|x| x.parse::<i32>().expect("Failed to parse int"))
            .collect();

        rules.push((rule[0].clone(), rule[1].clone()));
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();
    loop {
        buf.clear();
        let res = stdin.read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        updates.push(
            buf.trim_end()
                .split_terminator(",")
                .map(|x| x.parse().expect("Failed to parse"))
                .collect(),
        );
    }

    println!(
        "{}",
        updates
            .into_iter()
            .map(|x| process_update(&topo_sort(construct_nodes(&rules, &x)), x))
            .fold(0, |a, b| a + b)
    );
}
