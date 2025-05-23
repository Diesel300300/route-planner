use std::collections::HashMap;
use crate::model::{Node, Neighbor, EdgeData, Way};
use crate::graph::Graph;


const EARTH_RADIUS_M: f64 = 6_371_000.0;

pub fn haversine_distance(lat1: f64, lon1: f64,lat2: f64, lon2: f64,) -> f64 {
    let (φ1, λ1, φ2, λ2) = (
        lat1.to_radians(),
        lon1.to_radians(),
        lat2.to_radians(),
        lon2.to_radians(),
    );
    let dφ = φ2 - φ1;
    let dλ = λ2 - λ1;
    let a = (dφ / 2.0).sin().powi(2)
          + φ1.cos() * φ2.cos() * (dλ / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_M * c
}


pub struct GraphBuilder {
    /// Map real OSM ID → our new 0..N index
    id_to_idx: HashMap<u64, usize>,

    pub nodes: Vec<Node>,
    pub adj: Vec<Vec<Neighbor>>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            id_to_idx: HashMap::new(),
            nodes:     Vec::new(),
            adj:       Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) -> usize {
        if self.id_to_idx.contains_key(&node.id()) {
            return node.id() as usize  
        }
        let idx = self.nodes.len();
        self.id_to_idx.insert(node.id(), idx);
        self.nodes.push(node);
        self.adj.push(Vec::new());
        idx
    }
        
    // use OSM ID as input
    pub fn add_edge_bidirectional(&mut self, from: u64, to: u64, edge_data: EdgeData) {
        let from_idx = *self.id_to_idx.get(&from).expect("Node not found");
        let to_idx = *self.id_to_idx.get(&to).expect("Node not found");
        self.adj[from_idx].push(Neighbor { osm_id: to, node_index: to_idx, edge_data });
        self.adj[to_idx].push(Neighbor { osm_id: from, node_index: from_idx, edge_data });
    }

    pub fn add_way(&mut self, way: &Way) {
        // assume order of nodes in way means they are connected
        for i in 0..way.nodes().len() - 1 {
            // add nodes to the graph
            let from = way.nodes()[i].clone();
            let to = way.nodes()[i + 1].clone();
            self.add_node(from);
            self.add_node(to);
            let edge_data = EdgeData {
                way_id: way.id(),
                length_m: haversine_distance(from.lat(), from.lon(), to.lat(),to.lon()),
            };
            self.add_edge_bidirectional(from.id(), to.id(), edge_data);
        }
    }

    pub fn build(self) -> Graph {
        Graph::new(self.nodes, self.adj)
    }

}




#[test]
fn add_node() {
    let mut builder = GraphBuilder::new();
    let node1 = Node::new(1, 52.0, 13.0);
    let node2 = Node::new(2, 52.1, 13.1);
    let node3 = Node::new(3, 52.2, 13.2);
    
    builder.add_node(node1);
    builder.add_node(node2);
    builder.add_node(node3);
    
    let graph = builder.build();
    assert_eq!(graph.nodes().len(), 3);
    assert_eq!(graph.adj().len(), 3);
    assert_eq!(graph.adj()[0].len(), 0);
}

#[test]
fn add_edge_bidirectional() {
    let mut builder = GraphBuilder::new();
    let node1 = Node::new(1, 52.0, 13.0);
    let node2 = Node::new(2, 52.1, 13.1);
    
    builder.add_node(node1);
    builder.add_node(node2);
    
    let edge_data = EdgeData {
        way_id: 1,
        length_m: 100.0,
    };
    
    builder.add_edge_bidirectional(1, 2, edge_data);
    
    let graph = builder.build();
    assert_eq!(graph.adj()[0].len(), 1);
    assert_eq!(graph.adj()[1].len(), 1);
}

#[test]
fn add_way() {
    let mut builder = GraphBuilder::new();
    let node1 = Node::new(1, 52.0, 13.0);
    let node2 = Node::new(2, 52.1, 13.1);
    let node3 = Node::new(3, 52.2, 13.2);
    
    let way = Way::new(1, vec![1,2,3],vec![node1.clone(), node2.clone(), node3.clone()]);
    
    builder.add_way(&way);
    
    let graph = builder.build();
    assert_eq!(graph.nodes().len(), 3);
    assert_eq!(graph.adj()[0].len(), 1);
    assert_eq!(graph.adj()[1].len(), 2);
    assert_eq!(graph.adj()[2].len(), 1);
    // the first neighbor of node 1 should be node 2 OSM ID

    assert_eq!(graph.adj()[0][0].osm_id, 2);
    assert_eq!(graph.adj()[1].len(), 2);
    assert_eq!(graph.adj()[1][0].osm_id, 1);
    assert_eq!(graph.adj()[1][1].osm_id, 3);
}

#[test]
fn add_two_ways() {
    let mut builder = GraphBuilder::new();
    let node1 = Node::new(1, 52.0, 13.0);
    let node2 = Node::new(2, 52.1, 13.1);
    let node3 = Node::new(3, 52.2, 13.2);
    
    let way = Way::new(1, vec![1,2,3],vec![node1.clone(), node2.clone(), node3.clone()]);
    
    let node4 = Node::new(4, 52.3, 13.3);
    let way2 = Way::new(2, vec![3,4],vec![node3.clone(), node4.clone()]);

    builder.add_way(&way);
    builder.add_way(&way2);
    let graph = builder.build();

    assert_eq!(graph.nodes().len(), 4);
    assert_eq!(graph.adj()[0].len(), 1);

    // node with OSM ID 3 should have 2 neighbors, 2 and 4
    assert_eq!(graph.adj()[2].len(), 2);
    assert_eq!(graph.adj()[2][0].osm_id, 2);
    assert_eq!(graph.adj()[2][1].osm_id, 4);
}


#[test]
fn add_multiple_ways() {
    let mut builder = GraphBuilder::new();
    let node1 = Node::new(1, 52.0, 13.0);
    let node2 = Node::new(2, 52.1, 13.1);
    let node3 = Node::new(3, 52.2, 13.2);
    
    let way = Way::new(1, vec![1,2,3],vec![node1.clone(), node2.clone(), node3.clone()]);
    
    let node4 = Node::new(4, 52.3, 13.3);
    let way2 = Way::new(2, vec![3,4],vec![node3.clone(), node4.clone()]);

    let node5 = Node::new(5, 52.4, 13.4);
    let node6 = Node::new(6, 52.5, 13.5);
    let way3 = Way::new(3, vec![3,5,6],vec![node3.clone(), node5.clone(), node6.clone()]);

    builder.add_way(&way);
    builder.add_way(&way2);
    builder.add_way(&way3);
    let graph = builder.build();

    assert_eq!(graph.nodes().len(), 6);

    // node with OSM ID 3 should have 3 neighbors, 2,4 and 5
    assert_eq!(graph.adj()[2].len(), 3);
    assert_eq!(graph.adj()[2][0].osm_id, 2);
    assert_eq!(graph.adj()[2][1].osm_id, 4);
    assert_eq!(graph.adj()[2][2].osm_id, 5);
    
    // situation:
    // (1---2---[3)---4]
    //           |
    //           5---6
}


