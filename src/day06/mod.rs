#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use vec_tree::{Index, VecTree};

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::Path,
};

const DAY: usize = 6;

type INPUT = (VecTree<String>, Index, Index);

pub fn stage1(tree: &INPUT) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage1");

    tree.0
        .descendants_with_depth(tree.0.get_root_index().unwrap())
        .map(|n| n.1 as u32)
        .sum::<u32>() as usize
}

pub fn stage2(input: &INPUT) -> usize {
    #[cfg(feature = "profiler")]
    profile_scope!("stage2");
    let (tree, santa_node, you_node) = input;
    let mut ans = tree.ancestors(*you_node);
    ans.next().unwrap();
    let yous: HashSet<Index> = ans.collect();
    let mut ans = tree.ancestors(*santa_node);
    ans.next().unwrap();
    let santas: HashSet<Index> = ans.collect();
    let int = santas.intersection(&yous).cloned().collect();
    let all = santas.difference(&int).chain(yous.difference(&int));
    log::info!("{:?}", all.count());
    // all.court()
    0
}

fn build_tree(map: HashMap<String, Vec<String>>) -> INPUT {
    let mut santa_node = None;
    let mut you_node = None;
    let mut tree = VecTree::new();
    let mut nodes = VecDeque::new();
    let root_node = tree.insert_root("COM".to_string());
    nodes.push_back(("COM".to_string(), root_node));

    while !nodes.is_empty() {
        let node = nodes.pop_front().unwrap();
        // log::info!("Examining: {:?}", node);
        let children = map.get(&node.0);
        if children.is_none() {
            continue;
        }
        let children = children.unwrap();
        // log::info!("Children: {:?}", children);
        for child in children.iter() {
            let child_node = tree.insert(child.to_string(), node.1);
            if child == "SAN" {
                santa_node = Some(child_node);
            } else if child == "YOU" {
                you_node = Some(child_node);
            }
            nodes.push_back((child.to_string(), child_node));
        }
    }
    (tree, santa_node.unwrap(), you_node.unwrap())
}

fn parse_input(input: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut children = HashMap::new();
    children.insert("COM".to_string(), vec![]);

    for line in input {
        let mut s = line.split(")");
        let a: String = s.next().unwrap().to_string();
        let b: String = s.next().unwrap().to_string();
        if !children.contains_key(&a) {
            children.insert(a, vec![b]);
        } else {
            children.get_mut(&a).unwrap().push(b);
        }
    }

    children
}

pub fn run_day() {
    #[cfg(feature = "profiler")]
    profile_scope!("day1");
    let input_path = Path::new("src")
        .join(format!("day{:02}", DAY))
        .join("input");
    log::debug!("Opening file {:?}", input_path);
    let s = fs::read_to_string(input_path).expect("Some input needs to exist");
    let input = parse_input(s.lines().map(&str::to_string).collect());
    let tree = build_tree(input);
    let s1 = stage1(&tree);
    log::info!("{:?}", s1);
    let s2 = stage2(&tree);
    log::info!("{:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;
}
