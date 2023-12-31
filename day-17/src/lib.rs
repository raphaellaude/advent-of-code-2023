use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use petgraph::{graph::NodeIndex, Directed, Graph};

pub fn part_one(input: &str) -> u32 {
    let graph = parse(input);

    let n_lines = input.lines().count();
    let line_len = input.lines().next().unwrap().chars().count();

    let shortest_path = crucible_djikstra(&graph, 0, n_lines * line_len - 1);

    shortest_path
        .iter()
        .map(|node| *(graph.node_weight(*node).unwrap()))
        .sum()
}

fn parse(input: &str) -> Graph<u32, u32, Directed> {
    let mut graph = Graph::<u32, u32, Directed>::new();
    input.lines().for_each(|line| {
        line.chars().for_each(|c| {
            graph.add_node(c.to_digit(10).unwrap());
        });
    });

    let n_lines = input.lines().count() as u32;
    let line_len = input.lines().next().unwrap().chars().count() as u32;

    graph.extend_with_edges(
        (0..n_lines * line_len)
            .flat_map(|node| {
                let mut node_edges = vec![];
                if node >= line_len {
                    node_edges.push((node, node - line_len)); // above
                }
                if node % line_len > 0 {
                    node_edges.push((node, node - 1)); // left
                }
                if node % line_len != line_len - 1 {
                    node_edges.push((node, node + 1)); // right
                }
                if node < n_lines * (line_len - 1) {
                    node_edges.push((node, node + line_len)) // below
                }
                node_edges
            })
            .collect::<Vec<(u32, u32)>>(),
    );

    graph
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: NodeIndex,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn crucible_djikstra(
    graph: &Graph<u32, u32, Directed>,
    source: usize,
    dest: usize,
) -> Vec<NodeIndex> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut heap = BinaryHeap::new();

    let source = NodeIndex::new(source);
    let dest = NodeIndex::new(dest);

    for node in graph.node_indices() {
        dist.insert(node, u32::MAX);
        prev.insert(node, None);
    }

    dist.insert(source, 0);
    heap.push(State {
        cost: 0,
        position: source,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[&position] {
            continue;
        }

        if position == dest {
            let mut path = vec![];
            let mut current = dest;

            while current != source {
                path.push(current);
                current = prev[&current].unwrap();
            }

            path.reverse();
            return path;
        }

        for v in graph.neighbors(position) {
            let next = State {
                cost: cost + graph.node_weight(v).unwrap(),
                position: v,
            };

            if next.cost < dist[&v] {
                heap.push(next);
                dist.insert(v, next.cost);
                prev.insert(v, Some(position));
            }
        }
    }

    panic!("No shortest path found");
}

pub fn part_two(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part_one(&input), 102)
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_two(&input), 0)
    // }
}
