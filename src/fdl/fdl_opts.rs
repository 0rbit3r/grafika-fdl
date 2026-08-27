use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FDLOptions {
    pub pull_force_multiplier: f32,

    pub push_force_multiplier: f32,
    pub push_threshold: f32,

    pub center_pull_enabled: bool,
    pub center_pull_border_radius: f32,
}

impl Default for FDLOptions {
    fn default() -> FDLOptions {
        FDLOptions {
            pull_force_multiplier: 1.0,
            push_force_multiplier: 1.0,
            push_threshold: 5.0,
            center_pull_border_radius: 5.0,
            center_pull_enabled: true,
        }
    }
}
