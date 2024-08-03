pub const CORE_ASAR_FILE: &str = "core.asar";
pub const CORE_ASAR_BACKUP_FILE: &str = "core.asar.backup";

#[cfg(target_os = "windows")]
pub const LOCAL_APP_DATA: &str = "AppData\\Local";
#[cfg(target_os = "linux")]
pub const CONFIG_FOLDER: &str = ".config";

#[cfg(target_os = "windows")]
pub const DISCORD_VARIANTS: [&str; 3] = ["Discord", "DiscordPTB", "DiscordCanary"];
#[cfg(target_os = "linux")]
pub const DISCORD_VARIANTS: [&str; 3] = ["discord", "discordptb", "discordcanary"];