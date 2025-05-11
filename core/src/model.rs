use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Node {
    id: u64,
    lat: f64, 
    lon: f64
}

// getters for wasm package
// so this is external
#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u64 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn lat(&self) -> f64 {
        self.lat
    }

    #[wasm_bindgen(getter)]
    pub fn lon(&self) -> f64 {
        self.lon
    }
}

// no wasm here so this is internal for me to use
impl Node {
    pub fn new(id: u64, lat: f64, lon: f64) -> Self {
        Node { id, lat, lon }
    }
}

pub struct Way {
    pub node_refs: Vec<u64>
}

impl Way {
    pub fn new(node_refs: Vec<u64>) -> Self {
        Way { node_refs }
    }

    pub fn way(&self) -> Vec<u64> {
        // check if this is worth or i need to find something else
        self.node_refs.clone()
    }
}

