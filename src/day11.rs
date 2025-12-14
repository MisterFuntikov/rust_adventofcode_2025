use std::collections::{HashMap, HashSet};

fn helper(
    node: &str,
    nodes: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    seen_fft: bool,
    seen_dac: bool,
    memo: &mut HashMap<(String, bool, bool), [u64; 2]>,
) -> [u64; 2] {
    if node == "out" {
        return [1, if seen_fft && seen_dac { 1 } else { 0 }];
    }
    let key = (node.to_string(), seen_fft, seen_dac);
    let mut total = [0, 0];
    if let Some(&res) = memo.get(&key) {
        return res;
    }
    visited.insert(node.to_string());
    for next in nodes.get(node).unwrap() {
        if visited.contains(next) {
            continue;
        }
        visited.insert(next.clone());
        let r = helper(
            next,
            nodes,
            visited,
            seen_fft || next == "fft",
            seen_dac || next == "dac",
            memo,
        );
        (total[0], total[1]) = (total[0] + r[0], total[1] + r[1]);
        visited.remove(next);
    }
    visited.remove(node);
    memo.insert(key, total);
    total
}

pub fn day11(path: &String) -> [u64; 2] {
    let content = std::fs::read_to_string(path).unwrap();
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited = HashSet::new();
    let mut memo = HashMap::new();

    for line in content.lines().filter(|l| !l.is_empty()) {
        let (name, rest) = line.split_once(':').unwrap();
        let next: Vec<String> = rest.trim().split_whitespace().map(String::from).collect();
        nodes.insert(name.to_string(), next);
    }

    helper("you", &nodes, &mut visited, false, false, &mut memo);
    helper("svr", &nodes, &mut visited, false, false, &mut memo);

    [
        memo.get(&("you".to_string(), false, false)).unwrap()[0],
        memo.get(&("svr".to_string(), false, false)).unwrap()[1],
    ]
}
