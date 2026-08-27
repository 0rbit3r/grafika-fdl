mod fdl_opts;

use crate::graph::Graph;

pub use fdl_opts::FDLOptions;

pub struct NodeInt {
    pub id: usize,
    pub radius: f32,
    pub is_fixed: bool,
    pub forces_x: f32,
    pub forces_y: f32,
}

pub struct EdgeInt {
    pub source_id: usize,
    pub target_id: usize,
    pub length: f32,
    pub weight: f32,
}

impl Graph {
    pub fn run_fdl(&mut self, opts: &FDLOptions, steps: u32) {
        for _ in 0..steps {
            self.compute_forces(opts);
            self.update_positions();
        }
    }

    fn compute_forces(&mut self, opts: &FDLOptions) {
        self.compute_pull(opts);
        self.compute_push(opts);
    }

    fn compute_pull(&mut self, opts: &FDLOptions) {}

    fn compute_push(&mut self, opts: &FDLOptions) {
        let nodes_count = self.nodes.len();

        for a in 0..nodes_count {
            for b in a + 1..nodes_count {
                if self.nodes[a].is_fixed && self.nodes[b].is_fixed {
                    continue;
                }

                let dx = self.positions_x[a] - self.positions_x[b];
                let dy = self.positions_y[a] - self.positions_y[b];
                let border_dist = self.get_border_distance(a, b);
                let center_dist = get_distance(
                    self.positions_x[a],
                    self.positions_y[a],
                    self.positions_x[b],
                    self.positions_y[b],
                );

                let force = push_force(border_dist) - push_force(opts.push_threshold);

                let node_a_fixed = self.nodes[a].is_fixed;
                let node_b_fixed = self.nodes[b].is_fixed;
                self.nodes[a].forces_x += if node_a_fixed {
                    0.0
                } else {
                    dx / center_dist * force
                };
                self.nodes[a].forces_y += if node_a_fixed {
                    0.0
                } else {
                    dy / center_dist * force
                };
                self.nodes[b].forces_x += if node_b_fixed {
                    0.0
                } else {
                    dx / center_dist * force
                };
                self.nodes[b].forces_y += if node_b_fixed {
                    0.0
                } else {
                    dy / center_dist * force
                };
            }
        }
    }

    fn update_positions(&mut self) {
        for i in 0..self.positions_x.len() {
            self.positions_x[i] += self.nodes[i].forces_x;
            self.positions_y[i] += self.nodes[i].forces_y;
        }
    }

    fn get_border_distance(&self, node1_index: usize, node2_index: usize) -> f32 {
        get_distance(
            self.positions_x[node1_index],
            self.positions_y[node1_index],
            self.positions_x[node2_index],
            self.positions_y[node2_index],
        ) - self.nodes[node1_index].radius
            - self.nodes[node2_index].radius
    }
}

fn get_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx.hypot(dy).max(0.01) //prevent division by 0
}

fn push_force(border_dist: f32) -> f32 {
    20.0 / border_dist.max(0.001).sqrt()
}
