use std::collections::{VecDeque, BinaryHeap, HashSet};
use rand::rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;


use crate::model::{Node, Neighbor, Path};
use crate::builder::haversine_distance;

const MAX_LOOKBACK: u16 = 100; // how many states to look back in ancestry check

pub struct Graph {
    nodes: Vec<Node>,
    adj: Vec<Vec<Neighbor>>,
}



#[derive(Debug, Clone)]
pub struct SearchState {
    node: usize, // index of the current node in the graph
    prev: Option<usize>, // index in states
    distance: f64,
}

#[derive(Debug, Clone)]
pub struct SpecialDijkstraState {
    node: usize, // index of the current node in the graph
    prev: Option<usize>, // index in states
    distance: f64,
}


#[derive(Debug, Clone)]
pub struct HeapItem {
    state_idx: usize,
    priority: f64,
}


impl Ord for HeapItem {
    // cmp by priority (lower is better)
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .partial_cmp(&self.priority)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapItem {
    // again only priority matters
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for HeapItem {}


#[derive(Copy, Clone)]
struct BasicState {
    node: usize,
    dist: f64,
}

impl PartialEq for BasicState {
    fn eq(&self, other: &Self) -> bool { self.dist == other.dist }
}

impl Eq for BasicState {}

impl PartialOrd for BasicState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Notice we flip the order to make a min-heap
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for BasicState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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

    pub fn in_ancestry(states: &[SearchState], mut idx: usize, candidate: usize, max_lookback: u16) -> bool {
        for _ in 0..max_lookback {
            if states[idx].node == candidate {
                return true;
            }

            match states[idx].prev {
                Some(parent_idx) => idx = parent_idx,
                None => break,
            }
        }
        false
    }

    pub fn in_ancestry_dijkstra(states: &[SpecialDijkstraState], mut idx: usize, candidate: usize, max_lookback: u16) -> bool {
        for _ in 0..max_lookback {
            if states[idx].node == candidate {
                return true;
            }

            match states[idx].prev {
                Some(parent_idx) => idx = parent_idx,
                None => break,
            }
        }
        false
    }

    pub fn bfs(&self, start: usize, goal: usize, k: usize, target_distance: f64, tol: f64) -> Vec<(Vec<usize>,f64)> {
        let mut results = Vec::new();
        let mut states = Vec::new();
        let mut queue = VecDeque::new();

        states.push(SearchState {
            node: start,
            prev: None,
            distance: 0.0,
        });
        queue.push_back(0);

        while let Some(current) = queue.pop_front() {
            if results.len() >= k {
                break; // stop if we have enough results
            }
            let state = &states[current];
            let node = state.node;
            let distance_so_far = state.distance;

            if node == goal && (distance_so_far - target_distance).abs() <= tol as f64 {
                // found a valid path
                // reconstruct the path
                let mut path = Vec::new();
                let mut cur = Some(current);
                while let Some(i) = cur {
                    path.push(states[i].node);
                    cur = states[i].prev;
                }
                path.reverse(); // don't know if this really matters, don't think it does for other
                // implementations
                results.push((path, distance_so_far));
                continue;
            }

            for neighbor in &self.adj[node] {
                let next = neighbor.node_index;
                if (Self::in_ancestry(&states, current, next, MAX_LOOKBACK)) && next != goal {
                    continue;
                }

                let new_distance = distance_so_far + neighbor.edge_data.length_m;

                // check if we can still reach the target distance
                if new_distance > target_distance + tol as f64 {
                    continue; // skip paths that exceed the target distance
                }
                if new_distance + haversine_distance(
                    self.nodes[next].lat(),
                    self.nodes[next].lon(),
                    self.nodes[goal].lat(),
                    self.nodes[goal].lon(),
                ) > target_distance + tol as f64 {
                    continue; // skip paths that cannot reach the goal within tolerance
                }
                states.push(SearchState {
                    node: next,
                    prev: Some(current),
                    distance: new_distance,
                });
                queue.push_back(states.len() - 1);
            }
        }
        results
    }

    pub fn special_dijkstra(&self, start: usize, goal: usize, k: usize, target_distance: f64, tol: f64) -> Vec<(Vec<usize>,f64)> {
        let mut results = Vec::new();
        let mut heap = BinaryHeap::new();
        let mut states: Vec<SpecialDijkstraState> = Vec::new();
        states.push(SpecialDijkstraState {
            node: start,
            prev: None,
            distance: 0.0,
        });
        let d0 = haversine_distance(self.nodes[start].lat(), self.nodes[start].lon(), self.nodes[goal].lon(), self.nodes[goal].lon());
        heap.push(HeapItem {
            state_idx: 0,
            priority: (d0 - target_distance).abs(), 
        });

        while let Some(heap_item) = heap.pop() {
            let state = &states[heap_item.state_idx];
            let current = state.node;
            let distance_so_far = state.distance;
            if current == goal && (distance_so_far - target_distance).abs() <= tol as f64 {
                // found a valid path
                // reconstruct the path
                let mut path = Vec::new();
                let mut cur = Some(heap_item.state_idx);
                while let Some(i) = cur {
                    path.push(states[i].node);
                    cur = states[i].prev;
                }
                path.reverse(); // don't know if this really matters, don't think it does for other
                // implementations
                results.push((path, distance_so_far));
                if results.len() >= k {
                    break; // stop if we have enough results
                }
            }

            for neighbor in &self.adj[current] {
                let next = neighbor.node_index;
                let new_distance = distance_so_far + neighbor.edge_data.length_m;

                if (Self::in_ancestry_dijkstra(&states, heap_item.state_idx, next, MAX_LOOKBACK)) && next != goal {
                    continue;
                }
                // prune nodes too far away
                if new_distance > target_distance + tol as f64 {
                    continue; 
                }

                let d = haversine_distance(self.nodes[next].lat(), self.nodes[next].lon(), self.nodes[goal].lat(), self.nodes[goal].lon()) + distance_so_far;
                let priority = (d - target_distance).abs();

                states.push(SpecialDijkstraState {
                    node: next,
                    prev: Some(heap_item.state_idx),
                    distance: new_distance,
                });
                heap.push(HeapItem {
                    state_idx: states.len() - 1,
                    priority,
                });

                if states.len() % 100000 == 0 {
                    println!("states len: {}",states.len());
                }
            }
        }
        results
    }




    fn dijkstra(&self, goal: usize, max_dist: Option<f64>) -> (Vec<f64>,Vec<Option<usize>>) {
        let n = self.nodes.len();
        let mut dist = vec![f64::INFINITY; n];
        let mut heap = BinaryHeap::new();
        let mut parent = vec![None; n];

        // Start at goal
        dist[goal] = 0.0;
        heap.push(BasicState { node: goal, dist: 0.0 });

        while let Some(BasicState { node: u, dist: du }) = heap.pop() {
            if du > dist[u] { continue; }

            // stop exploring once du exceeds a cutoff
            if let Some(cutoff) = max_dist {
                if du > cutoff { break; }
            }

            for edge in &self.adj[u] {
                let v = edge.node_index;
                let dv = du + edge.edge_data.length_m;
                if dv < dist[v] {
                    dist[v] = dv;
                    parent[v] = Some(u);
                    heap.push(BasicState { node: v, dist: dv });
                }
            }
        }

        (dist, parent)
    }


    fn recursive_dfs(&self, node: usize, traversed_distance: f64, start: usize, d_goal: &Vec<f64>,
        target: f64, tol: f64, rng: &mut impl rand::Rng, path: &mut Vec<usize>, visited: &mut HashSet<usize>) -> Option<(Vec<usize>,f64)> {
        // Check midpoint condition (skip the start node)
        if node != start {
            let total = traversed_distance+ d_goal[node];
            if (total - target).abs() <= tol {
                return Some((path.clone(), traversed_distance));
            }
        }

        // Gather and shuffle neighbors
        let mut neighs: Vec<_> = self.adj[node]
            .iter()
            .map(|e| e.node_index)
            .collect();
        neighs.shuffle(rng);

        for &v in &neighs {
            if visited.contains(&v) {
                continue;
            }
            let edge_len = self.adj[node]
                .iter()
                .find(|e| e.node_index == v)
                .unwrap()
            .edge_data.length_m;
            let new_traversed_distance= traversed_distance + edge_len;

            // Prune overshoot and unreachable
            if new_traversed_distance > target + tol || new_traversed_distance + d_goal[v] > target + tol {
                continue;
            }

            // Recurse
            visited.insert(v);
            path.push(v);
            if let Some(solution) = Self::recursive_dfs(
                self, v, new_traversed_distance, start, d_goal, target, tol,
                rng, path, visited,
            ) {
                return Some(solution);
            }
            path.pop();
            visited.remove(&v);
        }
        None
    }


    fn find_paths_with_dfs(&self, start: usize, goal: usize, k:usize, target_distance: f64, tol: f64) -> Option<Vec<(Vec<usize>,f64)>> {
        let (d_goal, parent) = self.dijkstra(goal, Some(target_distance + tol));

        // If the goal is unreachable, return None
        if d_goal[start].is_infinite() {
            return None;
        }

        let mut rng = rng();
        let mut results: Vec<(Vec<usize>,f64)> = Vec::new();

        for _ in 0..k {
            // Prepare for DFS
            let mut path = vec![start];
            let mut visited = HashSet::new();
            visited.insert(start);
            // Find one random midpoint path
            if let Some(( mut outbound, traversed_distance)) = self.recursive_dfs(start, 0.0, start, &d_goal, target_distance, tol, &mut rng, &mut path, &mut visited) {
                // midpoint
                let mid = *outbound.last().unwrap();
                // Reconstruct inbound via parent pointers
                let mut inbound = Vec::new();
                let mut cur = mid;
                while cur != goal {
                    inbound.push(cur);
                    cur = match parent[cur] {
                        Some(p) => p,
                        None => break,
                    };
                }
                inbound.push(goal);
                // Combine (avoid duplicate mid)
                outbound.pop();
                outbound.extend(inbound);
                // Create path object
                results.push((outbound, traversed_distance + d_goal[mid]));
            }
        }

        Some(results)
    }


    // maps a lat and lon to a node in the graph
    // retursn the index of the node in the nodes array
    fn map_lat_lon_to_node(&self, lat: f64, lon: f64) -> usize {
        let mut idx: usize = 0;
        let mut shortest_dist = haversine_distance(lat, lon, self.nodes[0].lat(), self.nodes[0].lon());

        for (i, node) in self.nodes.iter().enumerate().skip(1) {
            let d = haversine_distance(lat, lon, node.lat(), node.lon());
            if d < shortest_dist {
                shortest_dist = d;
                idx = i;
            }
        }
        idx
    }


    fn convert_to_path(&self, indicies: &Vec<usize>, distance: f64) -> Path {
        let mut nodes: Vec<Node> = Vec::new();
        for idx in indicies.iter() {
            nodes.push(self.nodes[*idx]);
        }
        Path::new(nodes, distance)
    }

    pub fn get_paths_bfs(&self, start_lat: f64,start_lon:f64,   goal_lat: f64, goal_lon:f64, k: usize, target_distance: f64, tol: f64) -> Vec<Path> {
        let mut paths = Vec::new();
        let start_idx = self.map_lat_lon_to_node(start_lat, start_lon);
        let goal_idx = self.map_lat_lon_to_node(goal_lat, goal_lon);
        println!("Start node index: {}, Goal node index: {}", start_idx, goal_idx);
        let solutions = self.bfs(start_idx, goal_idx, k,target_distance, tol);
        for (solution,dist) in solutions.iter() {
            paths.push(self.convert_to_path(solution, *dist));
        }

        paths
    }
    
    pub fn get_paths_dfs(&self, start_lat: f64, start_lon: f64, goal_lat: f64, goal_lon: f64, k: usize, target_distance: f64, tol: f64) -> Option<Vec<Path>> {
        let mut paths = Vec::new();
        let start_idx = self.map_lat_lon_to_node(start_lat, start_lon);
        let goal_idx = self.map_lat_lon_to_node(goal_lat, goal_lon);
        println!("Start node index: {}, Goal node index: {}", start_idx, goal_idx);
        let solutions = self.find_paths_with_dfs(start_idx, goal_idx, k, target_distance, tol);
        if let Some(solutions) = solutions {
            for (solution,dist) in solutions.iter() {
                paths.push(self.convert_to_path(solution, *dist));
            }
        }
        Some(paths)
    }

    pub fn get_paths_special_dijkstra(
        &self, start_lat: f64, start_lon: f64, goal_lat: f64, goal_lon: f64, k: usize, target_distance: f64, tol: f64
    ) -> Vec<Path> {
        let mut paths = Vec::new();
        let start_idx = self.map_lat_lon_to_node(start_lat, start_lon);
        let goal_idx = self.map_lat_lon_to_node(goal_lat, goal_lon);
        println!("Start node index: {}, Goal node index: {}", start_idx, goal_idx);
        let solutions = self.special_dijkstra(start_idx, goal_idx, k,target_distance, tol);
        for (solution,dist) in solutions.iter() {
            paths.push(self.convert_to_path(solution, *dist));
        }

        paths
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







#[cfg(test)]
mod nearest_node_tests {
    use super::*;
    use crate::model::Node;

    /// Build a tiny graph with three nodes at distinct coordinates.
    ///
    /// Nodes:
    ///   0: (0.0, 0.0)
    ///   1: (0.0, 1.0)
    ///   2: (1.0, 0.0)
    fn build_three_node_graph() -> Graph {
        let nodes = vec![
            Node::new(0, 0.0, 0.0),
            Node::new(1, 0.0, 1.0),
            Node::new(2, 1.0, 0.0),
        ];
        // adjacency doesn't matter for nearest‐node tests
        let adj = vec![Vec::new(), Vec::new(), Vec::new()];
        Graph::new(nodes, adj)
    }

    #[test]
    fn nearest_to_origin() {
        let graph = build_three_node_graph();
        // Exactly at node 0
        assert_eq!(graph.map_lat_lon_to_node(0.0, 0.0), 0);
        // Slightly off toward node 0
        assert_eq!(graph.map_lat_lon_to_node(0.1, -0.1), 0);
    }

    #[test]
    fn nearest_to_north() {
        let graph = build_three_node_graph();
        // Point near (0,1) should pick node 1
        assert_eq!(graph.map_lat_lon_to_node(0.0, 0.9), 1);
        // Slight latitude shift but still closer to node 1
        assert_eq!(graph.map_lat_lon_to_node(0.1, 0.9), 1);
    }

    #[test]
    fn nearest_to_east() {
        let graph = build_three_node_graph();
        // Point near (1,0) should pick node 2
        assert_eq!(graph.map_lat_lon_to_node(0.9, 0.1), 2);
        // Slight longitude shift but still closer to node 2
        assert_eq!(graph.map_lat_lon_to_node(0.9, -0.1), 2);
    }
}


#[cfg(test)]
mod loop_tests {
    use super::*;
    use crate::model::{Node, Neighbor, EdgeData};

    fn make_node(id: u64) -> Node {
        Node::new(id, 0.0, 0.0)
    }

    /// Graph layout (edge weights in parentheses):
    ///
    ///          2
    ///         / \
    ///     (5)/   \(5)
    ///       / (5) \
    ///      1-------3
    ///      \      
    ///  (5)  \(5) 
    ///        \    
    ///         4   
    ///            
    ///      
    ///         
    ///
    /// - Big cycle: 1↔2↔3↔1 (each edge = 5)
    /// - Small loop: 1↔4↔1 (each edge = 5)
    /// - Node indices: 0 unused, 1 = start/goal, 2 & 3 = big cycle, 4 = small loop
    #[test]
    fn test_bfs_finds_exact_loop_on_complex_graph() {
        // build nodes
        let nodes = vec![
            make_node(10), // idx 0, unused
            make_node(11), // idx 1: start/goal
            make_node(12), // idx 2
            make_node(13), // idx 3
            make_node(14), // idx 4
        ];
        // initialize empty adjacency
        let mut adj = vec![Vec::new(); 5];
        // helper to add undirected edge of length 5.0
        let mut add_edge = |u: usize, v: usize| {
            adj[u].push(Neighbor {
                osm_id: 1, // dummy OSM ID
                node_index: v,
                edge_data: EdgeData { way_id: 1, length_m: 5.0 },
            });
            adj[v].push(Neighbor {
                osm_id: 0, // dummy OSM ID
                node_index: u,
                edge_data: EdgeData { way_id: 1, length_m: 5.0 },
            });
        };

        // big 3-node cycle at 1→2→3→1
        add_edge(1, 2);
        add_edge(2, 3);
        add_edge(3, 1);

        // small 2-node loop at 1→4→1
        add_edge(1, 4);

        let graph = Graph::new(nodes, adj);

        // search for a single loop from 1 back to 1, exact length = 15.0
        let paths = graph.bfs(
            1,
            1,
            1,
            15.0,
            0.1,
        );

        // only the [1,2,3,1] loop qualifies
        assert_eq!(paths.len(), 1, "expected exactly one matching loop");

        let (path, dist) = &paths[0];
        assert_eq!(path, &vec![1, 2, 3, 1], "wrong loop sequence");
        assert!((dist - 15.0).abs() < std::f64::EPSILON, "wrong loop distance");
    }
}









#[cfg(test)]
mod special_dijkstra_tests {
    use super::*;
    use crate::model::{Node, Neighbor, EdgeData};

    fn make_node(id: u64) -> Node {
        Node::new(id, 0.0, 0.0)
    }

    /// Diamond graph:
    ///
    ///    0
    ///   / \
    ///  1   2
    ///   \ /
    ///    3
    ///
    /// 0–1 + 1–3 = 5 + 5 = 10  
    /// 0–2 + 2–3 = 6 + 6 = 12
    fn build_diamond() -> Graph {
        let nodes = (0..4).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
            ],
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
            ],
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
            ],
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 0, length_m: 5.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 0, length_m: 6.0 } },
            ],
        ];
        Graph::new(nodes, adj)
    }

    #[test]
    fn sd_returns_exact_length() {
        let graph = build_diamond();
        let paths = graph.special_dijkstra(0, 3, 5, 10.0, 0.0);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].0, vec![0, 1, 3]);
        assert!((paths[0].1 - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sd_returns_within_tolerance() {
        let graph = build_diamond();
        let results = graph.special_dijkstra(0, 3, 5, 11.0, 2.0)
            .into_iter()
            .map(|(p, d)| (p, d))
            .collect::<Vec<_>>();
        // Expect both routes [0,1,3]=10 and [0,2,3]=12
        let mut paths: Vec<Vec<usize>> = results.iter().map(|(p, _)| p.clone()).collect();
        paths.sort();
        assert_eq!(paths, vec![vec![0,1,3], vec![0,2,3]]);
    }

    #[test]
    fn sd_respects_k_limit() {
        let graph = build_diamond();
        let paths = graph.special_dijkstra(0, 3, 1, 11.0, 2.0);
        assert_eq!(paths.len(), 1);
        // the best‐scoring route is [0,1,3]
        assert_eq!(paths[0].0, vec![0,1,3]);
    }

    #[test]
    fn sd_unreachable_returns_empty() {
        let nodes = vec![make_node(0), make_node(1)];
        let adj = vec![vec![], vec![]];
        let graph = Graph::new(nodes, adj);
        let paths = graph.special_dijkstra(0, 1, 3, 10.0, 5.0);
        assert!(paths.is_empty());
    }

    /// Complex graph from earlier tests:
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
    /// 0→1→3→5 = 6  
    /// 0→2→3→5 = 6  
    /// 0→2→4→5 = 8
    fn build_complex() -> Graph {
        let nodes = (0..6).map(make_node).collect::<Vec<_>>();
        let adj = vec![
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 1, length_m: 3.0 } },
            ],
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
            ],
            vec![
                Neighbor { osm_id: 0, node_index: 0, edge_data: EdgeData { way_id: 1, length_m: 3.0 } },
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
                Neighbor { osm_id: 4, node_index: 4, edge_data: EdgeData { way_id: 1, length_m: 4.0 } },
            ],
            vec![
                Neighbor { osm_id: 1, node_index: 1, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
                Neighbor { osm_id: 5, node_index: 5, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
            ],
            vec![
                Neighbor { osm_id: 2, node_index: 2, edge_data: EdgeData { way_id: 1, length_m: 4.0 } },
                Neighbor { osm_id: 5, node_index: 5, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
            ],
            vec![
                Neighbor { osm_id: 3, node_index: 3, edge_data: EdgeData { way_id: 1, length_m: 2.0 } },
                Neighbor { osm_id: 4, node_index: 4, edge_data: EdgeData { way_id: 1, length_m: 1.0 } },
            ],
        ];
        Graph::new(nodes, adj)
    }

    #[test]
    fn sd_k_constrained_complex() {
        let graph = build_complex();
        let results = graph.special_dijkstra(0, 5, 3, 6.0, 1.0);
        assert_eq!(results.len(), 2);
        // two perfect 6.0 loops: [0,1,3,5] and [0,2,3,5]
        let mut paths: Vec<Vec<usize>> = results.iter().map(|(p, _)| p.clone()).collect();
        paths.sort();
        assert_eq!(paths, vec![vec![0,1,3,5], vec![0,2,3,5]]);
        // distances are both exactly 6.0
        for &(_, d) in &results {
            assert!((d - 6.0).abs() < f64::EPSILON);
        }
    }
}

