use crate::{
    fdl::FDLOptions,
    graph::{Edge, Graph, Node},
};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmContainer {
    graph: Graph,
    options: FDLOptions,
}

#[wasm_bindgen]
impl WasmContainer {
    pub fn positions_x_ptr(&self) -> *const f32 {
        self.graph.positions_x.as_ptr()
    }

    pub fn positions_y_ptr(&self) -> *const f32 {
        self.graph.positions_y.as_ptr()
    }

    pub fn node_count(&self) -> usize {
        self.graph.positions_x.len()
    }

    pub fn get_index_of(&self, orig_id: &str) -> Option<usize>{
        self.graph.ids_map.get(orig_id).copied()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(nodes: &str, edges: &str, options: &str) -> WasmContainer {
        if let Ok(parsed_nodes) = serde_json::from_str::<Vec<Node>>(nodes) {
            if let Ok(parsed_edges) = serde_json::from_str::<Vec<Edge>>(edges) {
                if let Ok(parsed_opts) = serde_json::from_str::<FDLOptions>(options) {
                    return WasmContainer {
                        graph: Graph::build_graph(parsed_nodes, parsed_edges),
                        options: parsed_opts,
                    };
                }
            }
        }

        WasmContainer {
            graph: Graph::build_graph(vec![], vec![]),
            options: FDLOptions::default(),
        }
    }

    pub fn run_fdl(&mut self, steps: u32) {
        self.graph.run_fdl(&self.options, steps);
    }
}
