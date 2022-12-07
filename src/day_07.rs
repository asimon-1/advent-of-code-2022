use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_FILE: Regex = Regex::new(r"(?P<size>\d+) (?P<name>.*)").unwrap();
    static ref RE_DIR: Regex = Regex::new(r"dir (?P<name>.*)").unwrap();
    static ref RE_CD: Regex = Regex::new(r"\$ cd (?P<name>.*)").unwrap();
    static ref RE_LS: Regex = Regex::new(r"\$ ls").unwrap();
}

struct Node {
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    size: u32,
}

impl Node {
    fn is_dir(&self) -> bool {
        !self.children.is_empty() || self.size == 0
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Tree {
        Tree { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: Node) {
        let i = self.nodes.len();
        if let Some(parent_index) = node.parent {
            self.nodes[parent_index].children.push(i);
        }
        self.nodes.push(node);
    }
}

fn execute_command<'a>(s: &str, mut tree: Tree, mut cwd_index: usize) -> (Tree, usize) {
    if RE_LS.is_match(s) {
        // No action needed
    } else if RE_CD.is_match(s) {
        let captures = RE_CD.captures(s).unwrap();
        let name = captures.name("name").unwrap().as_str();
        cwd_index = match name {
            "/" => 0, // Go to root directory, always the first node
            ".." => tree.nodes[cwd_index].parent.unwrap(), // Go to the parent
            _ => {
                // Go to a named child directory
                *tree.nodes[cwd_index]
                    .children
                    .iter()
                    .find(|index| tree.nodes[**index].name == name && tree.nodes[**index].is_dir())
                    .unwrap()
            }
        };
    } else if RE_DIR.is_match(s) {
        // Found a directory, add it to the list and to the cwd's children
        let captures = RE_DIR.captures(s).unwrap();
        tree.add_node(Node {
            name: captures.name("name").unwrap().as_str().to_owned(),
            parent: Some(cwd_index),
            children: Vec::new(),
            size: 0,
        });
    } else if RE_FILE.is_match(s) {
        // Found a file, add it to the list and to the cwd's children
        let captures = RE_FILE.captures(s).unwrap();
        tree.add_node(Node {
            name: captures.name("name").unwrap().as_str().to_owned(),
            parent: Some(cwd_index),
            children: Vec::new(),
            size: captures
                .name("size")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap(),
        });
    } else {
        panic!("Unsupported command: {}", s);
    }
    (tree, cwd_index)
}

fn get_dir_size(tree: &Tree, ind: usize) -> u32 {
    tree.nodes[ind].size
        + tree.nodes[ind]
            .children
            .iter()
            .map(|e| get_dir_size(tree, *e))
            .sum::<u32>()
}

fn parse_input(input: &str) -> Tree {
    let mut tree = Tree::new();
    tree.add_node(Node {
        name: "/".to_owned(),
        parent: None,
        children: Vec::new(),
        size: 0,
    });
    let mut cwd_index: usize = 0;
    for line in input.lines() {
        (tree, cwd_index) = execute_command(line, tree, cwd_index);
    }
    tree
}

fn part_a(input: &str) -> u32 {
    let tree = parse_input(input);
    tree.nodes
        .iter()
        .enumerate()
        .map(|(index, x)| {
            if x.is_dir() {
                get_dir_size(&tree, index)
            } else {
                0
            }
        })
        .filter(|x| x <= &100_000)
        .sum()
}

fn part_b(input: &str) -> u32 {
    let tree = parse_input(input);
    let total_space: u32 = 70_000_000;
    let current_unused_space: u32 = total_space - get_dir_size(&tree, 0);
    let required_unused_space: u32 = 30_000_000;
    let min_dir_size: u32 = required_unused_space - current_unused_space;
    tree.nodes
        .iter()
        .enumerate()
        .map(|(index, x)| {
            if x.is_dir() {
                get_dir_size(&tree, index)
            } else {
                0
            }
        })
        .filter(|x| x >= &min_dir_size)
        .min()
        .unwrap()
}

pub use crate::boilerplate;
boilerplate!(7, 95437, 24933642, u32);
