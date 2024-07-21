use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse ordering to create a min-heap
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    use std::collections::BinaryHeap;
    let mut priority_queue = BinaryHeap::new();

    let mut distances: HashMap<Node, Cost> = g
        .nodes
        .iter()
        .map(|&x| if x == start { (x, 0) } else { (x, usize::MAX) })
        .collect();

    distances.insert(start, 0);

    priority_queue.push(Step {
        cost: 0,
        position: start,
        history: vec![],
    });

    while let Some(Step {
        cost,
        position,
        mut history,
    }) = priority_queue.pop()
    {
        if position == goal {
            history.push(goal);
            return Some((history, cost));
        }

        if let Some(neighbors) = &g.edges.get(&position) {
            for &(neighbor, weight) in neighbors.iter() {
                if weight < distances[&neighbor] {
                    let mut next = Step {
                        cost: cost + weight,
                        position: neighbor,
                        history: history.clone(),
                    };
                    next.history.push(position);
                    priority_queue.push(next);

                    if let Some(old_cost) = distances.get_mut(&neighbor) {
                        // this below line is modifying the map by reference
                        *old_cost = weight;
                    }
                }
            }
        }
    }

    return None;
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

// tests are by the course
#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
