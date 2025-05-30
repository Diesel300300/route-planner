use crate::graph::Graph;
use crate::model::{Node, Neighbor, EdgeData};

/// Build an N×N grid graph (total N*N nodes), laid out row-major:
/// node index = row*N + col, coords unused here (0,0), all edges length = 1.0
pub fn make_grid_graph(n: usize) -> Graph {
    // 1) create all N*N nodes
    let mut nodes = Vec::with_capacity(n * n);
    for i in 0..(n * n) {
        // assign a unique u64 ID (here same as index) and dummy coords
        nodes.push(Node::new(i as u64, 0.0, 0.0));
    }

    // 2) adjacency list, initially empty for each node
    let mut adj = vec![Vec::new(); n * n];

    // 3) helper to add an undirected unit-length edge
    let mut link = |u: usize, v: usize| {
        let edge = EdgeData { way_id: 0, length_m: 1.0 };
        adj[u].push(Neighbor {osm_id:0, node_index: v, edge_data: edge });
        adj[v].push(Neighbor {osm_id:0, node_index: u, edge_data: edge });
    };

    // 4) connect grid neighbors
    for row in 0..n {
        for col in 0..n {
            let idx = row * n + col;
            // right neighbor
            if col + 1 < n {
                link(idx, idx + 1);
            }
            // down neighbor
            if row + 1 < n {
                link(idx, idx + n);
            }
        }
    }

    Graph::new(nodes, adj)
}

/*
#[cfg(test)]
mod heap_tests_bfs {
    #[cfg(feature = "dhat-heap")]
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;
    
    use super::make_grid_graph;
    use dhat::Profiler;
    use crate::create_graph;

    #[test]
    fn profile_bfs_memory() {
        let _p = Profiler::new_heap();
        let node_axis = 5;
        let graph = make_grid_graph(node_axis);
        // run one heavy BFS
        let _ = graph.bfs(0, node_axis*node_axis-1, 1, 100.0, 50.0);
        // on test teardown you’ll get the same dhat summary
    }

    #[test]
    fn profile_real_osm() {
        let _p = Profiler::new_heap();
        let file_path = "data/map";
        let accepted_road_types: &[&str] = &["residential","unclassified","track","service","tertiary","road","secondary","primary","trunk","primary_link","trunk_link","tertiary_link","secondary_link","highway",
        ];
        let graph = create_graph(file_path, accepted_road_types).expect("Failed to create graph from OSM data");
        print!("Graph created with {} nodes and {} edges\n", graph.nodes().len(), graph.adj().len());

        let _ = graph.bfs(1838, 1816, 1, 3000.0, 100.0);
        // on test teardown you’ll get the same dhat summary
    }
}
*/


#[cfg(test)]
mod heap_tests_dijkstra {
    #[cfg(feature = "dhat-heap")]
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;
    
    use super::make_grid_graph;
    use dhat::Profiler;
    use crate::create_graph;

    #[test]
    fn profile_bfs_memory() {
        let _p = Profiler::new_heap();
        let node_axis = 5;
        let graph = make_grid_graph(node_axis);
        // run one heavy BFS
        let _ = graph.bfs(0, node_axis*node_axis-1, 1, 100.0, 50.0);
        // on test teardown you’ll get the same dhat summary
    }

    #[test]
    fn profile_real_osm() {
        let _p = Profiler::new_heap();
        let file_path = "data/map";
        let accepted_road_types: &[&str] = &["residential","unclassified","track","service","tertiary","road","secondary","primary","trunk","primary_link","trunk_link","tertiary_link","secondary_link","highway",
        ];
        let graph = create_graph(file_path, accepted_road_types).expect("Failed to create graph from OSM data");
        print!("Graph created with {} nodes and {} edges\n", graph.nodes().len(), graph.adj().len());

        let _ = graph.special_dijkstra(1838, 1816, 1, 10000.0, 100.0);
        // on test teardown you’ll get the same dhat summary
    }
}
