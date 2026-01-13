use std::net::SocketAddr;

use crate::{
    extension::DefaultParse,
    model::{AccessLevel, Color, Player},
};

/// Combines all admin list colors best commands into one function
pub fn admin_list_colors_best(color: Color) {
    crate::admin_list_colors_best_red(color.red);
    crate::admin_list_colors_best_green(color.green);
    crate::admin_list_colors_best_blue(color.blue);
}
/// Combines all admin list colors best commands into one function
pub fn admin_list_colors_worst(color: Color) {
    crate::admin_list_colors_worst_red(color.red);
    crate::admin_list_colors_worst_green(color.green);
    crate::admin_list_colors_worst_blue(color.blue);
}
/// Combines alive_locx and alive locy into one function
pub fn alive_loc(coords: (f32, f32)) {
    crate::alive_locx(coords.0);
    crate::alive_locy(coords.1);
}
/// Combines DEFAULT_KICK_TO SERVER and PORT into one function
pub fn default_kick_to_server_port(server: SocketAddr) {
    crate::default_kick_to_server(server.ip());
    crate::default_kick_to_port(server.port());
}
/// Creates a mapping of usernames to access levels using your server's config files!
pub fn user_levels() -> std::collections::HashMap<Player, AccessLevel> {
    // println!("Starting USER_LEVEL search!");
    let mut map: std::collections::HashMap<Player, AccessLevel> = std::collections::HashMap::new();
    if let Some((_, val)) = std::env::vars().find(|(key, _)| key == "ARMAGETRONAD_PATH_CONFIG") {
        // println!("Key found: {}", val);
        let file_names = ["settings_custom.cfg", "server_info.cfg"];
        for config_dir in val.split(':').map(|x| std::path::Path::new(x)) {
            // println!("Config directory found: {}", config_dir.display());
            for file_name in file_names {
                let path = config_dir.join(file_name);
                if let Ok(config) = std::fs::read_to_string(&path) {
                    // println!("Config found: {}", path.display());
                    for line in config.split('\n') {
                        if line.starts_with("USER_LEVEL") {
                            // println!("Line found: {}", line);
                            let mut splits = line.split(' ');
                            let username = splits.nth(1);
                            let level = splits.next();
                            if let Some(username) = username
                                && let Some(level) = level
                            {
                                // println!("Splits found: ({}, {})", username, level);
                                map.insert(
                                    Player(String::from(username)),
                                    AccessLevel::parse_or_default(level),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    map
}
