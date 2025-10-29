#[derive(Default)]
pub struct Settings {
    // TODO...
}

pub fn get() -> Settings {
    let settings = Settings::default();

    // TODO: macos support?
    #[cfg(target_os = "windows")]
    let file = std::fs::read_to_string(env!("LOCALAPPDATA").to_owned() + "\\chorus\\chorus.toml");
    #[cfg(all(unix, not(target_os = "macos")))]
    let file =
        std::fs::read_to_string(env!("HOME").to_owned() + "/.local/share/chorus/chorus.toml");

    if file.is_ok() {
        let _file: toml::Table = toml::from_str(file.expect("").as_str()).expect("bad config");

        // TODO...
    }
    settings
}
