use bevy_settings_lib::{FormatKind, SettingsPluginConfig, SettingsStorage};

pub fn get_settings_config() -> SettingsPluginConfig {
    SettingsPluginConfig {
        format: FormatKind::Toml,
        company: "MyCompany".into(),
        project: "RustTest".into(),
        file_name: Some("config".into()),
        storage: SettingsStorage::SystemConfigDir,
        ..Default::default()
    }
}