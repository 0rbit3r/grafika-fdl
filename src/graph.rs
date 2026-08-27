use std::collections::HashMap;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

use crate::fdl::{NodeInt, EdgeInt};

mod build_graph;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub is_fixed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub source_id: String,
    pub target_id: String,
    pub length: f32,
    pub weight: f32,
}

pub struct Graph {
    pub original_ids: Vec<Rc<str>>,
    pub ids_map: HashMap<Rc<str>, usize>,

    // positions are declared like this to make it possible to read them quickly
    // across wasm boundary. (mávám rukama)
    pub positions_x: Vec<f32>,
    pub positions_y: Vec<f32>,

    pub nodes: Vec<NodeInt>,
    pub edges: Vec<EdgeInt>,
}

