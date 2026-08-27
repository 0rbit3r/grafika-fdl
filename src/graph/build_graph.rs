use super::{Edge, EdgeInt, Graph, Node, NodeInt};
use std::collections::HashMap;
use std::rc::Rc;

impl Graph {
    /// Will build the graph from given nodes and edges
    /// edges referencing non-provided nodes are discarded
    /// duplicate nodes and edges are not handled!
    pub fn build_graph(nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
        let mut original_ids = vec![];
        let mut ids_map = HashMap::new();
        let mut int_nodes = vec![];
        let mut int_edges = vec![];
        let mut positions_x= vec![];
        let mut positions_y= vec![];

        for (i, node) in nodes.iter().enumerate() {
            let id_rc = Rc::from(node.id.as_str());
            ids_map.insert(Rc::clone(&id_rc), i);
            original_ids.push(id_rc);

            int_nodes.push(NodeInt {
                id: i,
                radius: node.radius,
                is_fixed: node.is_fixed,
                forces_x: 0.0,
                forces_y: 0.0,
            });
            positions_x.push(node.x);
            positions_y.push(node.y);
        }
        for edge in edges {
            if let Some(&source_id_int) = ids_map.get(edge.source_id.as_str()) {
                if let Some(&target_id_int) = ids_map.get(edge.target_id.as_str()) {
                    int_edges.push(EdgeInt {
                        length: edge.length,
                        weight: edge.weight,
                        source_id: source_id_int,
                        target_id: target_id_int,
                    });
                }
            }
        }

        Graph {
            nodes: int_nodes,
            edges: int_edges,
            original_ids,
            ids_map,
            positions_x,
            positions_y
        }
    }
}

#[cfg(test)]
mod test {
    use crate::graph::{Edge, Graph, Node};

    fn test_nodes() -> Vec<Node> {
        vec![
            Node {
                id: "id1".to_string(),
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                is_fixed: false,
            },
            Node {
                id: "id2".to_string(),
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                is_fixed: false,
            },
            Node {
                id: "id3".to_string(),
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                is_fixed: false,
            },
        ]
    }

    #[test]
    fn simple_path() {
        let graph = Graph::build_graph(
            test_nodes(),
            vec![
                Edge {
                    source_id: "id1".to_string(),
                    target_id: "id2".to_string(),
                    weight: 1.0,
                    length: 300.0,
                },
                Edge {
                    source_id: "id2".to_string(),
                    target_id: "id3".to_string(),
                    weight: 1.0,
                    length: 300.0,
                },
            ],
        );

        assert_eq!(3, graph.nodes.len());
        assert_eq!(2, graph.edges.len());
    }
    #[test]
    fn wrong_edge_reference() {
        let graph = Graph::build_graph(
            test_nodes(),
            vec![
                Edge {
                    source_id: "id1".to_string(),
                    target_id: "blahblah".to_string(),
                    weight: 1.0,
                    length: 300.0,
                },
                Edge {
                    source_id: "something".to_string(),
                    target_id: "id3".to_string(),
                    weight: 1.0,
                    length: 300.0,
                },
            ],
        );

        assert_eq!(3, graph.nodes.len());
        assert_eq!(0, graph.edges.len());
    }
}
