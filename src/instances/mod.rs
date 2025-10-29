#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

mod new;
pub use new::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Instance {
    pub name: String,
}

#[derive(Default)]
pub struct InstanceGroup {
    pub name: String,
    pub instances: Vec<Instance>,
    pub instance_groups: Vec<InstanceGroup>,
}

impl InstanceGroup {
    /// Draws this instance group, the instance groups inside it (and their groups and instances), and its own instances.
    pub fn draw_recursive(&self, ui: &mut egui::Ui, selected_instance: Rc<RefCell<Instance>>) {
        for group in &self.instance_groups {
            egui::CollapsingHeader::new(group.name.clone()).show(ui, |ui| {
                group.draw_recursive(ui, Rc::clone(&selected_instance));
            });
        }
        for instance in &self.instances {
            let clicked = {
                let selected = selected_instance.borrow();
                ui.selectable_label(selected.name == instance.name, instance.name.clone())
                    .clicked()
            };

            if clicked {
                *selected_instance.borrow_mut() = instance.clone();
            }
        }
    }
}

/// Returns the full list of instances.
pub fn get() -> InstanceGroup {
    let mut group = InstanceGroup::default();

    #[cfg(target_os = "windows")]
    let instances = std::fs::read_dir(env!("LOCALAPPDATA").to_owned() + "\\chorus\\instances");
    #[cfg(all(unix, not(target_os = "macos")))]
    let instances = std::fs::read_dir(env!("HOME").to_owned() + "/.local/share/chorus/instances");

    if instances.is_ok() {
        group = find(instances.unwrap());
    }

    group
}

/// Returns a list of instances in a folder.
fn find(dir: std::fs::ReadDir) -> InstanceGroup {
    let mut group = InstanceGroup::default();

    for item in dir {
        let item = item.expect("can't read direntry");
        let file_type = item.file_type().expect("cannot access file type of path");

        if file_type.is_file() {
            continue;
        }

        let mut is_instance = false;
        let mut instance = Instance::default();
        for item in std::fs::read_dir(item.path()).unwrap() {
            let item = item.expect("can't read direntry");
            let file_type = item.file_type().expect("cannot access file type of path");

            if file_type.is_dir() {
                continue;
            }

            if let Some(file_name) = item.path().file_name() {
                if file_name == "instance.toml" {
                    is_instance = true;

                    let parsed: toml::Table = toml::from_str(
                        std::fs::read_to_string(item.path())
                            .expect("cannot read instance.toml!")
                            .as_str(),
                    )
                    .expect("bad instance.toml");

                    if let Some(name) = parsed.get("name") {
                        instance.name = name.to_string();
                    }
                }
            }
        }

        if is_instance {
            group.instances.push(instance);
        } else {
            drop(instance);
            let name = item.path();
            let name = name.file_name().unwrap();
            let mut subgroup = find(std::fs::read_dir(item.path()).unwrap());
            subgroup.name = String::from(name.to_str().unwrap());
            group.instance_groups.push(subgroup);
        }
    }

    group
}
