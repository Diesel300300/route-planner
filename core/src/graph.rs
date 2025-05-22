use std::collections::VecDeque;

use crate::model::{Node, Neighbor, EdgeData};

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
    
    pub fn bfs(&self, start: usize, goal: usize) -> Option<Vec<usize>> {
        let n = self.nodes.len();
        let mut visited = vec![false; n];
        let mut prev = vec![None; n];
        let mut queue = VecDeque::new();
        
        visited[start] = true;
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if current == goal {
                // found it reconstruct path
                let mut path = Vec::new();
                let mut cur = Some(goal);
                while let Some(idx) = cur {
                    path.push(idx);
                    cur = prev[idx];
                }
                path.reverse();
                return Some(path);
            }

            // didn't find it check all neighbors that are not visited
            for nbr in &self.adj[current] {
                let nbr_index = nbr.node_index;
                if !visited[nbr_index] {
                    visited[nbr_index] = true;
                    prev[nbr_index] = Some(current);
                    queue.push_back(nbr_index);
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Node, Neighbor, EdgeData};

    /// Helper to build a Node with dummy coordinates.
    fn make_node(osm_id: u64) -> Node {
        Node::new(osm_id, 0.0, 0.0)
    }

    /// Test 1: diamond shape
    ///
    /// Graph:
    ///     0
    ///    / \
    ///   1   2
    ///    \ /
    ///     3
    ///
    #[test]
    fn test_bfs_diamond() {
        let nodes = (0..4).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            vec![Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData{ way_id:1, length_m:1.0 } },
                 Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData{ way_id:1, length_m:1.0 } }],
            vec![Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData{ way_id:1, length_m:1.0 } },
                 Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData{ way_id:1, length_m:1.0 } }],
            vec![Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData{ way_id:1, length_m:1.0 } },
                 Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData{ way_id:1, length_m:1.0 } }],
            vec![Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData{ way_id:1, length_m:1.0 } },
                 Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData{ way_id:1, length_m:1.0 } }],
        ];
        let graph = Graph::new(nodes, adj);

        // BFS from 0 to 3 can go 0→1→3 or 0→2→3; both are length=2,
        // but our implementation will pick 0→1→3 (1 before 2 in adjacency).
        let path = graph.bfs(0, 3);
        assert_eq!(path, Some(vec![0, 1, 3]));
    }

    /// Test 2: cycle graph
    ///
    /// Graph:
    ///     0 → 1 → 2
    ///     ↑       ↓
    ///     └───────┘
    ///
    #[test]
    fn test_bfs_cycle() {
        let nodes = (0..3).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            vec![Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData{ way_id:2, length_m:1.0 } }],
            vec![Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData{ way_id:2, length_m:1.0 } }],
            vec![Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData{ way_id:2, length_m:1.0 } }],
        ];
        let graph = Graph::new(nodes, adj);

        // BFS from 0 to 2 should go 0→1→2
        let path = graph.bfs(0, 2);
        assert_eq!(path, Some(vec![0, 1, 2]));
    }

    /// Test 3: unreachable
    ///
    /// Graph:
    ///   0    1
    ///
    #[test]
    fn test_bfs_unreachable() {
        let nodes = (0..2).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            vec![],    // 0 has no edges
            vec![],    // 1 is isolated
        ];
        let graph = Graph::new(nodes, adj);

        // No path from 0 to 1
        let path = graph.bfs(0, 1);
        assert_eq!(path, None);
    }

    /// Test 4: start == goal
    ///
    /// Graph:
    ///    single node 0
    ///
    #[test]
    fn test_bfs_trivial() {
        let nodes = vec![make_node(0)];
        let adj = vec![vec![]];
        let graph = Graph::new(nodes, adj);

        // Path from 0 to 0 is just [0]
        let path = graph.bfs(0, 0);
        assert_eq!(path, Some(vec![0]));
    }
}
