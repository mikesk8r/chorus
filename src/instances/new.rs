use crate::instances::Instance;

/// Creates a new instance in the filesytem.
/// 
/// Returns `true` if the instance was successfully created.
pub fn new_instance(instance: &Instance) -> bool {
    #[cfg(target_os = "windows")]
    let instances_path = env!("LOCALAPPDATA").to_owned() + "\\chorus\\instances";
    #[cfg(all(unix, not(target_os = "macos")))]
    let instances_path = env!("HOME").to_owned() + "/.local/share/chorus/instances";

    #[cfg(windows)]
    let instance_path = instances_path + "\\" + &instance.name;
    #[cfg(unix)]
    let instance_path = instances_path + "/" + &instance.name;

    if std::fs::create_dir_all(&instance_path).is_err() {
        return false;
    }

    let instance_name = instance.name.clone();
    let config = toml::toml! {
        name = instance_name
    };

    #[cfg(windows)]
    if std::fs::write(instance_path + "\\instance.toml", config.to_string()).is_err() {
        return false;
    }
    #[cfg(unix)]
    if std::fs::write(instance_path + "/instance.toml", config.to_string()).is_err() {
        return false;
    }

    true
}
