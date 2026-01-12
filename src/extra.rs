use std::net::SocketAddr;

use crate::model::Color;

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
