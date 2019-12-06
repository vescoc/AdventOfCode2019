#![feature(test)]
extern crate test;

use std::cmp::Eq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::ops;

#[macro_use]
extern crate lazy_static;

const SEPARATOR: char = ')';

lazy_static! {
    pub static ref DATA: Forest<&'static str> = {
        include_str!("../data.txt")
            .lines()
            .try_fold(Forest::new(), |acc, l| {
                let mut i = l.trim().split(SEPARATOR);
                match (i.next(), i.next()) {
                    (Some(parent), Some(child)) => {
                        let (acc, parent_ref) = acc.add_node(parent);
                        let (acc, child_ref) = acc.add_node(child);
                        let (acc, _) = acc.add_relationship(parent_ref, child_ref);
                        Ok(acc)
                    }
                    _ => Err(format!("Invalid separator / data: {}", l)),
                }
            })
            .expect("invalid data")
    };
    pub static ref ROOT: NodeRef = { DATA.roots().next().unwrap() };
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    parent: Option<NodeRef>,
    sibling: Option<NodeRef>,
    first_child: Option<NodeRef>,
    last_child: Option<NodeRef>,
}

impl<T: Debug> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            sibling: None,
            first_child: None,
            last_child: None,
        }
    }

    pub fn parent(&self) -> &Option<NodeRef> {
        &self.parent
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

type NodeRef = usize;

#[derive(Debug, Default)]
pub struct Forest<T>
where
    T: Eq + Hash + Debug,
{
    arena: Vec<Node<T>>,
    map: HashMap<T, NodeRef>,
}

impl<T> Forest<T>
where
    T: Eq + Hash + Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            arena: vec![],
            map: HashMap::new(),
        }
    }

    pub fn add_node(mut self, data: T) -> (Self, NodeRef) {
        if let Some(&node_ref) = self.map.get(&data) {
            (self, node_ref)
        } else {
            let node_ref = self.arena.len();
            self.arena.push(Node::new(data));
            self.map.insert(data, node_ref);

            (self, node_ref)
        }
    }

    pub fn add_relationship(mut self, parent: NodeRef, child: NodeRef) -> (Self, bool) {
        if let Some(last_child) = self.arena[parent].last_child {
            self.arena[last_child].sibling = Some(child);
        } else {
            self.arena[parent].first_child = Some(child);
        }
        self.arena[parent].last_child = Some(child);
        self.arena[child].parent = Some(parent);

        (self, true)
    }

    pub fn roots<'a>(&'a self) -> impl Iterator<Item = NodeRef> + 'a {
        self.arena
            .iter()
            .enumerate()
            .flat_map(|(i, n)| match n.parent {
                Some(_) => None,
                _ => Some(i),
            })
    }

    pub fn visit<S, F>(&self, root: NodeRef, state: S, mut f: F) -> S
    where
        F: FnMut(S, (NodeRef, &Node<T>)) -> S + Copy,
    {
        let node = &self.arena[root];
        let mut state = f(state, (root, &node));

        let mut node = node.first_child;
        while let Some(r) = node {
            state = self.visit(r, state, f);
            node = self.arena[r].sibling;
        }

        state
    }
}

impl<T> ops::Index<NodeRef> for Forest<T>
where
    T: Eq + Hash + Copy + Debug,
{
    type Output = Node<T>;

    fn index(&self, node_ref: NodeRef) -> &Node<T> {
        &self.arena[node_ref]
    }
}

pub fn breadth_first_search<S, M, F, I>(
    start_state: S,
    goal: fn(&S) -> bool,
    next_states: F,
) -> Result<(S, Vec<M>), ()>
where
    F: Fn(&S) -> I,
    I: Iterator<Item = (S, M)>,
    S: Eq + Hash + Copy,
    M: Copy,
{
    let mut q = VecDeque::new();
    q.push_back((start_state, vec![]));

    let mut visited = HashSet::new();
    visited.insert(start_state);

    while let Some((state, moves)) = q.pop_front() {
        if goal(&state) {
            return Ok((state, moves));
        }

        for (state, m) in next_states(&state) {
            if !visited.contains(&state) {
                let moves = moves.iter().copied().chain(iter::once(m)).collect();
                q.push_back((state, moves));
                visited.insert(state);
            }
        }
    }

    Err(())
}

pub fn part_1() -> u32 {
    DATA.visit(*ROOT, HashMap::new(), |mut state, (r, node)| {
        if let Some(parent) = node.parent() {
            state.insert(r, state[parent] + 1);
        } else {
            state.insert(r, 0);
        }

        state
    })
    .iter()
    .map(|(_, value)| value)
    .sum()
}

pub fn part_2() -> u32 {
    let (d, i) = DATA.visit(
        *ROOT,
        (
            HashMap::<&str, Vec<&str>>::new(),
            HashMap::<&str, Vec<&str>>::new(),
        ),
        |(mut d, mut i), (_, node)| {
            if let Some(parent) = node.parent() {
                let data = node.data();
                let parent_data = DATA[*parent].data();

                d.entry(data.to_owned())
                    .and_modify(|v| {
                        v.push(parent_data.to_owned());
                    })
                    .or_insert_with(|| vec![parent_data.to_owned()]);

                i.entry(parent_data.to_owned())
                    .and_modify(|v| {
                        v.push(data.to_owned());
                    })
                    .or_insert_with(|| vec![data.to_owned()]);
            }

            (d, i)
        },
    );

    let empty = vec![];

    breadth_first_search(
        "YOU",
        |state| state.eq(&"SAN"),
        |state| {
            d.get(state)
                .unwrap_or_else(|| &empty)
                .iter()
                .chain(i.get(state).unwrap_or_else(|| &empty).iter())
                .map(|n| (*n, 1))
        },
    )
    .expect("no solution")
    .1
    .iter()
    .sum::<u32>()
        - 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_forest() {
        let forest = Forest::new();
        let (forest, root) = forest.add_node("parent");
        let (forest, child) = forest.add_node("child");
        let (forest, _) = forest.add_relationship(root, child);

        assert_eq!(forest.roots().collect::<Vec<NodeRef>>(), vec![root]);
    }

    #[test]
    fn test_visit_count() {
        let forest = Forest::new();
        let (forest, root) = forest.add_node("parent");
        let (forest, child) = forest.add_node("child");
        let (forest, _) = forest.add_relationship(root, child);

        assert_eq!(forest.visit(root, 0, |state, _| { state + 1 }), 2);
    }

    #[test]
    fn test_visit_paths() {
        let forest = Forest::new();
        let (forest, root) = forest.add_node("parent");
        let (forest, child) = forest.add_node("child");
        let (forest, grandchild) = forest.add_node("grandchild");
        let (forest, _) = forest.add_relationship(root, child);
        let (forest, _) = forest.add_relationship(child, grandchild);

        assert_eq!(
            forest.visit(root, HashMap::new(), |mut state, (r, node)| {
                if let Some(parent) = node.parent() {
                    state.insert(r, state.get(parent).unwrap() + 1);
                } else {
                    state.insert(r, 0);
                }

                state
            }),
            vec![(root, 0), (child, 1), (grandchild, 2)]
                .into_iter()
                .collect()
        );
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }
}
