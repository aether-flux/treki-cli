use directories::ProjectDirs;

pub fn get_config_path () -> Option<std::path::PathBuf> {
    ProjectDirs::from("com", "treki", "treki-cli")
        .map(|proj| proj.config_dir().join("auth_token.json"))
}
