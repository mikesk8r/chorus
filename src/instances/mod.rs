#![allow(dead_code)]

#[derive(Default)]
pub struct Instance {
    name: String,
}

#[derive(Default)]
pub struct InstanceGroup {
    name: String,
    instances: Vec<Instance>,
    instance_groups: Vec<InstanceGroup>,
}

impl InstanceGroup {
    pub fn draw_recursive(&self, _ui: &mut egui::Ui) {
        // TODO
    }
}

pub fn get() -> InstanceGroup {
    let group = InstanceGroup::default();
    // TODO
    group
}
