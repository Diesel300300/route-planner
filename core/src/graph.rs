use std::collections::VecDeque;

use crate::model::{Node, Neighbor};

pub struct Graph {
    nodes: Vec<Node>,
    adj: Vec<Vec<Neighbor>>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, adj: Vec<Vec<Neighbor>>) -> Self {
        Graph {
            nodes,
            adj,
        }
    }
    
    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn adj(&self) -> &Vec<Vec<Neighbor>> {
        &self.adj
    }
    
    pub fn bfs(&self, start: usize, goal: usize, k: usize, target_distance: f64, tol: f64) -> Vec<(Vec<usize>,f64)> {
        let mut results = Vec::new();
        let mut queue : VecDeque<(Vec<usize>,f64)> = VecDeque::new();
        
        queue.push_back((vec![start],0.0));

        while let Some((path, dist_so_far)) = queue.pop_front() {
            if results.len() >= k {
                break;
            }
            let &last = path.last().unwrap();
            if last == goal {
                if (dist_so_far - target_distance).abs() <= tol as f64 {
                    results.push((path.clone(),dist_so_far));
                }
                continue;
            }
            for neighbor in &self.adj[last] {
                let next = neighbor.node_index;
                if path.contains(&next) { 
                    continue; // Avoid cycles
                }
                let new_dist = dist_so_far + neighbor.edge_data.length_m;
                // TODO: probably a smarter way to check sooner, maybe calc distance from node
                // where we are now to goal + dist_so_far and check if still possible
                // if over target distance, skip
                if new_dist > target_distance + tol as f64 {
                    continue; 
                }

                let mut new_path = path.clone();
                new_path.push(next);
                queue.push_back((new_path, new_dist));
            }
        }
        results
    }
}


#[cfg(test)]
mod constrained_bfs_tests {
    use super::*;
    use crate::model::{Node, Neighbor, EdgeData};

    fn make_node(id: u64) -> Node {
        Node::new(id, 0.0, 0.0)
    }

    /// Build a simple graph:
    ///
    ///    0
    ///   / \
    ///  1   2
    ///   \ /
    ///    3
    ///
    /// Edges 0–1 and 1–3 have length 5.0 (total 10.0)
    /// Edges 0–2 and 2–3 have length 6.0 (total 12.0)
    fn build_diamond() -> Graph {
        let nodes = (0..4).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            // neighbors of 0
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
            ],
            // neighbors of 1
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
            ],
            // neighbors of 2
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
            ],
            // neighbors of 3
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
            ],
        ];
        Graph::new(nodes, adj)
    }

    #[test]
    fn returns_exactly_target_length() {
        let graph = build_diamond();
        // only [0,1,3] has length 10.0
        let paths = graph.bfs(0, 3, 5, 10.0, 0.0);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].0, vec![0, 1, 3]);
        assert_eq!(paths[0].1, 10.0);
    }

    #[test]
    fn returns_within_tolerance() {
        let graph = build_diamond();
        // accept both 10.0 and 12.0 within ±2.0
        let paths = graph.bfs(0, 3, 5, 11.0, 2.0);
        // two paths: [0,1,3] length 10, [0,2,3] length 12
        let mut results = paths.iter().map(|(p, _)| p.clone()).collect::<Vec<_>>();
        results.sort();
        assert_eq!(results, vec![vec![0,1,3], vec![0,2,3]]);
    }

    #[test]
    fn respects_k_limit() {
        let graph = build_diamond();
        // k = 1, even though two are in range, only first is returned
        let paths = graph.bfs(0, 3, 1, 11.0, 2.0);
        assert_eq!(paths.len(), 1);
        // must be the [0,1,3] path
        assert_eq!(paths[0].0, vec![0,1,3]);
    }

    #[test]
    fn no_paths_if_none_within_range() {
        let graph = build_diamond();
        // tolerance zero, target 11.0, no path exactly 11.0
        let paths = graph.bfs(0, 3, 5, 11.0, 0.0);
        assert!(paths.is_empty());
    }

    #[test]
    fn unreachable_returns_empty() {
        // two isolated nodes
        let nodes = vec![make_node(0), make_node(1)];
        let adj = vec![vec![], vec![]];
        let graph = Graph::new(nodes, adj);

        let paths = graph.bfs(0, 1, 3, 10.0, 5.0);
        assert!(paths.is_empty());
    }


    /// Build a more complex graph:
    ///
    ///      0
    ///     / \
    ///   (2) (3)
    ///   /     \
    ///  1       2
    ///  |     /   \
    /// (2)  (1)   (4)
    ///  |   /       \
    ///  3          4
    ///   \        /
    ///    (2)  (1)
    ///      \  /
    ///       5
    ///
    /// Edge lengths as shown in parentheses.
    /// Paths from 0→5:
    /// - 0–1–3–5: 2+2+2 = 6
    /// - 0–2–3–5: 3+1+2 = 6
    /// - 0–2–4–5: 3+4+1 = 8
    fn build_complex() -> Graph {
        let nodes = (0..6).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            // 0
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 1, length_m: 3.0 } },
            ],
            // 1
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
            ],
            // 2
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 1, length_m: 3.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
                Neighbor { osm_id: 4, node_index: 4, edge_data: EdgeData { way_id: 1, length_m: 4.0 } },
            ],
            // 3
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
                Neighbor { osm_id: 5, node_index: 5, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
            ],
            // 4
            vec![
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 1, length_m: 4.0 } },
                Neighbor { osm_id: 5, node_index: 5, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
            ],
            // 5
            vec![
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 4, node_index: 4, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
            ],
        ];
        Graph::new(nodes, adj)
    }

    #[test]
    fn test_bfs_k_constrained_complex() {
        let graph = build_complex();

        // We want up to 3 paths, target = 6.0 meters, tol = 1.0
        let paths = graph.bfs(0, 5, 3, 6.0, 1.0);
        // Expect exactly two matching routes of length 6:
        //   [0,1,3,5] and [0,2,3,5], in that BFS order
        assert_eq!(paths.len(), 2);

        let expected = vec![
            (vec![0, 1, 3, 5], 6.0),
            (vec![0, 2, 3, 5], 6.0),
        ];
        assert_eq!(paths, expected);
    }
}

