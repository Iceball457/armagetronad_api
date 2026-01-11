use std::{io::Read, net::IpAddr, net::SocketAddr, path::Path};

pub mod command;
pub use command::*;

pub mod data;
pub use data::*;

pub mod ladderlog;
pub use ladderlog::*;

mod extension;
use extension::*;

/// Parses ladder log entries, and runs the closure with each new entry as input.
/// This function blocks until each entry is written.
pub fn run(mut callback: impl FnMut(LadderLogEntry)) {
    let mut encoding = String::from("latin1");
    loop {
        let mut buf = String::new();
        if encoding == "utf8".to_owned() {
            _ = std::io::stdin().read_line(&mut buf);
        }
        if encoding == "latin1".to_owned() {
            let mut bytes = Vec::new();
            let mut stdin = std::io::stdin().lock();
            loop {
                let mut byte = [0u8; 1];
                if stdin.read_exact(&mut byte).is_err() {
                    return; // EOF
                }
                bytes.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&bytes);
            buf.push_str(&decoded);
        }
        if let Some(entry) = LadderLogEntry::parse(&buf) {
            wait_for_external_script(true);
            if let LadderLogEntry::Encoding(ref new_encoding) = entry {
                encoding.clear();
                encoding.push_str(new_encoding);
            }
            callback(entry);
            wait_for_external_script(false);
        }
    }
}

/// ACCESS_LEVEL: Changes the access level of a configuration item to make it available to lower ranked users
pub fn access_level(config_item: &Command, access_level: &AccessLevel) {
    println!("{} {} {}", Command::AccessLevel, config_item, access_level);
}
/// ACCESS_LEVEL_ANNOUNCE_LOGIN: Maximal access level that determines if a player's login/logout message can be announced.
pub fn access_level_announce_login(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelAnnounceLogin, access_level);
}
/// ACCESS_LEVEL_AUTOKICK_IMMUNITY: Minimal access level to be protected against autokicks.
pub fn access_level_autokick_immunity(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelAutokickImmunity, access_level);
}
/// ACCESS_LEVEL_CHAT: Minimal access level for chatting.
pub fn access_level_chat(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelChat, access_level);
}
/// ACCESS_LEVEL_CHAT_TIMEOUT: Time in seconds between public announcements that someone wants to chat, but can't. Set to 0 to disable the public warnings.
pub fn access_level_chat_timeout(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelChatTimeout, access_level);
}
/// ACCESS_LEVEL_HIDE_OF: Minimal access level to be able to hide it's own user account information.
pub fn access_level_hide_of(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelHideOf, access_level);
}
/// ACCESS_LEVEL_HIDE_TO: Minimal access level to see everyone's user account information.
pub fn access_level_hide_to(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelHideTo, access_level);
}
/// ACCESS_LEVEL_IPS: Minimal access level you need for seeing IPs of other players in the /players command.
pub fn access_level_ips(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelIps, access_level);
}
/// ACCESS_LEVEL_LIST_ADMINS: Access level required to be able to use the "/admins" command.
pub fn access_level_list_admins(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelListAdmins, access_level);
}
/// ACCESS_LEVEL_LIST_ADMINS_SEE_EVERYONE: Users with this access level or better will be able to list any configured admin, regardless of ADMIN_LIST_MIN_ACCESS_LEVEL.
pub fn access_level_list_admins_see_everyone(access_level: &AccessLevel) {
    println!(
        "{} {}",
        Command::AccessLevelListAdminsSeeEveryone,
        access_level
    );
}
/// ACCESS_LEVEL_NVER: Minimal access level you need for seeing Network versions/strings from other players in /players.
pub fn access_level_nver(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelNver, access_level);
}
/// ACCESS_LEVEL_PLAY: Minimal access level for playing
pub fn access_level_play(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelPlay, access_level);
}
/// ACCESS_LEVEL_PLAY_SLIDERS: The access level required to play will only slide up if at least this many players of a higher level are online.
pub fn access_level_play_sliders(num_players: u8) {
    println!("{} {}", Command::AccessLevelPlaySliders, num_players);
}
/// ACCESS_LEVEL_PLAY_SLIDING: Sliding minimal access level for playing; if enough players of a higher access level than given by ACCESS_LEVEL_PLAY are online, their level will be the minimal level for play; however, it will never be higher than ACCESS_LEVEL_PLAY_SLIDING.
pub fn access_level_play_sliding(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelPlaySliding, access_level);
}
/// ACCESS_LEVEL_RTFM: Minimal access level for /teach or /rtfm command.
pub fn access_level_rtfm(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelRtfm, access_level);
}
/// ACCESS_LEVEL_SHOUT: Minimal access level for shouting.
pub fn access_level_shout(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelShout, access_level);
}
/// ACCESS_LEVEL_SHUFFLE_UP: Minimal access level for shuffling up
pub fn access_level_shuffle_up(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelShuffleUp, access_level);
}
/// ACCESS_LEVEL_SPY_MSG: Minimal access level you need for seeing /msg messages directed to others.
pub fn access_level_spy_msg(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelSpyMsg, access_level);
}
/// ACCESS_LEVEL_SPY_TEAM: Minimal access level you need for seeing /team messages as a spectator.
pub fn access_level_spy_team(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelSpyTeam, access_level);
}
/// ACCESS_LEVEL_VOTE_COMMAND: Minimal access level required to issue command votes.
pub fn access_level_vote_command(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelVoteCommand, access_level);
}
/// ACCESS_LEVEL_VOTE_COMMAND_EXECUTE: Minimal access level successful command votes will be executed at.
pub fn access_level_vote_command_execute(access_level: &AccessLevel) {
    println!(
        "{} {}",
        Command::AccessLevelVoteCommandExecute,
        access_level
    );
}
/// ACCESS_LEVEL_VOTE_INCLUDE: Minimal access level required to issue include votes.
pub fn access_level_vote_include(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelVoteInclude, access_level);
}
/// ACCESS_LEVEL_VOTE_INCLUDE_EXECUTE: Minimal access level successful include votes will be executed at.
pub fn access_level_vote_include_execute(access_level: &AccessLevel) {
    println!(
        "{} {}",
        Command::AccessLevelVoteIncludeExecute,
        access_level
    );
}
/// ACCESS_LEVEL_VOTE_KICK: Minimal access level required to issue kick votes.
pub fn access_level_vote_kick(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelVoteKick, access_level);
}
/// ACCESS_LEVEL_VOTE_SILENCE: Minimal access level required to issue silence and voice votes.
pub fn access_level_vote_silence(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelVoteSilence, access_level);
}
/// ACCESS_LEVEL_VOTE_SUSPEND: Minimal access level required to issue suspend votes.
pub fn access_level_vote_suspend(access_level: &AccessLevel) {
    println!("{} {}", Command::AccessLevelVoteSuspend, access_level);
}
/// ADD_HELP_TOPIC: Add a new help topic to be used with /help.
/// Usage: ADD_HELP_TOPIC \<topic> \<short description> \<text>
pub fn add_help_topic(topic: &str, short_desc: &str, text: &str) {
    println!(
        "{} {} {} {}",
        Command::AddHelpTopic,
        topic,
        short_desc,
        text
    );
}
/// ADD_MASTER_SERVER: Announce this server to another master server.
/// Usage: ADD_MASTER_SERVER \<host> \[\<port> (default=4533)].
pub fn add_master_server(socket: SocketAddr) {
    println!(
        "{} {} {}",
        Command::AddMasterServer,
        socket.ip(),
        socket.port()
    );
}
/// ADMINS: Lists the server admins. You should use /admins or /listadmins instead of this.
pub fn admins() {
    println!("{}", Command::Admins);
}
/// ADMIN_LIST_COLORS_BEST_BLUE: Blue color component to the best access level listed by /admins
pub fn admin_list_colors_best_blue(red: u8) {
    println!("{} {}", Command::AdminListColorsBestRed, red);
}
/// ADMIN_LIST_COLORS_BEST_GREEN: Green color component to the best access level listed by /admins
pub fn admin_list_colors_best_green(green: u8) {
    println!("{} {}", Command::AdminListColorsBestGreen, green);
}
/// ADMIN_LIST_COLORS_BEST_RED: Red color component to the best access level listed by /admins
pub fn admin_list_colors_best_red(blue: u8) {
    println!("{} {}", Command::AdminListColorsBestBlue, blue);
}
/// Combines all admin list colors best commands into one function
pub fn admin_list_colors_best(color: Color) {
    admin_list_colors_best_red(color.red);
    admin_list_colors_best_green(color.green);
    admin_list_colors_best_blue(color.blue);
}
/// ADMIN_LIST_COLORS_WORST_BLUE: Blue color component to the worst access level listed by /admins
pub fn admin_list_colors_worst_blue(red: u8) {
    println!("{} {}", Command::AdminListColorsWorstRed, red);
}
/// ADMIN_LIST_COLORS_WORST_GREEN: Green color component to the worst access level listed by /admins
pub fn admin_list_colors_worst_green(green: u8) {
    println!("{} {}", Command::AdminListColorsWorstGreen, green);
}
/// ADMIN_LIST_COLORS_WORST_RED: Red color component to the worst access level listed by /admins
pub fn admin_list_colors_worst_red(blue: u8) {
    println!("{} {}", Command::AdminListColorsWorstBlue, blue);
}
/// Combines all admin list colors best commands into one function
pub fn admin_list_colors_worst(color: Color) {
    admin_list_colors_worst_red(color.red);
    admin_list_colors_worst_green(color.green);
    admin_list_colors_worst_blue(color.blue);
}
/// ADMIN_LIST_MIN_ACCESS_LEVEL: Minimal access level to be shown in /admins
pub fn admin_list_min_access_level(access_level: &AccessLevel) {
    println!("{} {}", Command::AdminListMinAccessLevel, access_level);
}
/// AI_IQ: IQ of the AI opponents (0-100)
pub fn ai_iq(iq: u8) {
    println!("{} {}", Command::AiIq, iq);
}
/// ALIVE_LOCX: Horizontal position of the alive headcount display
pub fn alive_locx(x: f32) {
    println!("{} {}", Command::AliveLocx, x);
}
/// ALIVE_LOCY: Vertical position of the alive headcount display
pub fn alive_locy(y: f32) {
    println!("{} {}", Command::AliveLocy, y);
}
/// Combines alive_locx and alive locy into one function
pub fn alive_loc(coords: (f32, f32)) {
    alive_locx(coords.0);
    alive_locy(coords.1);
}
/// ALIVE_SIZE: Size of the alive headcount display
pub fn alive_size(size: f32) {
    println!("{} {}", Command::AliveSize, size)
}
/// ALLOW_CAM_1_0: Allow/forbid the different camera modes
pub fn allow_cam_1_0(allowed: bool) {
    println!("{} {}", Command::AllowCam1_0, allowed);
}
/// ALLOW_CAM_1_1: Allow/forbid the different camera modes
pub fn allow_cam_1_1(allowed: bool) {
    println!("{} {}", Command::AllowCam1_1, allowed);
}
/// ALLOW_CAM_1_2: Allow/forbid the different camera modes
pub fn allow_cam_1_2(allowed: bool) {
    println!("{} {}", Command::AllowCam1_2, allowed);
}
/// ALLOW_CAM_1_3: Allow/forbid the different camera modes
pub fn allow_cam_1_3(allowed: bool) {
    println!("{} {}", Command::AllowCam1_3, allowed);
}
/// ALLOW_CAM_1_4: Allow/forbid the different camera modes
pub fn allow_cam_1_4(allowed: bool) {
    println!("{} {}", Command::AllowCam1_4, allowed);
}
/// ALLOW_CAM_1_5: Allow/forbid the different camera modes
pub fn allow_cam_1_5(allowed: bool) {
    println!("{} {}", Command::AllowCam1_5, allowed);
}
/// ALLOW_CAM_1_6: Allow/forbid the different camera modes
pub fn allow_cam_1_6(allowed: bool) {
    println!("{} {}", Command::AllowCam1_6, allowed);
}
/// ALLOW_CAM_2_0: Allow/forbid the different camera modes
pub fn allow_cam_2_0(allowed: bool) {
    println!("{} {}", Command::AllowCam2_0, allowed);
}
/// ALLOW_CAM_2_1: Allow/forbid the different camera modes
pub fn allow_cam_2_1(allowed: bool) {
    println!("{} {}", Command::AllowCam2_1, allowed);
}
/// ALLOW_CAM_2_2: Allow/forbid the different camera modes
pub fn allow_cam_2_2(allowed: bool) {
    println!("{} {}", Command::AllowCam2_2, allowed);
}
/// ALLOW_CAM_2_3: Allow/forbid the different camera modes
pub fn allow_cam_2_3(allowed: bool) {
    println!("{} {}", Command::AllowCam2_3, allowed);
}
/// ALLOW_CAM_2_4: Allow/forbid the different camera modes
pub fn allow_cam_2_4(allowed: bool) {
    println!("{} {}", Command::AllowCam2_4, allowed);
}
/// ALLOW_CAM_2_5: Allow/forbid the different camera modes
pub fn allow_cam_2_5(allowed: bool) {
    println!("{} {}", Command::AllowCam2_5, allowed);
}
/// ALLOW_CAM_2_6: Allow/forbid the different camera modes
pub fn allow_cam_2_6(allowed: bool) {
    println!("{} {}", Command::AllowCam2_6, allowed);
}
/// ALLOW_CAM_3_0: Allow/forbid the different camera modes
pub fn allow_cam_3_0(allowed: bool) {
    println!("{} {}", Command::AllowCam3_0, allowed);
}
/// ALLOW_CAM_3_1: Allow/forbid the different camera modes
pub fn allow_cam_3_1(allowed: bool) {
    println!("{} {}", Command::AllowCam3_1, allowed);
}
/// ALLOW_CAM_3_2: Allow/forbid the different camera modes
pub fn allow_cam_3_2(allowed: bool) {
    println!("{} {}", Command::AllowCam3_2, allowed);
}
/// ALLOW_CAM_3_3: Allow/forbid the different camera modes
pub fn allow_cam_3_3(allowed: bool) {
    println!("{} {}", Command::AllowCam3_3, allowed);
}
/// ALLOW_CAM_3_4: Allow/forbid the different camera modes
pub fn allow_cam_3_4(allowed: bool) {
    println!("{} {}", Command::AllowCam3_4, allowed);
}
/// ALLOW_CAM_3_5: Allow/forbid the different camera modes
pub fn allow_cam_3_5(allowed: bool) {
    println!("{} {}", Command::AllowCam3_5, allowed);
}
/// ALLOW_CAM_3_6: Allow/forbid the different camera modes
pub fn allow_cam_3_6(allowed: bool) {
    println!("{} {}", Command::AllowCam3_6, allowed);
}
/// ALLOW_CAM_4_0: Allow/forbid the different camera modes
pub fn allow_cam_4_0(allowed: bool) {
    println!("{} {}", Command::AllowCam4_0, allowed);
}
/// ALLOW_CAM_4_1: Allow/forbid the different camera modes
pub fn allow_cam_4_1(allowed: bool) {
    println!("{} {}", Command::AllowCam4_1, allowed);
}
/// ALLOW_CAM_4_2: Allow/forbid the different camera modes
pub fn allow_cam_4_2(allowed: bool) {
    println!("{} {}", Command::AllowCam4_2, allowed);
}
/// ALLOW_CAM_4_3: Allow/forbid the different camera modes
pub fn allow_cam_4_3(allowed: bool) {
    println!("{} {}", Command::AllowCam4_3, allowed);
}
/// ALLOW_CAM_4_4: Allow/forbid the different camera modes
pub fn allow_cam_4_4(allowed: bool) {
    println!("{} {}", Command::AllowCam4_4, allowed);
}
/// ALLOW_CAM_4_5: Allow/forbid the different camera modes
pub fn allow_cam_4_5(allowed: bool) {
    println!("{} {}", Command::AllowCam4_5, allowed);
}
/// ALLOW_CAM_4_6: Allow/forbid the different camera modes
pub fn allow_cam_4_6(allowed: bool) {
    println!("{} {}", Command::AllowCam4_6, allowed);
}
/// ALLOW_CONTROL_DURING_CHAT: If set to 1, this allows a player to issue cycle and camera control commands during chat (losing the chatbot and the yellow chat pyramid).
pub fn allow_control_during_chat(allowed: bool) {
    println!("{} {}", Command::AllowControlDuringChat, allowed);
}
/// ALLOW_ENEMIES_SAME_CLIENT: If set to 1, this allows two players that play on the same client to fight for points with each other.
pub fn allow_enemies_same_client(allowed: bool) {
    println!("{} {}", Command::AllowEnemiesSameClient, allowed);
}
/// ALLOW_ENEMIES_SAME_IP: If set to 1, this allows two players that apparently come from the same machine to fight for points with each other.
pub fn allow_enemies_same_ip(allowed: bool) {
    println!("{} {}", Command::AllowEnemiesSameIp, allowed);
}
/// ALLOW_IMPOSTERS: If set to 1, players with identical names are tolerated. If set to 0, all but one will be renamed.
pub fn allow_imposters(allowed: bool) {
    println!("{} {}", Command::AllowImposters, allowed);
}
/// ALLOW_RENAME_PLAYER: Gives the given player the ability to rename.
pub fn allow_rename_player(player: &Player, allowed: bool) {
    println!("{} {} {}", Command::AllowRenamePlayer, player, allowed);
}
/// ALLOW_TEAM_CHANGE: If set to 1, all players can change teams. If set to 0, players can only change teams if they've been specifically allowed to by ALLOW_TEAM_CHANGE_PLAYER
pub fn allow_team_change(allowed: bool) {
    println!("{} {}", Command::AllowTeamChange, allowed);
}
/// ALLOW_TEAM_CHANGE_PLAYER: Allow a specific player to change teams even if ALLOW_TEAM_CHANGE is disabled
pub fn allow_team_change_player(player: &Player, allowed: bool) {
    println!("{} {} {}", Command::AllowTeamChangePlayer, player, allowed);
}
/// ALLOW_TEAM_NAME_COLOR: Allow a team to be named after a color
pub fn allow_team_name_color(allowed: bool) {
    println!("{} {}", Command::AllowTeamNameColor, allowed.byte())
}
/// ALLOW_TEAM_NAME_PLAYER: Allow a team to be named after the leading player
pub fn allow_team_name_player(allowed: bool) {
    println!("{} {}", Command::AllowTeamNamePlayer, allowed.byte())
}
/// ALLOW_VOTING: If set to 1, voting will be allowed for players.
pub fn allow_voting(allowed: bool) {
    println!("{} {}", Command::AllowVoting, allowed);
}
/// ALLOW_VOTING_SPECTATOR: If set to 1, voting will be allowed for spectators.
pub fn allow_voting_spectator(allowed: bool) {
    println!("{} {}", Command::AllowVotingSpectator, allowed);
}
/// ALPHA_BLEND: Enable alpha blending
pub fn alpha_blend(enabled: bool) {
    println!("{} {}", Command::AlphaBlend, enabled.byte());
}
/// ANTI_SPOOF: If set to 1, checks connecting clients for spoofed IPs. Only clients passing a connectivity test are allowed in. This is done in turtle mode automatically, but may be useful to have on at all times.
pub fn anti_spoof(enabled: bool) {
    println!("{} {}", Command::AntiSpoof, enabled.byte());
}
/// ARENA_AXES: In how many directions a cycle can turn 4 is the default, 6 is hexatron
pub fn arena_axes(axes: u8) {
    println!("{} {}", Command::ArenaAxes, axes);
}
/// ARENA_AXES_OVERRIDE: Block out older clients when ARENA_AXES differs from its default?
pub fn arena_axes_override(block_old_clients: bool) {
    println!(
        "{} {}",
        Command::ArenaAxesOverride,
        block_old_clients.byte()
    );
}
/// ARMAGETRON_LAST_SCREENMODE: Last screen resolution
pub fn armagetron_last_screenmode(screen: f32) {
    println!("{} {}", Command::ArmagetronLastScreenmode, screen);
}
/// ARMAGETRON_LAST_WINDOWSIZE: Last Window size
pub fn armagetron_last_windowsize(window: f32) {
    println!("{} {}", Command::ArmagetronLastWindowsize, window);
}
/// ARMAGETRON_SCREENMODE: Screen resolution
pub fn armagetron_screenmode(screen: f32) {
    println!("{} {}", Command::ArmagetronScreenmode, screen);
}
/// ARMAGETRON_VSYNC: What to do with the monitor's vertical sync
pub fn armagetron_vsync(enabled: bool) {
    println!("{} {}", Command::ArmagetronVsync, enabled.byte());
}
/// ARMAGETRON_VSYNC_LAST: armagetron_vsync_last_help
pub fn armagetron_vsync_last(enabled: bool) {
    println!("{} {}", Command::ArmagetronVsyncLast, enabled.byte());
}
/// ARMAGETRON_WINDOWSIZE: Window size
pub fn armagetron_windowsize(window: f32) {
    println!("{} {}", Command::ArmagetronWindowsize, window);
}
/// AUTHORITY_BLACKLIST: Comma separated list of authorities your server should refuse to query.
pub fn authority_blacklist(blacklist: Vec<Authority>) {
    println!(
        "{} {}",
        Command::AuthorityBlacklist,
        match blacklist
            .iter()
            .map(|x| x.0.clone())
            .reduce(|mut accum, next| {
                accum.push_str(&next);
                accum
            }) {
            Some(blacklist) => blacklist,
            None => String::new(),
        }
    );
}
/// AUTHORITY_LEVEL: Changes the access level for all users from the same authority. Mainly only useful for private authorities.
pub fn authority_level(authority: Authority, access_level: &AccessLevel) {
    println!("{} {} {}", Command::AuthorityLevel, authority, access_level);
}
/// AUTHORITY_WHITELIST: If non-empty, only authorities on this comma separated list will be queried by your server.
pub fn authority_whitelist(whitelist: Vec<Authority>) {
    println!(
        "{} {}",
        Command::AuthorityBlacklist,
        match whitelist
            .iter()
            .map(|x| x.0.clone())
            .reduce(|mut accum, next| {
                accum.push_str(&next);
                accum
            }) {
            Some(blacklist) => blacklist,
            None => String::new(),
        }
    );
}
/// AUTO_AIS: Automatically spawn AI players?
pub fn auto_ais(enabled: bool) {
    println!("{} {}", Command::AutoAis, enabled)
}
/// AUTO_INCAM_1: Automatically switch to internal camera in a maze
pub fn auto_incam_1(enabled: bool) {
    println!("{} {}", Command::AutoIncam1, enabled.byte());
}
/// AUTO_INCAM_2: Automatically switch to internal camera in a maze
pub fn auto_incam_2(enabled: bool) {
    println!("{} {}", Command::AutoIncam2, enabled.byte());
}
/// AUTO_INCAM_3: Automatically switch to internal camera in a maze
pub fn auto_incam_3(enabled: bool) {
    println!("{} {}", Command::AutoIncam3, enabled.byte());
}
/// AUTO_INCAM_4: Automatically switch to internal camera in a maze
pub fn auto_incam_4(enabled: bool) {
    println!("{} {}", Command::AutoIncam4, enabled.byte());
}
/// AUTO_IQ: Automatically adjust AI IQ?
pub fn auto_iq() {
    todo!();
}
/// AUTO_LOGIN_1: Should this player automatically request authentication?
pub fn auto_login_1(enabled: bool) {
    println!("{} {}", Command::AutoLogin1, enabled.byte());
}
/// AUTO_LOGIN_2: Should this player automatically request authentication?
pub fn auto_login_2(enabled: bool) {
    println!("{} {}", Command::AutoLogin2, enabled.byte());
}
/// AUTO_LOGIN_3: Should this player automatically request authentication?
pub fn auto_login_3(enabled: bool) {
    println!("{} {}", Command::AutoLogin3, enabled.byte());
}
/// AUTO_LOGIN_4: Should this player automatically request authentication?
pub fn auto_login_4(enabled: bool) {
    println!("{} {}", Command::AutoLogin4, enabled.byte());
}
/// AUTO_TEAM: Flag indicating whether players should be put into teams automatically.
pub fn auto_team(enabled: bool) {
    println!("{} {}", Command::AutoLogin4, enabled.byte());
}
/// AUTO_TEAM_SPEC_SPAM: If set to 0, spectators won't be announced when joining or leaving, provided AUTO_TEAM is set to 0.
pub fn auto_team_spec_spam(enabled: bool) {
    println!("{} {}", Command::AutoTeamSpecSpam, enabled.byte());
}
/// AXES_INDICATOR: Should the Axis Indicator be rendered?
pub fn axes_indicator(enabled: bool) {
    println!("{} {}", Command::AxesIndicator, enabled.byte());
}
/// BACKWARD_COMPATIBILITY: Maximum number of old protocol versions to support.
pub fn backward_compatibility() {
    todo!();
}
/// BAN: Bans the specified player from the server (kicks them first) for a variable time in minutes.
pub fn ban() {
    todo!();
}
/// BAN_IP: Bans the specified IP address from the server for a variable time.
pub fn ban_ip() {
    todo!();
}
/// BAN_LIST: Prints a list of currently banned IPs.
pub fn ban_list() {
    todo!();
}
/// BAN_USER: Allows to ban players based on their authentication ID.
pub fn ban_user() {
    todo!();
}
/// BAN_USER_LIST: Gives a list of banned users.
pub fn ban_user_list() {
    todo!();
}
/// BIG_BROTHER: Did we already send the big brother information?
pub fn big_brother() {
    todo!();
}
/// BOOKMARK_0_ADDRESS: Server address of the 1st server bookmark
pub fn bookmark_0_address() {
    todo!();
}
/// BOOKMARK_0_NAME: Name of the 1st server bookmark
pub fn bookmark_0_name() {
    todo!();
}
/// BOOKMARK_0_PORT: Server port of the 1st server bookmark
pub fn bookmark_0_port() {
    todo!();
}
/// BOOKMARK_10_ADDRESS: Server address of the 11th server bookmark
pub fn bookmark_10_address() {
    todo!();
}
/// BOOKMARK_10_NAME: Name of the 11th server bookmark
pub fn bookmark_10_name() {
    todo!();
}
/// BOOKMARK_10_PORT: Server port of the 11th server bookmark
pub fn bookmark_10_port() {
    todo!();
}
/// BOOKMARK_1_ADDRESS: Server address of the 2nd server bookmark
pub fn bookmark_1_address() {
    todo!();
}
/// BOOKMARK_1_NAME: Name of the 2nd server bookmark
pub fn bookmark_1_name() {
    todo!();
}
/// BOOKMARK_1_PORT: Server port of the 2nd server bookmark
pub fn bookmark_1_port() {
    todo!();
}
/// BOOKMARK_2_ADDRESS: Server address of the 3rd server bookmark
pub fn bookmark_2_address() {
    todo!();
}
/// BOOKMARK_2_NAME: Name of the 3rd server bookmark
pub fn bookmark_2_name() {
    todo!();
}
/// BOOKMARK_2_PORT: Server port of the 3rd server bookmark
pub fn bookmark_2_port() {
    todo!();
}
/// BOOKMARK_3_ADDRESS: Server address of the 4th server bookmark
pub fn bookmark_3_address() {
    todo!();
}
/// BOOKMARK_3_NAME: Name of the 4th server bookmark
pub fn bookmark_3_name() {
    todo!();
}
/// BOOKMARK_3_PORT: Server port of the 4th server bookmark
pub fn bookmark_3_port() {
    todo!();
}
/// BOOKMARK_4_ADDRESS: Server address of the 5th server bookmark
pub fn bookmark_4_address() {
    todo!();
}
/// BOOKMARK_4_NAME: Name of the 5th server bookmark
pub fn bookmark_4_name() {
    todo!();
}
/// BOOKMARK_4_PORT: Server port of the 5th server bookmark
pub fn bookmark_4_port() {
    todo!();
}
/// BOOKMARK_5_ADDRESS: Server address of the 6th server bookmark
pub fn bookmark_5_address() {
    todo!();
}
/// BOOKMARK_5_NAME: Name of the 6th server bookmark
pub fn bookmark_5_name() {
    todo!();
}
/// BOOKMARK_5_PORT: Server port of the 6th server bookmark
pub fn bookmark_5_port() {
    todo!();
}
/// BOOKMARK_6_ADDRESS: Server address of the 7th server bookmark
pub fn bookmark_6_address() {
    todo!();
}
/// BOOKMARK_6_NAME: Name of the 7th server bookmark
pub fn bookmark_6_name() {
    todo!();
}
/// BOOKMARK_6_PORT: Server port of the 7th server bookmark
pub fn bookmark_6_port() {
    todo!();
}
/// BOOKMARK_7_ADDRESS: Server address of the 8th server bookmark
pub fn bookmark_7_address() {
    todo!();
}
/// BOOKMARK_7_NAME: Name of the 8th server bookmark
pub fn bookmark_7_name() {
    todo!();
}
/// BOOKMARK_7_PORT: Server port of the 8th server bookmark
pub fn bookmark_7_port() {
    todo!();
}
/// BOOKMARK_8_ADDRESS: Server address of the 9th server bookmark
pub fn bookmark_8_address() {
    todo!();
}
/// BOOKMARK_8_NAME: Name of the 9th server bookmark
pub fn bookmark_8_name() {
    todo!();
}
/// BOOKMARK_8_PORT: Server port of the 9th server bookmark
pub fn bookmark_8_port() {
    todo!();
}
/// BOOKMARK_9_ADDRESS: Server address of the 10th server bookmark
pub fn bookmark_9_address() {
    todo!();
}
/// BOOKMARK_9_NAME: Name of the 10th server bookmark
pub fn bookmark_9_name() {
    todo!();
}
/// BOOKMARK_9_PORT: Server port of the 10th server bookmark
pub fn bookmark_9_port() {
    todo!();
}
/// BOOKMARK__MASTER0_ADDRESS: Address of the first subculture master
pub fn bookmark_master0_address() {
    todo!();
}
/// BOOKMARK__MASTER0_NAME: Name of the first subculture
pub fn bookmark_master0_name() {
    todo!();
}
/// BOOKMARK__MASTER0_PORT: Port of the first subculture master
pub fn bookmark_master0_port() {
    todo!();
}
/// BOOKMARK__MASTER10_ADDRESS: Address of the 11th subculture master
pub fn bookmark_master10_address() {
    todo!();
}
/// BOOKMARK__MASTER10_NAME: Name of the 11th subculture
pub fn bookmark_master10_name() {
    todo!();
}
/// BOOKMARK__MASTER10_PORT: Port of the 11th subculture master
pub fn bookmark_master10_port() {
    todo!();
}
/// BOOKMARK__MASTER1_ADDRESS: Address of the second subculture master
pub fn bookmark_master1_address() {
    todo!();
}
/// BOOKMARK__MASTER1_NAME: Name of the second subculture
pub fn bookmark_master1_name() {
    todo!();
}
/// BOOKMARK__MASTER1_PORT: Port of the second subculture master
pub fn bookmark_master1_port() {
    todo!();
}
/// BOOKMARK__MASTER2_ADDRESS: Address of the 3rd subculture master
pub fn bookmark_master2_address() {
    todo!();
}
/// BOOKMARK__MASTER2_NAME: Name of the 3rd subculture
pub fn bookmark_master2_name() {
    todo!();
}
/// BOOKMARK__MASTER2_PORT: Port of the 3rd subculture master
pub fn bookmark_master2_port() {
    todo!();
}
/// BOOKMARK__MASTER3_ADDRESS: Address of the 4th subculture master
pub fn bookmark_master3_address() {
    todo!();
}
/// BOOKMARK__MASTER3_NAME: Name of the 4th subculture
pub fn bookmark_master3_name() {
    todo!();
}
/// BOOKMARK__MASTER3_PORT: Port of the 4th subculture master
pub fn bookmark_master3_port() {
    todo!();
}
/// BOOKMARK__MASTER4_ADDRESS: Address of the 5th subculture master
pub fn bookmark_master4_address() {
    todo!();
}
/// BOOKMARK__MASTER4_NAME: Name of the 5th subculture
pub fn bookmark_master4_name() {
    todo!();
}
/// BOOKMARK__MASTER4_PORT: Port of the 5th subculture master
pub fn bookmark_master4_port() {
    todo!();
}
/// BOOKMARK__MASTER5_ADDRESS: Address of the 6th subculture master
pub fn bookmark_master5_address() {
    todo!();
}
/// BOOKMARK__MASTER5_NAME: Name of the 6th subculture
pub fn bookmark_master5_name() {
    todo!();
}
/// BOOKMARK__MASTER5_PORT: Port of the 6th subculture master
pub fn bookmark_master5_port() {
    todo!();
}
/// BOOKMARK__MASTER6_ADDRESS: Address of the 7th subculture master
pub fn bookmark_master6_address() {
    todo!();
}
/// BOOKMARK__MASTER6_NAME: Name of the 7th subculture
pub fn bookmark_master6_name() {
    todo!();
}
/// BOOKMARK__MASTER6_PORT: Port of the 7th subculture master
pub fn bookmark_master6_port() {
    todo!();
}
/// BOOKMARK__MASTER7_ADDRESS: Address of the 8th subculture master
pub fn bookmark_master7_address() {
    todo!();
}
/// BOOKMARK__MASTER7_NAME: Name of the 8th subculture
pub fn bookmark_master7_name() {
    todo!();
}
/// BOOKMARK__MASTER7_PORT: Port of the 8th subculture master
pub fn bookmark_master7_port() {
    todo!();
}
/// BOOKMARK__MASTER8_ADDRESS: Address of the 9th subculture master
pub fn bookmark_master8_address() {
    todo!();
}
/// BOOKMARK__MASTER8_NAME: Name of the 9th subculture
pub fn bookmark_master8_name() {
    todo!();
}
/// BOOKMARK__MASTER8_PORT: Port of the 9th subculture master
pub fn bookmark_master8_port() {
    todo!();
}
/// BOOKMARK__MASTER9_ADDRESS: Address of the 10th subculture master
pub fn bookmark_master9_address() {
    todo!();
}
/// BOOKMARK__MASTER9_NAME: Name of the 10th subculture
pub fn bookmark_master9_name() {
    todo!();
}
/// BOOKMARK__MASTER9_PORT: Port of the 10th subculture master
pub fn bookmark_master9_port() {
    todo!();
}
/// BRAKE_GAUGE_LOCX: Horizontal position of the brake meter
pub fn brake_gauge_locx() {
    todo!();
}
/// BRAKE_GAUGE_LOCY: Vertical position of the brake meter
pub fn brake_gauge_locy() {
    todo!();
}
/// BRAKE_GAUGE_SIZE: Size of the brake meter
pub fn brake_gauge_size() {
    todo!();
}
/// BUG_COLOR_OVERFLOW: Allows the player's colors to overflow and wrap around for the cycle, allowing different colors for cycle and trail.
pub fn bug_color_overflow() {
    todo!();
}
/// BUG_RIP: Allows the rim wall to be ripped open by a VERY close grind.
pub fn bug_rip() {
    todo!();
}
/// BUG_TRANSPARENCY: Unsupported: make all rim walls semi-transparent by rendering them without occlusion tests
pub fn bug_transparency() {
    todo!();
}
/// BUG_TRANSPARENCY_DEMAND: Unsupported: use transparency instead of lowering walls
pub fn bug_transparency_demand() {
    todo!();
}
/// BUG_TUNNEL: Allows players to pass through walls on odd occasions.
pub fn bug_tunnel() {
    todo!();
}
/// CAMCENTER_1: Center internal camera on driving direction
pub fn camcenter_1() {
    todo!();
}
/// CAMCENTER_2: Center internal camera on driving direction
pub fn camcenter_2() {
    todo!();
}
/// CAMCENTER_3: Center internal camera on driving direction
pub fn camcenter_3() {
    todo!();
}
/// CAMCENTER_4: Center internal camera on driving direction
pub fn camcenter_4() {
    todo!();
}
/// CAMERA_CUSTOM_BACK: Position of the custom camera: how much is it moved back from the cycle?
pub fn camera_custom_back() {
    todo!();
}
/// CAMERA_CUSTOM_BACK_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_CUSTOM_BACK.
pub fn camera_custom_back_fromspeed() {
    todo!();
}
/// CAMERA_CUSTOM_PITCH: Position of the custom camera: how much does it look up/down?
pub fn camera_custom_pitch() {
    todo!();
}
/// CAMERA_CUSTOM_RISE: Position of the custom camera: how much is it moved up from the cycle?
pub fn camera_custom_rise() {
    todo!();
}
/// CAMERA_CUSTOM_RISE_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_CUSTOM_RISE.
pub fn camera_custom_rise_fromspeed() {
    todo!();
}
/// CAMERA_CUSTOM_TURN_SPEED: Speed the custom camera turns with
pub fn camera_custom_turn_speed() {
    todo!();
}
/// CAMERA_CUSTOM_TURN_SPEED_180: Extra factor to CAMERA_CUSTOM_TURN_SPEED after a quick reverse
pub fn camera_custom_turn_speed_180() {
    todo!();
}
/// CAMERA_CUSTOM_ZOOM: Position of the custom camera: how much the camera zooms in your cycle at the beginning of the round (to show the team's formation
pub fn camera_custom_zoom() {
    todo!();
}
/// CAMERA_FOLLOW_START_X: Start position of the fixed external camera
pub fn camera_follow_start_x() {
    todo!();
}
/// CAMERA_FOLLOW_START_Y: Start position of the fixed external camera
pub fn camera_follow_start_y() {
    todo!();
}
/// CAMERA_FOLLOW_START_Z: Start position of the fixed external camera
pub fn camera_follow_start_z() {
    todo!();
}
/// CAMERA_FORBID_CUSTOM: Forbids the use of the custom camera on all clients
pub fn camera_forbid_custom() {
    todo!();
}
/// CAMERA_FORBID_CUSTOM_GLANCE: Forbids use of special glance camera settings
pub fn camera_forbid_custom_glance() {
    todo!();
}
/// CAMERA_FORBID_FOLLOW: Forbids the use of the fixed external camera on all clients
pub fn camera_forbid_follow() {
    todo!();
}
/// CAMERA_FORBID_FREE: Forbids the use of the free camera on all clients
pub fn camera_forbid_free() {
    todo!();
}
/// CAMERA_FORBID_IN: Forbids the use of the internal camera on all clients
pub fn camera_forbid_in() {
    todo!();
}
/// CAMERA_FORBID_SERVER_CUSTOM: Forbids the use of the server custom camera
pub fn camera_forbid_server_custom() {
    todo!();
}
/// CAMERA_FORBID_SMART: Forbids the use of the internal camera on all clients
pub fn camera_forbid_smart() {
    todo!();
}
/// CAMERA_FREE_START_X: Start position of the free camera
pub fn camera_free_start_x() {
    todo!();
}
/// CAMERA_FREE_START_Y: Start position of the free camera
pub fn camera_free_start_y() {
    todo!();
}
/// CAMERA_FREE_START_Z: Start position of the free camera
pub fn camera_free_start_z() {
    todo!();
}
/// CAMERA_GLANCE_BACK: Position of the glance camera: how much is it moved back from the cycle?
pub fn camera_glance_back() {
    todo!();
}
/// CAMERA_GLANCE_BACK_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_GLANCE_BACK.
pub fn camera_glance_back_fromspeed() {
    todo!();
}
/// CAMERA_GLANCE_PITCH: Position of the glance camera: how much does it look up/down?
pub fn camera_glance_pitch() {
    todo!();
}
/// CAMERA_GLANCE_RISE: Position of the glance camera: how much is it moved up from the cycle?
pub fn camera_glance_rise() {
    todo!();
}
/// CAMERA_GLANCE_RISE_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_GLANCE_RISE.
pub fn camera_glance_rise_fromspeed() {
    todo!();
}
/// CAMERA_IN_TURN_SPEED: Speed the internal camera turns with
pub fn camera_in_turn_speed() {
    todo!();
}
/// CAMERA_OVERRIDE_CUSTOM_GLANCE: Overrides custom glance settings with values from the server
pub fn camera_override_custom_glance() {
    todo!();
}
/// CAMERA_OVERRIDE_CUSTOM_GLANCE_SERVER_CUSTOM: Overrides custom glance settings with values from the server only for the server custom camera
pub fn camera_override_custom_glance_server_custom() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_BACK: Position of the custom camera: how much is it moved back from the cycle?
pub fn camera_server_custom_back() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_BACK_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_SERVER_CUSTOM_BACK.
pub fn camera_server_custom_back_fromspeed() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_PITCH: Position of the custom camera: how much does it look up/down?
pub fn camera_server_custom_pitch() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_RISE: Position of the custom camera: how much is it moved up from the cycle?
pub fn camera_server_custom_rise() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_RISE_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_SERVER_CUSTOM_RISE.
pub fn camera_server_custom_rise_fromspeed() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_TURN_SPEED: Speed the server custom camera turns with. Turn values are taken from the client-side settings if this is negative.
pub fn camera_server_custom_turn_speed() {
    todo!();
}
/// CAMERA_SERVER_CUSTOM_TURN_SPEED_180: Extra factor to CAMERA_SERVER_CUSTOM_TURN_SPEED after a quick reverse
pub fn camera_server_custom_turn_speed_180() {
    todo!();
}
/// CAMERA_SERVER_GLANCE_BACK: Position of the server glance camera: how much is it moved back from the cycle?
pub fn camera_server_glance_back() {
    todo!();
}
/// CAMERA_SERVER_GLANCE_BACK_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_SERVER_GLANCE_BACK.
pub fn camera_server_glance_back_fromspeed() {
    todo!();
}
/// CAMERA_SERVER_GLANCE_PITCH: Position of the server glance camera: how much does it look up/down?
pub fn camera_server_glance_pitch() {
    todo!();
}
/// CAMERA_SERVER_GLANCE_RISE: Position of the server glance camera: how much is it moved up from the cycle?
pub fn camera_server_glance_rise() {
    todo!();
}
/// CAMERA_SERVER_GLANCE_RISE_FROMSPEED: This value is multiplied with the current speed and added to CAMERA_SERVER_GLANCE_RISE.
pub fn camera_server_glance_rise_fromspeed() {
    todo!();
}
/// CAMERA_SMART_START_X: Start position of the smart camera
pub fn camera_smart_start_x() {
    todo!();
}
/// CAMERA_SMART_START_Y: Start position of the smart camera
pub fn camera_smart_start_y() {
    todo!();
}
/// CAMERA_SMART_START_Z: Start position of the smart camera
pub fn camera_smart_start_z() {
    todo!();
}
/// CAMERA_VISIBILITY_CLIP_SPEED: Speed with which the visibility targets is brought into view
pub fn camera_visibility_clip_speed() {
    todo!();
}
/// CAMERA_VISIBILITY_EXTENSION: Distance (measured in seconds, gets multiplied by speed) of the visibility targets from the watched object
pub fn camera_visibility_extension() {
    todo!();
}
/// CAMERA_VISIBILITY_LOWER_WALL: If set to 1, walls are lowered when they block the view and the camera is not moved
pub fn camera_visibility_lower_wall() {
    todo!();
}
/// CAMERA_VISIBILITY_LOWER_WALL_SMART: Like CAMERA_VISIBILITY_LOWER_WALL, but special setting for the smart camera
pub fn camera_visibility_lower_wall_smart() {
    todo!();
}
/// CAMERA_VISIBILITY_RECOVERY_SPEED: The speed the external visibility targets recovers from wall hits
pub fn camera_visibility_recovery_speed() {
    todo!();
}
/// CAMERA_VISIBILITY_SIDESKEW: Extra forward component of the sideways visibility targets
pub fn camera_visibility_sideskew() {
    todo!();
}
/// CAMERA_VISIBILITY_WALL_DISTANCE: The distance the visibility targets keep from walls
pub fn camera_visibility_wall_distance() {
    todo!();
}
/// CAMWOBBLE_1: Lets the internal camera move with your cycle
pub fn camwobble_1() {
    todo!();
}
/// CAMWOBBLE_2: Lets the internal camera move with your cycle
pub fn camwobble_2() {
    todo!();
}
/// CAMWOBBLE_3: Lets the internal camera move with your cycle
pub fn camwobble_3() {
    todo!();
}
/// CAMWOBBLE_4: Lets the internal camera move with your cycle
pub fn camwobble_4() {
    todo!();
}
/// CASACL: For the duration of the rest of the configuration file  this directive appears in, elevate the access level.
pub fn casacl() {
    todo!();
}
/// CENTER_MESSAGE: Prints a big message on the screen of all connected clients.
pub fn center_message() {
    todo!();
}
/// CHATBOT_ALWAYS_ACTIVE: if set to 1, the chatbot is active all of the time
pub fn chatbot_always_active() {
    todo!();
}
/// CHATBOT_DECAY: rate at which the quality of the chatbot decays over time
pub fn chatbot_decay() {
    todo!();
}
/// CHATBOT_DELAY: time between entering chat and chatbot activation
pub fn chatbot_delay() {
    todo!();
}
/// CHATBOT_MIN_TIMESTEP: minimal time in seconds between chatbot thoughts
pub fn chatbot_min_timestep() {
    todo!();
}
/// CHATBOT_NEW_WALL_BLINDNESS: the chatbot won't see walls that were built less than this many seconds ago
pub fn chatbot_new_wall_blindness() {
    todo!();
}
/// CHATBOT_RANGE: time in seconds the bot is capable of planning ahead
pub fn chatbot_range() {
    todo!();
}
/// CHATTER_REMOVE_TIME: Time in seconds after which a permanent chatter is removed from the game
pub fn chatter_remove_time() {
    todo!();
}
/// CHAT_LOG: Write machine parsable chat messages to var/chatlog.txt
pub fn chat_log() {
    todo!();
}
/// CHAT_TOOLTIP: chat_tooltip_help
pub fn chat_tooltip() {
    todo!();
}
/// CHECK_ERRORS: Listen to errors claiming a video mode does not exist
pub fn check_errors() {
    todo!();
}
/// CLIENT_PORT: Port we try to connect to
pub fn client_port() {
    todo!();
}
/// CM_LOCY: Vertical position of the center messages
pub fn cm_locy() {
    todo!();
}
/// COLORDEPTH: Color depth to use (0: 16 1: desktop 2: 24)
pub fn colordepth() {
    todo!();
}
/// COLOR_B_1: Cycle and wall color, blue component.
pub fn color_b_1() {
    todo!();
}
/// COLOR_B_2: Cycle and wall color, blue component.
pub fn color_b_2() {
    todo!();
}
/// COLOR_B_3: Cycle and wall color, blue component.
pub fn color_b_3() {
    todo!();
}
/// COLOR_B_4: Cycle and wall color, blue component.
pub fn color_b_4() {
    todo!();
}
/// COLOR_G_1: Cycle and wall color, green component.
pub fn color_g_1() {
    todo!();
}
/// COLOR_G_2: Cycle and wall color, green component.
pub fn color_g_2() {
    todo!();
}
/// COLOR_G_3: Cycle and wall color, green component.
pub fn color_g_3() {
    todo!();
}
/// COLOR_G_4: Cycle and wall color, green component.
pub fn color_g_4() {
    todo!();
}
/// COLOR_R_1: Cycle and wall color, red component.
pub fn color_r_1() {
    todo!();
}
/// COLOR_R_2: Cycle and wall color, red component.
pub fn color_r_2() {
    todo!();
}
/// COLOR_R_3: Cycle and wall color, red component.
pub fn color_r_3() {
    todo!();
}
/// COLOR_R_4: Cycle and wall color, red component.
pub fn color_r_4() {
    todo!();
}
/// COLOR_STRINGS: Print colored strings
pub fn color_strings() {
    todo!();
}
/// CONNECTION_FLOOD_SENSITIVITY: The times PING_FLOOD_TIME_X, multiplied by this value, count for all incoming messages from clients not connected already. A flood here activates turtle mode. Negative values disable global flood protection.
pub fn connection_flood_sensitivity() {
    todo!();
}
/// CONNECTION_LIMIT: Maximum number of packets from unknown peers to handle at one
pub fn connection_limit() {
    todo!();
}
/// CONSOLE_COLUMNS: Number of characters in each line of console output. 0 for automatic mode. -1 to take the large bitmap font in its native size no matter what.
pub fn console_columns() {
    todo!();
}
/// CONSOLE_INDENT: Number of spaces each continuation of a wrapped console line is indented by
pub fn console_indent() {
    todo!();
}
/// CONSOLE_LADDER_LOG: Sends ladder log output to the console
pub fn console_ladder_log() {
    todo!();
}
/// CONSOLE_LOG: Write all console messages to var/consolelog.txt
pub fn console_log() {
    todo!();
}
/// CONSOLE_MESSAGE: Prints a message on the console of all connected clients.
pub fn console_message(message: &str) {
    println!("{} {}", Command::ConsoleMessage, message);
}
/// CONSOLE_ROWS: Number of lines of console output without user intervention
pub fn console_rows() {
    todo!();
}
/// CONSOLE_ROWS_MAX: Number of lines of console output when scrolling back
pub fn console_rows_max() {
    todo!();
}
/// CUSTOM_SCREEN_ASPECT: Custom screen aspect ratio ( pixel width/pixel height)
pub fn custom_screen_aspect() {
    todo!();
}
/// CUSTOM_SCREEN_HEIGHT: Custom screen size
pub fn custom_screen_height() {
    todo!();
}
/// CUSTOM_SCREEN_WIDTH: Custom screen size
pub fn custom_screen_width() {
    todo!();
}
/// CUSTOM_SERVER_NAME: Name of the server to connect to
pub fn custom_server_name() {
    todo!();
}
/// CYCLE_ACCEL: Wall acceleration factor
pub fn cycle_accel() {
    todo!();
}
/// CYCLE_ACCEL_ENEMY: Multiplicator to CYCLE_ACCEL for your enemies' walls
pub fn cycle_accel_enemy() {
    todo!();
}
/// CYCLE_ACCEL_ENEMY_OVERRIDE: Block out older clients when CYCLE_ACCEL_ENEMY differs from its default?
pub fn cycle_accel_enemy_override() {
    todo!();
}
/// CYCLE_ACCEL_OFFSET: Minimum numeric wall distance, must be positive
pub fn cycle_accel_offset() {
    todo!();
}
/// CYCLE_ACCEL_RIM: Multiplicator to CYCLE_ACCEL for the rim walls
pub fn cycle_accel_rim() {
    todo!();
}
/// CYCLE_ACCEL_RIM_OVERRIDE: Block out older clients when CYCLE_ACCEL_RIM differs from its default?
pub fn cycle_accel_rim_override() {
    todo!();
}
/// CYCLE_ACCEL_SELF: Multiplicator to CYCLE_ACCEL for your own wall
pub fn cycle_accel_self() {
    todo!();
}
/// CYCLE_ACCEL_SELF_OVERRIDE: Block out older clients when CYCLE_ACCEL_SELF differs from its default?
pub fn cycle_accel_self_override() {
    todo!();
}
/// CYCLE_ACCEL_SLINGSHOT: Multiplicator to the total effect of CYCLE_ACCEL, if the cycle is between its own wall and another wall
pub fn cycle_accel_slingshot() {
    todo!();
}
/// CYCLE_ACCEL_SLINGSHOT_OVERRIDE: Block out older clients when CYCLE_ACCEL_SLINGSHOT differs from its default?
pub fn cycle_accel_slingshot_override() {
    todo!();
}
/// CYCLE_ACCEL_TEAM: Multiplicator to CYCLE_ACCEL for your teammates' walls
pub fn cycle_accel_team() {
    todo!();
}
/// CYCLE_ACCEL_TEAM_OVERRIDE: Block out older clients when CYCLE_ACCEL_TEAM differs from its default?
pub fn cycle_accel_team_override() {
    todo!();
}
/// CYCLE_ACCEL_TUNNEL: Multiplicator to the total effect of CYCLE_ACCEL, if the cycle is between two walls not created by it
pub fn cycle_accel_tunnel() {
    todo!();
}
/// CYCLE_ACCEL_TUNNEL_OVERRIDE: Block out older clients if CYCLE_ACCEL_TUNNEL differs from its default?
pub fn cycle_accel_tunnel_override() {
    todo!();
}
/// CYCLE_AVOID_OLDCLIENT_BAD_SYNC: If set to 1, old clients will not get sync messages in situations that are known to confuse them
pub fn cycle_avoid_oldclient_bad_sync() {
    todo!();
}
/// CYCLE_BLINK_FREQUENCY: Frequency in Hz an invulnerable cycle blinks with.
pub fn cycle_blink_frequency() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_ENEMY: Factor your speed is multiplied with when breaking from an enemy wall
pub fn cycle_boostfactor_enemy() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_ENEMY_OVERRIDE: Block out older clients if CYCLE_BOOSTFACTOR_ENEMY differs from its default?
pub fn cycle_boostfactor_enemy_override() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_RIM: Factor your speed is multiplied with when breaking from the rim wall
pub fn cycle_boostfactor_rim() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_RIM_OVERRIDE: Block out older clients if CYCLE_BOOSTFACTOR_RIM differs from its default?
pub fn cycle_boostfactor_rim_override() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_SELF: Factor your speed is multiplied with when breaking from your own wall
pub fn cycle_boostfactor_self() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_SELF_OVERRIDE: Block out older clients if CYCLE_BOOSTFACTOR_SELF differs from its default?
pub fn cycle_boostfactor_self_override() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_TEAM: Factor your speed is multiplied with when breaking from a teammate's wall
pub fn cycle_boostfactor_team() {
    todo!();
}
/// CYCLE_BOOSTFACTOR_TEAM_OVERRIDE: Block out older clients if CYCLE_BOOSTFACTOR_TEAM differs from its default?
pub fn cycle_boostfactor_team_override() {
    todo!();
}
/// CYCLE_BOOST_ENEMY: Speed boost when breaking from an enemy wall
pub fn cycle_boost_enemy() {
    todo!();
}
/// CYCLE_BOOST_ENEMY_OVERRIDE: Block out older clients if CYCLE_BOOST_ENEMY differs from its default?
pub fn cycle_boost_enemy_override() {
    todo!();
}
/// CYCLE_BOOST_RIM: Speed boost when breaking from the rim wall
pub fn cycle_boost_rim() {
    todo!();
}
/// CYCLE_BOOST_RIM_OVERRIDE: Block out older clients if CYCLE_BOOST_RIM differs from its default?
pub fn cycle_boost_rim_override() {
    todo!();
}
/// CYCLE_BOOST_SELF: Speed boost when breaking from your own wall
pub fn cycle_boost_self() {
    todo!();
}
/// CYCLE_BOOST_SELF_OVERRIDE: Block out older clients if CYCLE_BOOST_SELF differs from its default?
pub fn cycle_boost_self_override() {
    todo!();
}
/// CYCLE_BOOST_TEAM: Speed boost when breaking from a teammate's wall
pub fn cycle_boost_team() {
    todo!();
}
/// CYCLE_BOOST_TEAM_OVERRIDE: Block out older clients if CYCLE_BOOST_TEAM differs from its default?
pub fn cycle_boost_team_override() {
    todo!();
}
/// CYCLE_BRAKE: Brake intensity
pub fn cycle_brake() {
    todo!();
}
/// CYCLE_BRAKE_DEPLETE: Rate at which the brake reservoir depletes when you are braking
pub fn cycle_brake_deplete() {
    todo!();
}
/// CYCLE_BRAKE_DEPLETE_OVERRIDE: Block out older clients when CYCLE_BRAKE_DEPLETE differs from its default?
pub fn cycle_brake_deplete_override() {
    todo!();
}
/// CYCLE_BRAKE_REFILL: Rate at which the brake reservoir refills when you are not braking
pub fn cycle_brake_refill() {
    todo!();
}
/// CYCLE_BRAKE_REFILL_OVERRIDE: Block out older clients when CYCLE_BRAKE_REFILL differs from its default?
pub fn cycle_brake_refill_override() {
    todo!();
}
/// CYCLE_BRAKE_TOOLTIP: cycle_brake_tooltip_help
pub fn cycle_brake_tooltip() {
    todo!();
}
/// CYCLE_DELAY: Minimum time between turns (must be greater than 0)
pub fn cycle_delay() {
    todo!();
}
/// CYCLE_DELAY_DOUBLEBIND_BONUS: Factor CYCLE_DELAY is multiplied with for consecutive turns in the same direction
pub fn cycle_delay_doublebind_bonus() {
    todo!();
}
/// CYCLE_DELAY_DOUBLEBIND_BONUS_OVERRIDE: Block out older clients if CYCLE_DELAY_DOUBLEBIND_BONUS differs from its default?
pub fn cycle_delay_doublebind_bonus_override() {
    todo!();
}
/// CYCLE_DELAY_TIMEBASED: Turn delays will be based on the time since the last turn if this is 1 (default) and the distance if this is 0. Intermediate values and values out of these bounds are supported as well.
pub fn cycle_delay_timebased() {
    todo!();
}
/// CYCLE_DELAY_TIMEBASED_OVERRIDE: Block out older clients when CYCLE_DELAY_TIMEBASED differs from its default?
pub fn cycle_delay_timebased_override() {
    todo!();
}
/// CYCLE_DIST_WALL_SHRINK: Distance multiplier in wall length calculation. All values are legal. See settings.cfg for full docs.
pub fn cycle_dist_wall_shrink() {
    todo!();
}
/// CYCLE_DIST_WALL_SHRINK_OFFSET: Distance offset in wall length calculation. See settings.cfg for full docs.
pub fn cycle_dist_wall_shrink_offset() {
    todo!();
}
/// CYCLE_FAIR_ANTILAG: If set to 1, this deactivates the anti lag-sliding code when old clients are connected
pub fn cycle_fair_antilag() {
    todo!();
}
/// CYCLE_FIRST_SPAWN_PROTECTION: Set to 1 if the invulnerability and wall delay should already be active on the initial spawn at the beginning of a round.
pub fn cycle_first_spawn_protection() {
    todo!();
}
/// CYCLE_INVULNERABLE_TIME: Time in seconds a cycle is invulnerable after a respawn.
pub fn cycle_invulnerable_time() {
    todo!();
}
/// CYCLE_MAX_REFCOUNT: Maximum allowed reference count on cycles before they self destruct. This setting is to protect against performance related DOS attacks.
pub fn cycle_max_refcount() {
    todo!();
}
/// CYCLE_PACKETLOSS_TOLERANCE: Cycle death is prevented as long as the player's failure to turn can be explained by the loss of this many network packets. Enabling this allows cheating.
pub fn cycle_packetloss_tolerance() {
    todo!();
}
/// CYCLE_PING_RUBBER: Additional niceness for high ping players
pub fn cycle_ping_rubber() {
    todo!();
}
/// CYCLE_RUBBER: Niceness factor to allow you drive really close to a wall
pub fn cycle_rubber() {
    todo!();
}
/// CYCLE_RUBBER_DELAY: During this fraction of the cycle delay time after each turn, rubber efficiency will be multiplied with CYCLE_RUBBER_DELAY_BONUS.
pub fn cycle_rubber_delay() {
    todo!();
}
/// CYCLE_RUBBER_DELAY_BONUS: Factor for CYCLE_RUBBER_DELAY rubber efficiency.
pub fn cycle_rubber_delay_bonus() {
    todo!();
}
/// CYCLE_RUBBER_DELAY_BONUS_OVERRIDE: Block out older clients when CYCLE_RUBBER_DELAY_BONUS differs from its default?
pub fn cycle_rubber_delay_bonus_override() {
    todo!();
}
/// CYCLE_RUBBER_DELAY_OVERRIDE: Block out older clients when CYCLE_RUBBER_DELAY differs from its default?
pub fn cycle_rubber_delay_override() {
    todo!();
}
/// CYCLE_RUBBER_LEGACY: Revert to old, framerate dependent and old-clients-ripping, rubber code if old clients are present. Old means <= 0.2.7.0 here.
pub fn cycle_rubber_legacy() {
    todo!();
}
/// CYCLE_RUBBER_MALUS_TURN_OVERRIDE: Block out older clients when CYCLE_RUBBER_MALUS_TURN differs from its default?
pub fn cycle_rubber_malus_turn_override() {
    todo!();
}
/// CYCLE_RUBBER_MINADJUST: When adjusting to or 180ing into a wall, allow going closer by at least this amount (relative to the last distance)
pub fn cycle_rubber_minadjust() {
    todo!();
}
/// CYCLE_RUBBER_MINADJUST_OVERRIDE: Block out older clients when CYCLE_RUBBER_MINADJUST differs from its default?
pub fn cycle_rubber_minadjust_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE: The minimal distance rubber code keeps you from the wall in front of you
pub fn cycle_rubber_mindistance() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_GAP: If > 0, CYCLE_RUBBER_MINDISTANCE effectively is never taken to be bigger than this value times the size of any detected gaps the cycle can squeeze through. For "Open" gameplay.
pub fn cycle_rubber_mindistance_gap() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_GAP_BACKDOOR: If > 0, CYCLE_RUBBER_MINDISTANCE effectively is never taken to be bigger than this value times the size of any detected backdoor gaps the cycle can squeeze through. For "Open" gameplay. If = 0, CYCLE_RUBBER_MINDISTANCE_GAP applies to backdoors, too.
pub fn cycle_rubber_mindistance_gap_backdoor() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_GAP_BACKDOOR_OVERRIDE: Block out older clients if CYCLE_RUBBER_MINDISTANCE_GAP_BACKDOOR differs from its default?
pub fn cycle_rubber_mindistance_gap_backdoor_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_GAP_OVERRIDE: Block out older clients if CYCLE_RUBBER_MINDISTANCE_GAP differs from its default?
pub fn cycle_rubber_mindistance_gap_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_GAP_SIDE: Gap detection only sees gaps that the cycle may reach in no less than this many seconds.
pub fn cycle_rubber_mindistance_gap_side() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_LEGACY: Extra factor for minimal distance to walls enforced by the rubber code, active when peers with the rip bug are connected
pub fn cycle_rubber_mindistance_legacy() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_OVERRIDE: Block out older clients when CYCLE_RUBBER_MINDISTANCE differs from its default?
pub fn cycle_rubber_mindistance_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_PREPARATION: Timescale in seconds a cycle's last turn time is compared with to determine the effect of CYCLE_RUBBER_MINDISTANCE_UNPREPARED.
pub fn cycle_rubber_mindistance_preparation() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_PREPARATION_OVERRIDE: Block out older clients when CYCLE_RUBBER_MINDISTANCE_PREPARATION differs from its default?
pub fn cycle_rubber_mindistance_preparation_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_RATIO: Additional distance to CYCLE_RUBBER_MINDISTANCE for every length unit of the wall you have in front of you
pub fn cycle_rubber_mindistance_ratio() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_RATIO_OVERRIDE: Block out older clients when CYCLE_RUBBER_MINDISTANCE_RATIO differs from its default?
pub fn cycle_rubber_mindistance_ratio_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_RESERVOIR: Additional distance if you have an empty rubber meter (gets faded out gradually as you use up all your rubber)
pub fn cycle_rubber_mindistance_reservoir() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_RESERVOIR_OVERRIDE: Block out older clients when CYCLE_RUBBER_MINDISTANCE_RESERVOIR differs from its default?
pub fn cycle_rubber_mindistance_reservoir_override() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_UNPREPARED: Additional distance for unprepared grinds; it gets applied when the cycle's last turn was just a fraction of a second ago and faded out preparation times larger than CYCLE_RUBBER_MINDISTANCE_PREPARATION.
pub fn cycle_rubber_mindistance_unprepared() {
    todo!();
}
/// CYCLE_RUBBER_MINDISTANCE_UNPREPARED_OVERRIDE: Block out older clients when CYCLE_RUBBER_MINDISTANCE_UNPREPARED differs from its default?
pub fn cycle_rubber_mindistance_unprepared_override() {
    todo!();
}
/// CYCLE_RUBBER_SPEED: Logarithmic speed of wall approximation when rubber is in effect (every second, you get closer to the wall by a factor of ~0.4^{this value})
pub fn cycle_rubber_speed() {
    todo!();
}
/// CYCLE_RUBBER_SPEED_OVERRIDE: Block out older clients when CYCLE_RUBBER_SPEED differs from its default?
pub fn cycle_rubber_speed_override() {
    todo!();
}
/// CYCLE_RUBBER_TIME: Timescale rubber is restored on.
pub fn cycle_rubber_time() {
    todo!();
}
/// CYCLE_RUBBER_TIMEBASED: Rubber usage is based on distance travelled if this is 0 (default) and the time passed if this is 1. Intermediate values and values out of these bounds are supported as well.
pub fn cycle_rubber_timebased() {
    todo!();
}
/// CYCLE_RUBBER_TIMEBASED_OVERRIDE: Block out older clients when CYCLE_RUBBER_TIMEBASED differs from its default?
pub fn cycle_rubber_timebased_override() {
    todo!();
}
/// CYCLE_RUBBER_TIME_OVERRIDE: Block out older clients when CYCLE_RUBBER_TIME differs from its default?
pub fn cycle_rubber_time_override() {
    todo!();
}
/// CYCLE_RUBBER_WALL_SHRINK: With finite length trails, the used rubber is multiplied with this value and the result is subtracted from the wall length.
pub fn cycle_rubber_wall_shrink() {
    todo!();
}
/// CYCLE_RUBBER_WALL_SHRINK_OVERRIDE: Block out older clients when CYCLE_RUBBER_WALL_SHRINK differs from its default?
pub fn cycle_rubber_wall_shrink_override() {
    todo!();
}
/// CYCLE_SMOOTH_MIN_SPEED: Minimum speed of smooth correction relative to cycle speed.
pub fn cycle_smooth_min_speed() {
    todo!();
}
/// CYCLE_SMOOTH_THRESHOLD: Only syncs that differ from your position by less than this amount (measured in speed) will be handled smoothly, bigger differences will be applied instantly.
pub fn cycle_smooth_threshold() {
    todo!();
}
/// CYCLE_SMOOTH_TIME: Timescale for smoothing options. Increasing this will make interpolation smoother, but less accurate. Decreasing it will make network synchronization jumpy.
pub fn cycle_smooth_time() {
    todo!();
}
/// CYCLE_SOUND_SPEED: Sound speed divisor
pub fn cycle_sound_speed() {
    todo!();
}
/// CYCLE_SPEED: Basic speed of your cycle if you drive straight and not close to walls
pub fn cycle_speed() {
    todo!();
}
/// CYCLE_SPEED_DECAY_ABOVE: Rate of cycle speed approaching the value of CYCLE_SPEED from above
pub fn cycle_speed_decay_above() {
    todo!();
}
/// CYCLE_SPEED_DECAY_ABOVE_OVERRIDE: Block out older clients when CYCLE_SPEED_DECAY_ABOVE differs from its default?
pub fn cycle_speed_decay_above_override() {
    todo!();
}
/// CYCLE_SPEED_DECAY_BELOW: Rate of cycle speed approaching the value of CYCLE_SPEED from below
pub fn cycle_speed_decay_below() {
    todo!();
}
/// CYCLE_SPEED_DECAY_BELOW_OVERRIDE: Block out older clients when CYCLE_SPEED_DECAY_BELOW differs from its default?
pub fn cycle_speed_decay_below_override() {
    todo!();
}
/// CYCLE_SPEED_MAX: Maximal speed of your cycle, measured relative to CYCLE_SPEED. A value of 0 means no top speed.
pub fn cycle_speed_max() {
    todo!();
}
/// CYCLE_SPEED_MAX_OVERRIDE: Block out older clients if CYCLE_SPEED_MAX differs from its default?
pub fn cycle_speed_max_override() {
    todo!();
}
/// CYCLE_SPEED_MIN: Minimal speed of your cycle, measured relative to CYCLE_SPEED
pub fn cycle_speed_min() {
    todo!();
}
/// CYCLE_SPEED_MIN_OVERRIDE: Block out older clients when CYCLE_SPEED_MIN differs from its default?
pub fn cycle_speed_min_override() {
    todo!();
}
/// CYCLE_START_SPEED: Initial cycle speed
pub fn cycle_start_speed() {
    todo!();
}
/// CYCLE_SYNC_FF: Speed of simulation of the extrapolating sync; decrease for lower CPU load, but higher effective ping
pub fn cycle_sync_ff() {
    todo!();
}
/// CYCLE_SYNC_FF_STEPS: Number of extrapolation simulation timesteps each real timestep; increase for better accuracy
pub fn cycle_sync_ff_steps() {
    todo!();
}
/// CYCLE_SYNC_INTERVAL_ENEMY: Time in seconds between server-client updates of enemy cycles
pub fn cycle_sync_interval_enemy() {
    todo!();
}
/// CYCLE_SYNC_INTERVAL_SELF: Time in seconds between server-client updates of enemy cycles owned by the client itself
pub fn cycle_sync_interval_self() {
    todo!();
}
/// CYCLE_TIME_TOLERANCE: Maximum time difference of execution of turns on server and client (for clients that send timing information)
pub fn cycle_time_tolerance() {
    todo!();
}
/// CYCLE_TIME_TOLERANCE_OVERRIDE: Block out older clients when CYCLE_TIME_TOLERANCE differs from its default?
pub fn cycle_time_tolerance_override() {
    todo!();
}
/// CYCLE_TURN_LEFT_TOOLTIP: cycle_turn_left_tooltip_help
pub fn cycle_turn_left_tooltip() {
    todo!();
}
/// CYCLE_TURN_MEMORY: Number of pending turns a cycle will memorize exactly
pub fn cycle_turn_memory() {
    todo!();
}
/// CYCLE_TURN_RIGHT_TOOLTIP: cycle_turn_right_tooltip_help
pub fn cycle_turn_right_tooltip() {
    todo!();
}
/// CYCLE_TURN_SPEED_FACTOR: Factor the speed of a lightcycle is multiplied with when turning
pub fn cycle_turn_speed_factor() {
    todo!();
}
/// CYCLE_TURN_SPEED_FACTOR_OVERRIDE: Block out older clients when CYCLE_TURN_SPEED_FACTOR differs from its default?
pub fn cycle_turn_speed_factor_override() {
    todo!();
}
/// CYCLE_WALL_NEAR: Maximum accelerating wall distance
pub fn cycle_wall_near() {
    todo!();
}
/// CYCLE_WALL_TIME: Time in seconds a cycle does not make a wall after a respawn.
pub fn cycle_wall_time() {
    todo!();
}
/// CYCLE_WIDTH: The width of the cycle collision object. It can only squeeze through tunnels wider than that without taking harm.
pub fn cycle_width() {
    todo!();
}
/// CYCLE_WIDTH_OVERRIDE: Block out older clients if CYCLE_WIDTH differs from its default?
pub fn cycle_width_override() {
    todo!();
}
/// CYCLE_WIDTH_RUBBER_MAX: If the cycle_width conditions are massively violated, use up this much rubber.If set to 1, the rubber usage rate is the same as if you were sitting in front of a wall.
pub fn cycle_width_rubber_max() {
    todo!();
}
/// CYCLE_WIDTH_RUBBER_MAX_OVERRIDE: Block out older clients if CYCLE_WIDTH_RUBBER_MAX differs from its default?
pub fn cycle_width_rubber_max_override() {
    todo!();
}
/// CYCLE_WIDTH_RUBBER_MIN: If the cycle_width conditions are barely violated, use up this much rubber.If set to 1, the rubber usage rate is the same as if you were sitting in front of a wall.
pub fn cycle_width_rubber_min() {
    todo!();
}
/// CYCLE_WIDTH_RUBBER_MIN_OVERRIDE: Block out older clients if CYCLE_WIDTH_RUBBER_MIN differs from its default?
pub fn cycle_width_rubber_min_override() {
    todo!();
}
/// CYCLE_WIDTH_SIDE: Minimum distance of a cycle to a wall on either side before it takes harm.
pub fn cycle_width_side() {
    todo!();
}
/// CYCLE_WIDTH_SIDE_OVERRIDE: Block out older clients if CYCLE_WIDTH_SIDE differs from its default?
pub fn cycle_width_side_override() {
    todo!();
}
/// DECLARE_ROUND_WINNER: Declare the winner of the current round and end the round. Usage: DECLARE_ROUND_WINNER <player>
pub fn declare_round_winner(team: Team) {
    println!("{} {}", Command::DeclareRoundWinner, team);
}
/// DEDICATED_IDLE: After running this time (in hours), the dedicated server takes the next chance to quit.
pub fn dedicated_idle(hours: f32) {
    println!("{} {}", Command::DedicatedIdle, hours);
}
/// DEFAULT_KICK_REASON: The reason given to a player kicked by KICK if none is specified.
pub fn default_kick_reason(reason: &str) {
    println!("{} {}", Command::DefaultKickReason, reason);
}
/// DEFAULT_KICK_TO_PORT: Default server port a player is redirected to by KICK_TO and MOVE_TO.
pub fn default_kick_to_port(port: u16) {
    println!("{} {}", Command::DefaultKickToPort, port);
}
/// DEFAULT_KICK_TO_REASON: The reason given to a player kicked by KICK_TO or MOVE_TO if none is specified.
pub fn default_kick_to_reason(reason: &str) {
    println!("{} {}", Command::DefaultKickToReason, reason);
}
/// DEFAULT_KICK_TO_SERVER: Default server IP/name a player is redirected to by KICK_TO and MOVE_TO.
pub fn default_kick_to_server(server: IpAddr) {
    println!("{} {}", Command::DefaultKickToServer, server);
}
/// Combines DEFAULT_KICK_TO SERVER and PORT into one function
pub fn default_kick_to_server_port(server: SocketAddr) {
    default_kick_to_server(server.ip());
    default_kick_to_port(server.port());
}
/// DEFAULT_SHOUT_PLAYER: 1 if the default chat action for players should be shouting, 0 if it should be team chat. 2 if the default action should be shouting and the access level requirement should be overridden.
pub fn default_shout_player(enabled: bool) {
    println!("{} {}", Command::DefaultShoutPlayer, enabled.byte());
}
/// DEFAULT_SHOUT_SPECTATOR: 1 if the default chat action for spectators should be shouting, 0 if it should be spectator chat. 2 if the default action should be shouting and the access level requirement should be overridden.
pub fn default_shout_spectator(enabled: bool) {
    println!("{} {}", Command::DefaultShoutSpectator, enabled.byte());
}
/// DISALLOW_RENAME_PLAYER: Prevents the given player from rename-ing.
pub fn disallow_rename_player(disabled: bool) {
    println!("{} {}", Command::DisallowRenamePlayer, disabled);
}
/// DISALLOW_TEAM_CHANGE_PLAYER: Reverse of ALLOW_TEAM_CHANGE_PLAYER
pub fn disallow_team_change_player(disabled: bool) {
    println!("{} {}", Command::DisallowTeamChangePlayer, disabled);
}
/// DITHER: Use dithering
pub fn dither(enabled: bool) {
    println!("{} {}", Command::Dither, enabled.byte());
}
/// DOUBLEBIND_TIME: Time in seconds during which no two different keyboard events can trigger the same action
pub fn doublebind_time(seconds: f32) {
    println!("{} {}", Command::DoublebindTime, seconds);
}
/// DOUBLEBIND_TIME_OVERRIDE: Block out older clients when DOUBLEBIND_TIME differs from its default?
pub fn doublebind_time_override(block_older_clients: bool) {
    println!(
        "{} {}",
        Command::DoublebindTimeOverride,
        block_older_clients.byte()
    );
}
/// ENABLE_CHAT: If set to 0, all chat will be suppressed (if reset on the server, messages from logged in players and private/team messages are still shown)
pub fn enable_chat() {
    todo!();
}
/// ENABLE_FRIENDS: Turn on/off mates filtering.
pub fn enable_friends() {
    todo!();
}
/// ENEMY_CHATBOT_PENALTY: Penalty in seconds if the victim is in chatbot state and the enemy influence is just the chatbot evading a wall
pub fn enemy_chatbot_penalty() {
    todo!();
}
/// ENEMY_CURRENTTIME_INFLUENCE: If set to 1, not the build time of the encountered wall, but the current time enters the comparison of enemy influences. Arbitrary blending values are allowed.
pub fn enemy_currenttime_influence() {
    todo!();
}
/// ENEMY_DEAD_PENALTY: Penalty on the effective time in seconds if the enemy influence detection comes from a dead player
pub fn enemy_dead_penalty() {
    todo!();
}
/// ENEMY_SUICIDE_TIMEOUT: If no enemy influence can be found for the last this many seconds, a player's death counts as a suicide.
pub fn enemy_suicide_timeout() {
    todo!();
}
/// ENEMY_TEAMMATE_PENALTY: Penalty on the effective time in seconds if the enemy influence detection is from a teammate
pub fn enemy_teammate_penalty() {
    todo!();
}
/// EXPECT_ACK_ON_CLIENT_PLAYBACK: If 1, a client playing back a recording will expect correct ack responses from the server, at the default of 0 it tries to hobble along. Activate to accurately debug network problems.
pub fn expect_ack_on_client_playback() {
    todo!();
}
/// EXPLOSION: Enable explosions?
pub fn explosion() {
    todo!();
}
/// EXPLOSION_RADIUS: Blast radius of the cycle explosions
pub fn explosion_radius() {
    todo!();
}
/// EXTRA_ROUND_TIME: Length of an extra pause at the beginning of the round
pub fn extra_round_time() {
    todo!();
}
/// FADEOUT_NAME_DELAY: Time the player names are shown. Set to 0 if you don't want to show them at all or -1 if you want to show them always.
pub fn fadeout_name_delay() {
    todo!();
}
/// FAILED_ATTEMPTS: Number of failed attempts to initialize graphics mode
pub fn failed_attempts() {
    todo!();
}
/// FASTEST_LOCX: Horizontal position of the fastest player display
pub fn fastest_locx() {
    todo!();
}
/// FASTEST_LOCY: Vertical position of the fastest player display
pub fn fastest_locy() {
    todo!();
}
/// FASTEST_SIZE: Size of the fastest player display
pub fn fastest_size() {
    todo!();
}
/// FAST_FORWARD_MAXSTEP: Maximum recording time between rendered frames in fast forward mode
pub fn fast_forward_maxstep() {
    todo!();
}
/// FAST_FORWARD_MAXSTEP_REAL: Maximum real time between rendered frames in fast forward mode
pub fn fast_forward_maxstep_real() {
    todo!();
}
/// FAST_FORWARD_MAXSTEP_REL: Maximum fraction of the time left until the end of FF mode between rendered frames
pub fn fast_forward_maxstep_rel() {
    todo!();
}
/// FAV_NUM_PER_TEAM_PLAYER_1: The favorite number of players per team for this player
pub fn fav_num_per_team_player_1() {
    todo!();
}
/// FAV_NUM_PER_TEAM_PLAYER_2: The favorite number of players per team for this player
pub fn fav_num_per_team_player_2() {
    todo!();
}
/// FAV_NUM_PER_TEAM_PLAYER_3: The favorite number of players per team for this player
pub fn fav_num_per_team_player_3() {
    todo!();
}
/// FAV_NUM_PER_TEAM_PLAYER_4: The favorite number of players per team for this player
pub fn fav_num_per_team_player_4() {
    todo!();
}
/// FILTER_COLOR_NAMES: Filter color codes from player names.
pub fn filter_color_names() {
    todo!();
}
/// FILTER_COLOR_SERVER_NAMES: Filter color codes from server names in the server browser.
pub fn filter_color_server_names() {
    todo!();
}
/// FILTER_COLOR_STRINGS: Filter color codes from strings coming in over the network.
pub fn filter_color_strings() {
    todo!();
}
/// FILTER_COLOR_TEAM: Filter color codes from /team messages.
pub fn filter_color_team() {
    todo!();
}
/// FILTER_DARK_COLOR_NAMES: Filter dark color codes from player names.
pub fn filter_dark_color_names() {
    todo!();
}
/// FILTER_DARK_COLOR_SERVER_NAMES: Filter dark color codes from server names in the server browser.
pub fn filter_dark_color_server_names() {
    todo!();
}
/// FILTER_DARK_COLOR_STRINGS: Filter dark color codes from strings coming in over the network.
pub fn filter_dark_color_strings() {
    todo!();
}
/// FILTER_DARK_COLOR_TEAM: Filter dark color codes from /team messages.
pub fn filter_dark_color_team() {
    todo!();
}
/// FILTER_NAME_ENDS: Filter whitespace and other junk from beginnings and ends of player names.
pub fn filter_name_ends() {
    todo!();
}
/// FILTER_NAME_MIDDLE: Filter excess whitespace and other junk from the middle of player names.
pub fn filter_name_middle() {
    todo!();
}
/// FINISH_TYPE: What happens when the last human is dead?
pub fn finish_type() {
    todo!();
}
/// FIRST_USE: Is this the first time you use Armagetron Advanced?
pub fn first_use() {
    todo!();
}
/// FLOOR_BLUE: Floor color
pub fn floor_blue() {
    todo!();
}
/// FLOOR_DETAIL: Floor detail settings
pub fn floor_detail() {
    todo!();
}
/// FLOOR_GREEN: Floor color
pub fn floor_green() {
    todo!();
}
/// FLOOR_MIRROR: Floor mirror mode
pub fn floor_mirror() {
    todo!();
}
/// FLOOR_MIRROR_INT: Intensity of the floor mirror effect
pub fn floor_mirror_int() {
    todo!();
}
/// FLOOR_RED: Floor color
pub fn floor_red() {
    todo!();
}
/// FONT_SMALL_THRESHOLD_HEIGHT: Minimal pixel size of small font
pub fn font_small_threshold_height() {
    todo!();
}
/// FONT_SMALL_THRESHOLD_WIDTH: Minimal pixel size of small font
pub fn font_small_threshold_width() {
    todo!();
}
/// FORCE_TURTLE_MODE: Forces turtle mode, usually only active while a server is under attack. For testing, mainly.
pub fn force_turtle_mode() {
    todo!();
}
/// FORTRESS_COLLAPSE_SPEED: Speed a fortress zone collapses with
pub fn fortress_collapse_speed() {
    todo!();
}
/// FORTRESS_CONQUEST_DECAY_RATE: Rate a fortress zone "recovers" from being conquered
pub fn fortress_conquest_decay_rate() {
    todo!();
}
/// FORTRESS_CONQUEST_RATE: Rate a fortress zone gets conquered with for each enemy in it
pub fn fortress_conquest_rate() {
    todo!();
}
/// FORTRESS_CONQUEST_TIMEOUT: time without enemy contact that makes a fortress zone collapse harmlessly
pub fn fortress_conquest_timeout() {
    todo!();
}
/// FORTRESS_DEFEND_RATE: Rate a fortress zone "recovers" for each defending player
pub fn fortress_defend_rate() {
    todo!();
}
/// FORTRESS_HELD_SCORE: Score you get for holding your fortress.
pub fn fortress_held_score() {
    todo!();
}
/// FRIEND_1: A friend
pub fn friend_1() {
    todo!();
}
/// FRIEND_10: A friend
pub fn friend_10() {
    todo!();
}
/// FRIEND_2: A friend
pub fn friend_2() {
    todo!();
}
/// FRIEND_3: A friend
pub fn friend_3() {
    todo!();
}
/// FRIEND_4: A friend
pub fn friend_4() {
    todo!();
}
/// FRIEND_5: A friend
pub fn friend_5() {
    todo!();
}
/// FRIEND_6: A friend
pub fn friend_6() {
    todo!();
}
/// FRIEND_7: A friend
pub fn friend_7() {
    todo!();
}
/// FRIEND_8: A friend
pub fn friend_8() {
    todo!();
}
/// FRIEND_9: A friend
pub fn friend_9() {
    todo!();
}
/// FULLSCREEN: Fullscreen or windowed mode?
pub fn fullscreen() {
    todo!();
}
/// FULLSCREEN_MESSAGE: Prints a big message all over the screen, interrupting gameplay for a configurable timeout. Use with care.
pub fn fullscreen_message() {
    todo!();
}
/// GAME_TIMEOUT: Base timeout for game state synchronisation; gives approximately the maximum time between rounds.
pub fn game_timeout() {
    todo!();
}
/// GAME_TYPE: Type of game played. 0 for freestyle, 1 for last team standing and 2 for humans vs. AIs.
pub fn game_type() {
    todo!();
}
/// GLANCE_BACK_TOOLTIP: glance_back_tooltip_help
pub fn glance_back_tooltip() {
    todo!();
}
/// GLANCE_LEFT_TOOLTIP: glance_left_tooltip_help
pub fn glance_left_tooltip() {
    todo!();
}
/// GLANCE_RIGHT_TOOLTIP: glance_right_tooltip_help
pub fn glance_right_tooltip() {
    todo!();
}
/// GLOBAL_ID: If set to 1, Global IDs (Armathentication) will be enabled on this server.
pub fn global_id() {
    todo!();
}
/// GL_EXTENSIONS: OpenGL system information
pub fn gl_extensions() {
    todo!();
}
/// GL_RENDERER: OpenGL system information
pub fn gl_renderer() {
    todo!();
}
/// GL_VENDOR: OpenGL system information
pub fn gl_vendor() {
    todo!();
}
/// GL_VERSION: OpenGL system information
pub fn gl_version() {
    todo!();
}
/// GRID_SIZE: Distance between grid lines
pub fn grid_size() {
    todo!();
}
/// GRID_SIZE_MOVIEPACK: Distance between grid lines when moviepack is active
pub fn grid_size_moviepack() {
    todo!();
}
/// HASH_METHOD_BLACKLIST: List of hash authentication methods to disable support for.
pub fn hash_method_blacklist() {
    todo!();
}
/// HELP_INTRODUCTORY_BLURB: Message that is displayed before the list of help topics if someone uses /help without arguments
pub fn help_introductory_blurb() {
    todo!();
}
/// HIDE_IDENTITY_1: Should this player hide their ID?
pub fn hide_identity_1() {
    todo!();
}
/// HIDE_IDENTITY_2: Should this player hide their ID?
pub fn hide_identity_2() {
    todo!();
}
/// HIDE_IDENTITY_3: Should this player hide their ID?
pub fn hide_identity_3() {
    todo!();
}
/// HIDE_IDENTITY_4: Should this player hide their ID?
pub fn hide_identity_4() {
    todo!();
}
/// HIGH_RIM: Draw high rim walls
pub fn high_rim() {
    todo!();
}
/// HISTORY_SIZE_CHAT: Number of lines kept in the chat history.
pub fn history_size_chat() {
    todo!();
}
/// HISTORY_SIZE_CONSOLE: Number of lines kept in the console history.
pub fn history_size_console() {
    todo!();
}
/// HUD_CACHE_THRESHOLD: Update threshold for HUD gauges, only in effect when display lists are activated: if the relative change times the amount of time since the last update is bigger than this, the gauge gets updated.
pub fn hud_cache_threshold() {
    todo!();
}
/// HUD_MAX_WIDTH: The maximal width of the lower HUD relative to the screen height. 0 is the legacy setting that makes the HUD just stretch sideways and not adapt the height to screens that don't have a 4:3 aspect ratio, which is what versions 0.2.9.0 and earlier did.
pub fn hud_max_width() {
    todo!();
}
/// IDLE_KICK_TIME: Time in seconds after which an inactive player is kicked
pub fn idle_kick_time() {
    todo!();
}
/// IDLE_REMOVE_TIME: Time in seconds after which an inactive player is removed from the game
pub fn idle_remove_time() {
    todo!();
}
/// INCLUDE: Includes the following file
pub fn include(path: &Path) {
    if let Some(path) = path.to_str() {
        println!("{} {:?}", Command::Include, path);
    }
}
/// INFINITY_PLANE: Use infinite points (Does not work properly on most Windows systems)
pub fn infinity_plane() {
    todo!();
}
/// INGAME_MENU_TOOLTIP: ingame_menu_tooltip_help
pub fn ingame_menu_tooltip() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_1: Instant chat available with hotkeys
pub fn instant_chat_string_1_1() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_10: Instant chat available with hotkeys
pub fn instant_chat_string_1_10() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_11: Instant chat available with hotkeys
pub fn instant_chat_string_1_11() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_12: Instant chat available with hotkeys
pub fn instant_chat_string_1_12() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_13: Instant chat available with hotkeys
pub fn instant_chat_string_1_13() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_14: Instant chat available with hotkeys
pub fn instant_chat_string_1_14() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_15: Instant chat available with hotkeys
pub fn instant_chat_string_1_15() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_16: Instant chat available with hotkeys
pub fn instant_chat_string_1_16() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_17: Instant chat available with hotkeys
pub fn instant_chat_string_1_17() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_18: Instant chat available with hotkeys
pub fn instant_chat_string_1_18() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_19: Instant chat available with hotkeys
pub fn instant_chat_string_1_19() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_2: Instant chat available with hotkeys
pub fn instant_chat_string_1_2() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_20: Instant chat available with hotkeys
pub fn instant_chat_string_1_20() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_21: Instant chat available with hotkeys
pub fn instant_chat_string_1_21() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_22: Instant chat available with hotkeys
pub fn instant_chat_string_1_22() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_23: Instant chat available with hotkeys
pub fn instant_chat_string_1_23() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_24: Instant chat available with hotkeys
pub fn instant_chat_string_1_24() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_25: Instant chat available with hotkeys
pub fn instant_chat_string_1_25() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_3: Instant chat available with hotkeys
pub fn instant_chat_string_1_3() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_4: Instant chat available with hotkeys
pub fn instant_chat_string_1_4() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_5: Instant chat available with hotkeys
pub fn instant_chat_string_1_5() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_6: Instant chat available with hotkeys
pub fn instant_chat_string_1_6() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_7: Instant chat available with hotkeys
pub fn instant_chat_string_1_7() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_8: Instant chat available with hotkeys
pub fn instant_chat_string_1_8() {
    todo!();
}
/// INSTANT_CHAT_STRING_1_9: Instant chat available with hotkeys
pub fn instant_chat_string_1_9() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_1: Instant chat available with hotkeys
pub fn instant_chat_string_2_1() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_10: Instant chat available with hotkeys
pub fn instant_chat_string_2_10() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_11: Instant chat available with hotkeys
pub fn instant_chat_string_2_11() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_12: Instant chat available with hotkeys
pub fn instant_chat_string_2_12() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_13: Instant chat available with hotkeys
pub fn instant_chat_string_2_13() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_14: Instant chat available with hotkeys
pub fn instant_chat_string_2_14() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_15: Instant chat available with hotkeys
pub fn instant_chat_string_2_15() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_16: Instant chat available with hotkeys
pub fn instant_chat_string_2_16() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_17: Instant chat available with hotkeys
pub fn instant_chat_string_2_17() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_18: Instant chat available with hotkeys
pub fn instant_chat_string_2_18() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_19: Instant chat available with hotkeys
pub fn instant_chat_string_2_19() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_2: Instant chat available with hotkeys
pub fn instant_chat_string_2_2() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_20: Instant chat available with hotkeys
pub fn instant_chat_string_2_20() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_21: Instant chat available with hotkeys
pub fn instant_chat_string_2_21() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_22: Instant chat available with hotkeys
pub fn instant_chat_string_2_22() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_23: Instant chat available with hotkeys
pub fn instant_chat_string_2_23() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_24: Instant chat available with hotkeys
pub fn instant_chat_string_2_24() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_25: Instant chat available with hotkeys
pub fn instant_chat_string_2_25() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_3: Instant chat available with hotkeys
pub fn instant_chat_string_2_3() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_4: Instant chat available with hotkeys
pub fn instant_chat_string_2_4() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_5: Instant chat available with hotkeys
pub fn instant_chat_string_2_5() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_6: Instant chat available with hotkeys
pub fn instant_chat_string_2_6() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_7: Instant chat available with hotkeys
pub fn instant_chat_string_2_7() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_8: Instant chat available with hotkeys
pub fn instant_chat_string_2_8() {
    todo!();
}
/// INSTANT_CHAT_STRING_2_9: Instant chat available with hotkeys
pub fn instant_chat_string_2_9() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_1: Instant chat available with hotkeys
pub fn instant_chat_string_3_1() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_10: Instant chat available with hotkeys
pub fn instant_chat_string_3_10() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_11: Instant chat available with hotkeys
pub fn instant_chat_string_3_11() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_12: Instant chat available with hotkeys
pub fn instant_chat_string_3_12() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_13: Instant chat available with hotkeys
pub fn instant_chat_string_3_13() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_14: Instant chat available with hotkeys
pub fn instant_chat_string_3_14() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_15: Instant chat available with hotkeys
pub fn instant_chat_string_3_15() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_16: Instant chat available with hotkeys
pub fn instant_chat_string_3_16() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_17: Instant chat available with hotkeys
pub fn instant_chat_string_3_17() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_18: Instant chat available with hotkeys
pub fn instant_chat_string_3_18() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_19: Instant chat available with hotkeys
pub fn instant_chat_string_3_19() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_2: Instant chat available with hotkeys
pub fn instant_chat_string_3_2() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_20: Instant chat available with hotkeys
pub fn instant_chat_string_3_20() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_21: Instant chat available with hotkeys
pub fn instant_chat_string_3_21() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_22: Instant chat available with hotkeys
pub fn instant_chat_string_3_22() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_23: Instant chat available with hotkeys
pub fn instant_chat_string_3_23() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_24: Instant chat available with hotkeys
pub fn instant_chat_string_3_24() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_25: Instant chat available with hotkeys
pub fn instant_chat_string_3_25() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_3: Instant chat available with hotkeys
pub fn instant_chat_string_3_3() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_4: Instant chat available with hotkeys
pub fn instant_chat_string_3_4() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_5: Instant chat available with hotkeys
pub fn instant_chat_string_3_5() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_6: Instant chat available with hotkeys
pub fn instant_chat_string_3_6() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_7: Instant chat available with hotkeys
pub fn instant_chat_string_3_7() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_8: Instant chat available with hotkeys
pub fn instant_chat_string_3_8() {
    todo!();
}
/// INSTANT_CHAT_STRING_3_9: Instant chat available with hotkeys
pub fn instant_chat_string_3_9() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_1: Instant chat available with hotkeys
pub fn instant_chat_string_4_1() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_10: Instant chat available with hotkeys
pub fn instant_chat_string_4_10() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_11: Instant chat available with hotkeys
pub fn instant_chat_string_4_11() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_12: Instant chat available with hotkeys
pub fn instant_chat_string_4_12() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_13: Instant chat available with hotkeys
pub fn instant_chat_string_4_13() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_14: Instant chat available with hotkeys
pub fn instant_chat_string_4_14() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_15: Instant chat available with hotkeys
pub fn instant_chat_string_4_15() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_16: Instant chat available with hotkeys
pub fn instant_chat_string_4_16() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_17: Instant chat available with hotkeys
pub fn instant_chat_string_4_17() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_18: Instant chat available with hotkeys
pub fn instant_chat_string_4_18() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_19: Instant chat available with hotkeys
pub fn instant_chat_string_4_19() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_2: Instant chat available with hotkeys
pub fn instant_chat_string_4_2() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_20: Instant chat available with hotkeys
pub fn instant_chat_string_4_20() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_21: Instant chat available with hotkeys
pub fn instant_chat_string_4_21() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_22: Instant chat available with hotkeys
pub fn instant_chat_string_4_22() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_23: Instant chat available with hotkeys
pub fn instant_chat_string_4_23() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_24: Instant chat available with hotkeys
pub fn instant_chat_string_4_24() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_25: Instant chat available with hotkeys
pub fn instant_chat_string_4_25() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_3: Instant chat available with hotkeys
pub fn instant_chat_string_4_3() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_4: Instant chat available with hotkeys
pub fn instant_chat_string_4_4() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_5: Instant chat available with hotkeys
pub fn instant_chat_string_4_5() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_6: Instant chat available with hotkeys
pub fn instant_chat_string_4_6() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_7: Instant chat available with hotkeys
pub fn instant_chat_string_4_7() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_8: Instant chat available with hotkeys
pub fn instant_chat_string_4_8() {
    todo!();
}
/// INSTANT_CHAT_STRING_4_9: Instant chat available with hotkeys
pub fn instant_chat_string_4_9() {
    todo!();
}
/// KEEP_PLAYER_SLOT: If set to 1, every time the server gets full, an unworthy spectator is kicked.
pub fn keep_player_slot() {
    todo!();
}
/// KEEP_WINDOW_ACTIVE: Keeps rendering active when the program window loses input focus.
pub fn keep_window_active() {
    todo!();
}
/// KEYBOARD: Keyboard settings
pub fn keyboard() {
    todo!();
}
/// KICK: Kicks the specified player from the server.
pub fn kick() {
    todo!();
}
/// KICK_TO: Kicks the specified player from the server and, if the client supports it, redirects them to a different server.
pub fn kick_to() {
    todo!();
}
/// KILL: Kill a specific player (as warning before a kick)
pub fn kill(player: &Player) {
    println!("{} {}", Command::Kill, player.0);
}
/// LADDERLOG_DECORATE_TIMESTAMP: Decorates every line of ladderlog output with the current date and time
pub fn ladderlog_decorate_timestamp() {
    todo!();
}
/// LADDERLOG_GAME_TIME_INTERVAL: If non-negative, write a line with the current game time to the ladder log every n seconds.
pub fn ladderlog_game_time_interval() {
    todo!();
}
/// LADDERLOG_WRITE_ALL: Set all the LADDER_LOG_WRITE_* settings to the same value
pub fn ladderlog_write_all(value: bool) {
    println!("{} {}", Command::LadderlogWriteAll, value.byte())
}
/// LADDERLOG_WRITE_AUTHORITY_BLURB: Write to ladderlog: AUTHORITY_BLURB <blurb> <player> <text>
pub fn ladderlog_write_authority_blurb(value: bool) {
    println!("{} {}", Command::LadderlogWriteAuthorityBlurb, value.byte())
}
/// LADDERLOG_WRITE_BASEZONE_CONQUERED: Write to ladderlog: BASEZONE_CONQUERED <team> <cx> <cy>
pub fn ladderlog_write_basezone_conquered(value: bool) {
    println!(
        "{} {}",
        Command::LadderlogWriteBasezoneConquered,
        value.byte()
    )
}
/// LADDERLOG_WRITE_BASEZONE_CONQUERER: Write to ladderlog: BASEZONE_CONQUERER <player>
pub fn ladderlog_write_basezone_conquerer(value: bool) {
    println!(
        "{} {}",
        Command::LadderlogWriteBasezoneConquerer,
        value.byte()
    )
}
/// LADDERLOG_WRITE_CHAT: Write to ladderlog: CHAT \<chatter> \[/me] \<chat string>
pub fn ladderlog_write_chat(value: bool) {
    println!("{} {}", Command::LadderlogWriteChat, value.byte())
}
/// LADDERLOG_WRITE_DEATH_FRAG: Write to ladderlog: DEATH_FRAG \<prey> \<predator>
pub fn ladderlog_write_death_frag(value: bool) {
    println!("{} {}", Command::LadderlogWriteDeathFrag, value.byte())
}
/// LADDERLOG_WRITE_DEATH_SUICIDE: Write to ladderlog: DEATH_SUICIDE \<player>
pub fn ladderlog_write_death_suicide(value: bool) {
    println!("{} {}", Command::LadderlogWriteDeathSuicide, value.byte())
}
/// LADDERLOG_WRITE_DEATH_TEAMKILL: Write to ladderlog: DEATH_TEAMKILL \<prey> \<predator>
pub fn ladderlog_write_death_teamkill(value: bool) {
    println!("{} {}", Command::LadderlogWriteDeathTeamkill, value.byte())
}
/// LADDERLOG_WRITE_ENCODING: Write to ladderlog: ENCODING \<charset>. Specifies the encoding for data in ladderlog.txt.
pub fn ladderlog_write_encoding(value: bool) {
    println!("{} {}", Command::LadderlogWriteEncoding, value.byte())
}
/// LADDERLOG_WRITE_GAME_END: Write to ladderlog: GAME_END \<date and time>
pub fn ladderlog_write_game_end(value: bool) {
    println!("{} {}", Command::LadderlogWriteGameEnd, value.byte())
}
/// LADDERLOG_WRITE_GAME_TIME: Write to ladderlog: GAME_TIME \<time> (see also: GAME_TIME_INTERVAL)
pub fn ladderlog_write_game_time(value: bool) {
    println!("{} {}", Command::LadderlogWriteGameTime, value.byte())
}
/// LADDERLOG_WRITE_MATCH_WINNER: Write to ladderlog: MATCH_WINNER \<team> \<players>
pub fn ladderlog_write_match_winner(value: bool) {
    println!("{} {}", Command::LadderlogWriteMatchWinner, value.byte())
}
/// LADDERLOG_WRITE_NEW_MATCH: Write to ladderlog: NEW_MATCH \<date and time>
pub fn ladderlog_write_new_match(value: bool) {
    println!("{} {}", Command::LadderlogWriteNewMatch, value.byte())
}
/// LADDERLOG_WRITE_NEW_ROUND: Write to ladderlog: NEW_ROUND <date and time>
pub fn ladderlog_write_new_round(value: bool) {
    println!("{} {}", Command::LadderlogWriteNewRound, value.byte())
}
/// LADDERLOG_WRITE_NUM_HUMANS: Write to ladderlog: NUM_HUMANS <number of humans>
pub fn ladderlog_write_num_humans(value: bool) {
    println!("{} {}", Command::LadderlogWriteNumHumans, value.byte())
}
/// LADDERLOG_WRITE_ONLINE_PLAYER: Write to ladderlog: ONLINE_PLAYER \<name> \[\<ping> \[\<team>]]
pub fn ladderlog_write_online_player(value: bool) {
    println!("{} {}", Command::LadderlogWriteOnlinePlayer, value.byte())
}
/// LADDERLOG_WRITE_PLAYER_ENTERED: Write to ladderlog: PLAYER_ENTERED \<name> \<IP> \<screen name>
pub fn ladderlog_write_player_entered(value: bool) {
    println!("{} {}", Command::LadderlogWritePlayerEntered, value.byte())
}
/// LADDERLOG_WRITE_PLAYER_LEFT: Write to ladderlog: PLAYER_LEFT \<name> \<IP>
pub fn ladderlog_write_player_left(value: bool) {
    println!("{} {}", Command::LadderlogWritePlayerLeft, value.byte())
}
/// LADDERLOG_WRITE_PLAYER_RENAMED: Write to ladderlog: PLAYER_RENAMED \<old name> \<new name> \<ip> \<screen name>
pub fn ladderlog_write_player_renamed(value: bool) {
    println!("{} {}", Command::LadderlogWritePlayerRenamed, value.byte())
}
/// LADDERLOG_WRITE_POSITIONS: Write to ladderlog: POSITIONS <team> <player1 player2 ...>
pub fn ladderlog_write_positions(value: bool) {
    println!("{} {}", Command::LadderlogWritePositions, value.byte())
}
/// LADDERLOG_WRITE_ROUND_SCORE: Write to ladderlog: ROUND_SCORE <score difference> \<player> \[\<team>]
pub fn ladderlog_write_round_score(value: bool) {
    println!("{} {}", Command::LadderlogWriteRoundScore, value.byte())
}
/// LADDERLOG_WRITE_ROUND_SCORE_TEAM: Write to ladderlog: ROUND_SCORE_TEAM \<score difference> \<team>
pub fn ladderlog_write_round_score_team(value: bool) {
    println!("{} {}", Command::LadderlogWriteRoundScoreTeam, value.byte())
}
/// LADDERLOG_WRITE_ROUND_WINNER: Write to ladderlog: ROUND_WINNER \<team> \<players>
pub fn ladderlog_write_round_winner(value: bool) {
    println!("{} {}", Command::LadderlogWriteRoundWinner, value.byte())
}
/// LADDERLOG_WRITE_SACRIFICE: Write to ladderlog: SACRIFICE \<player who used the hole> \<player who created the hole> \<player owning the wall the hole was made into>
pub fn ladderlog_write_sacrifice(value: bool) {
    println!("{} {}", Command::LadderlogWriteSacrifice, value.byte())
}
/// LADDERLOG_WRITE_TEAM_CREATED: Write to ladderlog: TEAM_CREATED \<team name>
pub fn ladderlog_write_team_created(value: bool) {
    println!("{} {}", Command::LadderlogWriteTeamCreated, value.byte())
}
/// LADDERLOG_WRITE_TEAM_DESTROYED: Write to ladderlog: TEAM_DESTROYED \<team name>
pub fn ladderlog_write_team_destroyed(value: bool) {
    println!("{} {}", Command::LadderlogWriteTeamDestroyed, value.byte())
}
/// LADDERLOG_WRITE_TEAM_PLAYER_ADDED: Write to ladderlog: TEAM_PLAYER_ADDED \<team name> \<player>
pub fn ladderlog_write_team_player_added(value: bool) {
    println!(
        "{} {}",
        Command::LadderlogWriteTeamPlayerAdded,
        value.byte()
    )
}
/// LADDERLOG_WRITE_TEAM_PLAYER_REMOVED: Write to ladderlog: TEAM_PLAYER_REMOVED \<team name> \<player>
pub fn ladderlog_write_team_player_removed(value: bool) {
    println!(
        "{} {}",
        Command::LadderlogWriteTeamPlayerRemoved,
        value.byte()
    )
}
/// LADDERLOG_WRITE_TEAM_RENAMED: Write to ladderlog: TEAM_RENAMED \<old team name> \<new team name>
pub fn ladderlog_write_team_renamed(value: bool) {
    println!("{} {}", Command::LadderlogWriteTeamRenamed, value.byte())
}
/// LADDERLOG_WRITE_WAIT_FOR_EXTERNAL_SCRIPT: Write to ladderlog: WAIT_FOR_EXTERNAL_SCRIPT (see also: WAIT_FOR_EXTERNAL_SCRIPT and WAIT_FOR_EXTERNAL_SCRIPT_TIMEOUT)
pub fn ladderlog_write_wait_for_external_script(value: bool) {
    println!(
        "{} {}",
        Command::LadderlogWriteWaitForExternalScript,
        value.byte()
    )
}
/// LADDER_GAIN_EXTRA: Ping dependent ladder extra score for the winner
pub fn ladder_gain_extra() {
    todo!();
}
/// LADDER_LOSE_MIN_ON_LOAD: Minimum of you ladder score lost on each load
pub fn ladder_lose_min_on_load() {
    todo!();
}
/// LADDER_LOSE_PERCENT_ON_LOAD: Percentage of your ladder score lost on each load
pub fn ladder_lose_percent_on_load() {
    todo!();
}
/// LADDER_MIN_BET: Minimum score you put in the ladder pot
pub fn ladder_min_bet() {
    todo!();
}
/// LADDER_PERCENT_BET: Percentage of your score you put in the ladder pot
pub fn ladder_percent_bet() {
    todo!();
}
/// LADDER_TAX: Percentage of the ladder pot the IRS takes
pub fn ladder_tax() {
    todo!();
}
/// LAG_CREDIT: Maximal seconds of total lag credit.
pub fn lag_credit() {
    todo!();
}
/// LAG_CREDIT_SINGLE: Maximal seconds of lag credit for a single lag credit event.
pub fn lag_credit_single() {
    todo!();
}
/// LAG_CREDIT_TIME: Timescale lag credit is restored on.
pub fn lag_credit_time() {
    todo!();
}
/// LAG_CREDIT_VARIANCE: Maximal multiple of the lag variance for a single lag credit event.
pub fn lag_credit_variance() {
    todo!();
}
/// LAG_FAST_TIME: Timescale the fast lag measurement decays on.
pub fn lag_fast_time() {
    todo!();
}
/// LAG_FAST_WEIGHT: Extra weight lag reports from the server influence the fast lag compensation with.
pub fn lag_fast_weight() {
    todo!();
}
/// LAG_FREQUENCY_THRESHOLD: Minimal frequency of lag events (measured against the total number of input events) that needs to be exceeded before the server informs the client. Should be between 0 and 1.
pub fn lag_frequency_threshold() {
    todo!();
}
/// LAG_MAX_SPEEDUP_TIMER: Maximal speed increase of timer while lag is compensated for.
pub fn lag_max_speedup_timer() {
    todo!();
}
/// LAG_OFFSET_CLIENT: Extra amount of lag compensation, determined by the client.
pub fn lag_offset_client() {
    todo!();
}
/// LAG_OFFSET_LEGACY: Extra amount of lag compensation for clients that don't support automatic compensation, determined by the server.
pub fn lag_offset_legacy() {
    todo!();
}
/// LAG_OFFSET_SERVER: Extra amount of lag compensation, determined by the server.
pub fn lag_offset_server() {
    todo!();
}
/// LAG_O_METER: Draw Lag-O-Meter in network play
pub fn lag_o_meter() {
    todo!();
}
/// LAG_O_METER_BLEND: Amount the player color should be blended with white to get the color of the Lag-O-Meter. 1 means white, 0 means the color of the player.
pub fn lag_o_meter_blend() {
    todo!();
}
/// LAG_O_METER_SCALE: Scale of the Lag-O-Meter. 1.0 is the "correct" value, older clients were hardcoded to .5 due to a bug.
pub fn lag_o_meter_scale() {
    todo!();
}
/// LAG_O_METER_THRESHOLD: The Lag-O-Meter will only be drawn if the product of cycle speed and lag is bigger than this value.
pub fn lag_o_meter_threshold() {
    todo!();
}
/// LAG_O_METER_USE_OLD: Should we use the old buggy Lag-O-Meter? This functionality will go away soon.
pub fn lag_o_meter_use_old() {
    todo!();
}
/// LAG_SLOW_TIME: Timescale the slow lag measurement decays on.
pub fn lag_slow_time() {
    todo!();
}
/// LAG_SLOW_WEIGHT: Extra weight lag reports from the server influence the slow lag compensation with.
pub fn lag_slow_weight() {
    todo!();
}
/// LAG_SWEET_SPOT: Sweet spot, the fill ratio of lag credit the server tries to keep the client at.
pub fn lag_sweet_spot() {
    todo!();
}
/// LAG_THRESHOLD: Amount of lag not compensated for on each lag event.
pub fn lag_threshold() {
    todo!();
}
/// LANGUAGE_FIRST: The language Armagetron Advanced will use
pub fn language_first() {
    todo!();
}
/// LANGUAGE_SECOND: Fallback language if the first language is not available
pub fn language_second() {
    todo!();
}
/// LAST_CHAT_BREAK_TIME: Last round time a player in chat mode is able to pause the timer
pub fn last_chat_break_time() {
    todo!();
}
/// LAST_CHECK_ERRORS: Listen to errors claiming a video mode does not exist, last successful init
pub fn last_check_errors() {
    todo!();
}
/// LAST_COLORDEPTH: Color depth, last successful init
pub fn last_colordepth() {
    todo!();
}
/// LAST_FULLSCREEN: Fullscreen or windowed mode, last successful init
pub fn last_fullscreen() {
    todo!();
}
/// LAST_ZDEPTH: z buffer depth, last successful init
pub fn last_zdepth() {
    todo!();
}
/// LEGACY_LOG_NAMES: If 1, the log names of unauthenticated players are kept like they were before authentication was implemented. If 0, log names are escaped so that authenticated player names look best.
pub fn legacy_log_names() {
    todo!();
}
/// LIMIT_ROUNDS: End the match after this number of rounds
pub fn limit_rounds(rounds: u32) {
    println!("{} {}", Command::LimitRounds, rounds)
}
/// LIMIT_SCORE: End the match when a player reaches this score
pub fn limit_score(score: u32) {
    println!("{} {}", Command::LimitScore, score)
}
/// LIMIT_TIME: End the match after this number of minutes
pub fn limit_time(minutes: u32) {
    println!("{} {}", Command::LimitTime, minutes)
}
/// LOCAL_TEAM: Adds a local account for an entire team (team tags are compared).
pub fn local_team() {
    todo!();
}
/// LOCAL_USER: Adds a local user account from a name/password pair.
pub fn local_user() {
    todo!();
}
/// LOWER_SKY: Draw lower sky plane
pub fn lower_sky() {
    todo!();
}
/// MAP_FILE: File that contains the map used for playing
pub fn map_file(path: &Path) {
    if let Some(path) = path.to_str() {
        println!("{} {:?}", Command::MapFile, path);
    }
}
/// MAP_FILE_OVERRIDE: Block out older clients when MAP_FILE differs from its default?
pub fn map_file_override() {
    todo!();
}
/// MAP_URI: DEPRECIATED - use RESOURCE_REPOSITORY_SERVER and MAP_FILE instead
#[deprecated]
pub fn map_uri() {
    todo!();
}
/// MAX_CLIENTS: Maximum number of network clients to accept
pub fn max_clients() {
    todo!();
}
/// MAX_CLIENTS_SAME_IP_HARD: Maximum number of network clients to accept from the same IP; more logins will be ignored
pub fn max_clients_same_ip_hard() {
    todo!();
}
/// MAX_CLIENTS_SAME_IP_SOFT: Maximum number of network clients to accept from the same IP; more logins will get kicked when the server is full
pub fn max_clients_same_ip_soft() {
    todo!();
}
/// MAX_IN_RATE: Maximum network input rate
pub fn max_in_rate() {
    todo!();
}
/// MAX_OUT_RATE: Maximum network output rate
pub fn max_out_rate() {
    todo!();
}
/// MAX_PLAYERS_SAME_IP: maximum number of players from the same IP (note that each client can legally host up to four players)
pub fn max_players_same_ip() {
    todo!();
}
/// MAX_PROTOCOL_VERSION: If > 0, maximum protocol version allowed to play; features not supported by this version are going to be permanently disabled.
pub fn max_protocol_version() {
    todo!();
}
/// MAX_VOTES: The maximum number of total votes that can be active at any given moment.
pub fn max_votes() {
    todo!();
}
/// MAX_VOTES_PER_VOTER: The maximum number of votes suggested by each voter that can be active at any given moment.
pub fn max_votes_per_voter() {
    todo!();
}
/// MD5_PREFIX: Extra hash prefix for local accounts used to scramble the password
pub fn md5_prefix() {
    todo!();
}
/// MD5_SUFFIX: Extra hash suffix for local accounts used to scramble the password
pub fn md5_suffix() {
    todo!();
}
/// MESSAGE_OF_DAY: Message sent to clients on connection, if supported by the client, it will be displayed fullscreen
pub fn message_of_day() {
    todo!();
}
/// MESSAGE_OF_DAY_TIMEOUT: Time message_of_day is displayed for in fullscreen mode
pub fn message_of_day_timeout() {
    todo!();
}
/// MIN_PLAYERS: Minimum number of players
pub fn min_players() {
    todo!();
}
/// MIN_PLAY_TIME_ONLINE: Online play time in minutes required to play here
pub fn min_play_time_online() {
    todo!();
}
/// MIN_PLAY_TIME_TEAM: Team play time in minutes required to play here
pub fn min_play_time_team() {
    todo!();
}
/// MIN_PLAY_TIME_TOTAL: Total play time in minutes required to play here
pub fn min_play_time_total() {
    todo!();
}
/// MIN_PROTOCOL_VERSION: Minimum protocol version allowed to play.
pub fn min_protocol_version() {
    todo!();
}
/// MIN_VOTERS: Number of voters that need to be online to enable voting.
pub fn min_voters() {
    todo!();
}
/// MOUSE_GRAB: Grab the mouse pointer, so it can't leave the window
pub fn mouse_grab() {
    todo!();
}
/// MOVE_TO: Kicks the specified player from the server and, if the client supports it, redirects them to a different server. Does not imply an autoban penalty.
pub fn move_to() {
    todo!();
}
/// MOVIEPACK: Use the moviepack if available
pub fn moviepack() {
    todo!();
}
/// MOVIEPACK_FLOOR_BLUE: Floor color
pub fn moviepack_floor_blue() {
    todo!();
}
/// MOVIEPACK_FLOOR_GREEN: Floor color
pub fn moviepack_floor_green() {
    todo!();
}
/// MOVIEPACK_FLOOR_RED: Floor color
pub fn moviepack_floor_red() {
    todo!();
}
/// MOVIEPACK_RIM_WALL_STRETCH_X: Extension of one square of rim wall texture in the horizontal direction for the moviepack
pub fn moviepack_rim_wall_stretch_x() {
    todo!();
}
/// MOVIEPACK_RIM_WALL_STRETCH_Y: Extension of the rim wall texture in the vertical direction for the moviepack
pub fn moviepack_rim_wall_stretch_y() {
    todo!();
}
/// MOVIEPACK_WALL_STRETCH: The distance of the vertical lines on the moviepack walls
pub fn moviepack_wall_stretch() {
    todo!();
}
/// NAME_TEAM_AFTER_PLAYER_1: If set, the team is named after the leading player
pub fn name_team_after_player_1() {
    todo!();
}
/// NAME_TEAM_AFTER_PLAYER_2: If set, the team is named after the leading player
pub fn name_team_after_player_2() {
    todo!();
}
/// NAME_TEAM_AFTER_PLAYER_3: If set, the team is named after the leading player
pub fn name_team_after_player_3() {
    todo!();
}
/// NAME_TEAM_AFTER_PLAYER_4: If set, the team is named after the leading player
pub fn name_team_after_player_4() {
    todo!();
}
/// NETWORK_AUTOBAN_FACTOR: Autoban players for NETWORK_AUTOBAN_FACTOR * ( kph - NETWORK_AUTOBAN_OFFSET ) minutes when they get kicked; kph is the average number of kicks per hour they get.
pub fn network_autoban_factor() {
    todo!();
}
/// NETWORK_AUTOBAN_MAX_KPH: Maximal value of the kicks per hour; larger values are smoothly clamped.
pub fn network_autoban_max_kph() {
    todo!();
}
/// NETWORK_AUTOBAN_OFFSET: Autoban players for NETWORK_AUTOBAN_FACTOR * ( kph - NETWORK_AUTOBAN_OFFSET ) minutes when they get kicked; kph is the average number of kicks per hour they get.
pub fn network_autoban_offset() {
    todo!();
}
/// NETWORK_MIN_BAN: When a client's connection is blocked because they're banned, make them banned for at least this many seconds.
pub fn network_min_ban() {
    todo!();
}
/// NETWORK_SPECTATOR_TIME: If set to something bigger than zero, this is the maximal time in seconds a client without players is tolerated.
pub fn network_spectator_time() {
    todo!();
}
/// NEW_FEATURE_DELAY: Disable features that only came in during the last X protocol versions.
pub fn new_feature_delay() {
    todo!();
}
/// NEW_TEAM_ALLOWED: Is it currently allowed to create a new team?
pub fn new_team_allowed(value: bool) {
    println!("{} {}", Command::NewTeamAllowed, value.byte());
}
/// NUM_AIS: Number of AI players
pub fn num_ais() {
    todo!();
}
/// PASSWORD: Password setting
pub fn password() {
    todo!();
}
/// PASSWORD_STORAGE: Determines where your passwords are stored: 1 means on hard disk (dangerous), 0 in memory and -1 means they are not stored at all.
pub fn password_storage() {
    todo!();
}
/// PING_CHARITY: How much ping are you willing to take over from your opponent?
pub fn ping_charity() {
    todo!();
}
/// PING_CHARITY_MAX: Server option: maximum ping charity value. Set to 0 to avoid instant kills. Active only if all clients are 0.2.8.3 or better.
pub fn ping_charity_max() {
    todo!();
}
/// PING_CHARITY_MIN: Server option: minimum ping charity value. Use to enforce fairness. Active only if all clients are 0.2.8.3 or better.
pub fn ping_charity_min() {
    todo!();
}
/// PING_CHARITY_SERVER: Don't touch: the server says this is the maximal ping compensation.
pub fn ping_charity_server() {
    todo!();
}
/// PING_FLOOD_GLOBAL: The times PING_FLOOD_TIME_X, multiplied by this value, count for all pings from all machines. Negative values disable global flood protection.
pub fn ping_flood_global() {
    todo!();
}
/// PING_FLOOD_TIME_10: Minimum time for 10 ping packets from one machine to arrive.
pub fn ping_flood_time_10() {
    todo!();
}
/// PING_FLOOD_TIME_100: Minimum time for 100 ping packets from one machine to arrive.
pub fn ping_flood_time_100() {
    todo!();
}
/// PING_FLOOD_TIME_20: Minimum time for 20 ping packets from one machine to arrive.
pub fn ping_flood_time_20() {
    todo!();
}
/// PING_FLOOD_TIME_50: Minimum time for 50 ping packets from one machine to arrive.
pub fn ping_flood_time_50() {
    todo!();
}
/// PING_LOCX: Horizontal position of the ping display
pub fn ping_locx() {
    todo!();
}
/// PING_LOCY: Vertical position of the ping display
pub fn ping_locy() {
    todo!();
}
/// PING_SIZE: Size of the ping display
pub fn ping_size() {
    todo!();
}
/// PLAYERS: Prints list of currently active players
pub fn players() {
    todo!();
}
/// PLAYER_1: Player name
pub fn player_1() {
    todo!();
}
/// PLAYER_2: Player name
pub fn player_2() {
    todo!();
}
/// PLAYER_3: Player name
pub fn player_3() {
    todo!();
}
/// PLAYER_4: Player name
pub fn player_4() {
    todo!();
}
/// PLAYER_CHAT_WAIT_FRACTION: Maximum fraction of time to wait for a single player to stop chatting.
pub fn player_chat_wait_fraction() {
    todo!();
}
/// PLAYER_CHAT_WAIT_MAX: Maximum time in seconds to wait for a single player to stop chatting.
pub fn player_chat_wait_max() {
    todo!();
}
/// PLAYER_CHAT_WAIT_SINGLE: Set to 1 if only one player should get their chat wait time reduced at any given time.
pub fn player_chat_wait_single() {
    todo!();
}
/// PLAYER_CHAT_WAIT_TEAMLEADER: Set to 1 if only team leaders, and 0 if all players, should be allowed to pause the timer.
pub fn player_chat_wait_teamleader() {
    todo!();
}
/// PLAYER_LIST_HIDDEN_PLAYER_PREFIX: The prefix that is shown on hidden players' Global ID and access level when we can see it.
pub fn player_list_hidden_player_prefix() {
    todo!();
}
/// PLAYER_MESSAGE: Sends a message to a specified player.
pub fn player_message() {
    todo!();
}
/// PLAYER_RANDOM_COLOR: If set to 1, each local player will receive a semi-random color every round, trying to get as far away from every other player color.
pub fn player_random_color() {
    todo!();
}
/// PLAY_TIME_ONLINE: Total time in minutes someone has played with this client online
pub fn play_time_online() {
    todo!();
}
/// PLAY_TIME_TEAM: Total time in minutes someone has played with this client in a team
pub fn play_time_team() {
    todo!();
}
/// PLAY_TIME_TOTAL: Total time in minutes someone has played with this client
pub fn play_time_total() {
    todo!();
}
/// PNG_SCREENSHOT: Store screenshots as PNG files, not BMP files.
pub fn png_screenshot() {
    todo!();
}
/// PREDICT_OBJECTS: Predict cycle movement in network play
pub fn predict_objects() {
    todo!();
}
/// PREFIX_SPAM_ENABLE: Should spam prefix checking be enabled? Set to 1 to enable, 0 to disable.
pub fn prefix_spam_enable() {
    todo!();
}
/// PREFIX_SPAM_LENGTH_MULTIPLIER: Multiplier applied to prefix length when calculating prefix spam score.
pub fn prefix_spam_length_multiplier() {
    todo!();
}
/// PREFIX_SPAM_NUMBER_COLOR_CODES_MULTIPLIER: Multiplier applied to the number of color codes in prefix when calculating prefix spam score.
pub fn prefix_spam_number_color_codes_multiplier() {
    todo!();
}
/// PREFIX_SPAM_NUMBER_KNOWN_PREFIXES_MULTIPLIER: Multiplier applied to the number of known spam prefixes when calculating prefix spam score.
pub fn prefix_spam_number_known_prefixes_multiplier() {
    todo!();
}
/// PREFIX_SPAM_REQUIRED_SCORE: The required prefix spam score a prefix must have for it to be considered spam.
pub fn prefix_spam_required_score() {
    todo!();
}
/// PREFIX_SPAM_START_COLOR_MULTIPLIER: If a prefix begins with a color code it will have this multiplier applied to its score.
pub fn prefix_spam_start_color_multiplier() {
    todo!();
}
/// PREFIX_SPAM_TIMEOUT_MULTIPLIER: Multiplier applied to time calculation to determine how long a known prefix is remembered.
pub fn prefix_spam_timeout_multiplier() {
    todo!();
}
/// PROTECT_SENSITIVE_FILES: Try to protect user.cfg from read access by other users?
pub fn protect_sensitive_files() {
    todo!();
}
/// REAL_ARENA_SIZE_FACTOR: The currently active arena size. Leave it alone! Change size_factor instead.
pub fn real_arena_size_factor() {
    todo!();
}
/// REAL_CYCLE_SPEED_FACTOR: The currently active cycle speed multiplier. Leave it alone! Change speed_factor instead.
pub fn real_cycle_speed_factor() {
    todo!();
}
/// RECORDING_DEBUGLEVEL: Level of additional information in recording file.
pub fn recording_debuglevel() {
    todo!();
}
/// RECORD_TURTLE_MODE: Keep debug recording even in turtle mode. Normally, it is stopped to keep the server responsive. The begginning of an attack is recorded anyway, of course.
pub fn record_turtle_mode() {
    todo!();
}
/// REMOVE_HELP_TOPIC: Remove a help topic.
pub fn remove_help_topic() {
    todo!();
}
/// RENAME: Renames the given player.
pub fn rename() {
    todo!();
}
/// RESERVE_SCREEN_NAME: Reserves a screen name to a registered user
pub fn reserve_screen_name() {
    todo!();
}
/// RESOURCE_REPOSITORY_CLIENT: URI the client uses to search for map files if they aren't stored locally. Better leave it alone
pub fn resource_repository_client() {
    todo!();
}
/// RESOURCE_REPOSITORY_SERVER: URI clients and the server use to search for map files if they aren't stored locally
pub fn resource_repository_server() {
    todo!();
}
/// RIM_WALL_STRETCH_X: Extension of the rim wall texture in the horizontal direction
pub fn rim_wall_stretch_x() {
    todo!();
}
/// RIM_WALL_STRETCH_Y: Extension of the rim wall texture in the vertical direction
pub fn rim_wall_stretch_y() {
    todo!();
}
/// RIM_WALL_WRAP_Y: Set to 1 if the rim wall texture should repeat in the vertical direction
pub fn rim_wall_wrap_y() {
    todo!();
}
/// RINCLUDE: Includes a file using the resource system. Use the direct link syntax to your profit here.
pub fn rinclude() {
    todo!();
}
/// ROUND_CENTER_MESSAGE: Big message sent to clients after every round
pub fn round_center_message() {
    todo!();
}
/// ROUND_CONSOLE_MESSAGE: Message sent to clients after every round
pub fn round_console_message() {
    todo!();
}
/// RUBBER_GAUGE_LOCX: Horizontal position of the rubber meter
pub fn rubber_gauge_locx() {
    todo!();
}
/// RUBBER_GAUGE_LOCY: Vertical position of the rubber meter
pub fn rubber_gauge_locy() {
    todo!();
}
/// RUBBER_GAUGE_SIZE: Size of the rubber meter
pub fn rubber_gauge_size() {
    todo!();
}
/// SAVED_IN_VERSION: Used internally to remember which version the user configuration file was last saved it
pub fn saved_in_version() {
    todo!();
}
/// SAY: Dedicated server only: let the server administrator say something.
pub fn say(message: &str) {
    println!("{} {}", Command::Say, message);
}
/// SCORE_DEATHZONE: What you get for hitting the Death Zone
pub fn score_deathzone() {
    todo!();
}
/// SCORE_DIE: What you get for dying
pub fn score_die() {
    todo!();
}
/// SCORE_HOLE: What you get for making a hole for your teammates
pub fn score_hole() {
    todo!();
}
/// SCORE_KILL: What you get for killing someone
pub fn score_kill() {
    todo!();
}
/// SCORE_LOCX: Horizontal position of the score display
pub fn score_locx() {
    todo!();
}
/// SCORE_LOCY: Vertical position of the score display
pub fn score_locy() {
    todo!();
}
/// SCORE_SIZE: Size of the score display
pub fn score_size() {
    todo!();
}
/// SCORE_SUICIDE: What you get for stupidly dying
pub fn score_suicide() {
    todo!();
}
/// SCORE_SURVIVE: What you get for surviving
pub fn score_survive() {
    todo!();
}
/// SCORE_WIN: What you get for winning a round
pub fn score_win() {
    todo!();
}
/// SERVER_DNS: If your server is on dynamic IP and you set up a dynamic DNS that always points to it, you can set this variable to the DNS name to help clients remember your server across IP changes.
pub fn server_dns() {
    todo!();
}
/// SERVER_IP: IP the server listens on
pub fn server_ip() {
    todo!();
}
/// SERVER_NAME: Name of this server
pub fn server_name() {
    todo!();
}
/// SERVER_PORT: Port this server listens on
pub fn server_port() {
    todo!();
}
/// SETTING_LEGACY_BEHAVIOR_ANNOYING: Default legacy behavior for settings that only cause minor annoyances on old clients, like enemy cycles stopping for .1 seconds after each turn. Example: CYCLE_RUBBER_MINDISTANCE
pub fn setting_legacy_behavior_annoying() {
    todo!();
}
/// SETTING_LEGACY_BEHAVIOR_BREAKING: Default legacy behavior for settings that absolutely break the client and make play impossible. Example of an affected setting: MAP_FILE
pub fn setting_legacy_behavior_breaking() {
    todo!();
}
/// SETTING_LEGACY_BEHAVIOR_BUMPY: Default legacy behavior for settings that allow play on old clients in principle, but with severe limitations (cycles bouncing around, player commands not executed on time). Example: CYCLE_DELAY_TIMEBASED
pub fn setting_legacy_behavior_bumpy() {
    todo!();
}
/// SETTING_LEGACY_BEHAVIOR_CHEATING: Default legacy behavior for settings where the default behavior could be considered cheating if non-default was set. Example: DOUBLEBIND_TIME
pub fn setting_legacy_behavior_cheating() {
    todo!();
}
/// SETTING_LEGACY_BEHAVIOR_VISUAL: Default legacy behavior for settings that only affect status displays and visuals, not game physics. Example: CYCLE_RUBBER_* (the client displays the rubber meter, but it's not used for anything)
pub fn setting_legacy_behavior_visual() {
    todo!();
}
/// SHOW_ALIVE: Show the number of enemies and friends left on the HUD?
pub fn show_alive() {
    todo!();
}
/// SHOW_BRAKE: Show the brake meter on the HUD?
pub fn show_brake() {
    todo!();
}
/// SHOW_FASTEST: Show the fastest player on the HUD?
pub fn show_fastest() {
    todo!();
}
/// SHOW_FPS: Enable fps display
pub fn show_fps() {
    todo!();
}
/// SHOW_HUD: Show the HUD?
pub fn show_hud() {
    todo!();
}
/// SHOW_OWN_IP: Should your public IP be displayed when connecting to the master server?
pub fn show_own_ip() {
    todo!();
}
/// SHOW_OWN_NAME: Should your name be displayed above your cycle on your screen?
pub fn show_own_name() {
    todo!();
}
/// SHOW_PING: Show your ping on the HUD?
pub fn show_ping() {
    todo!();
}
/// SHOW_RECORDING_TIME: Enable display of recording/playback time
pub fn show_recording_time() {
    todo!();
}
/// SHOW_RUBBER: Show the rubber meter on the HUD?
pub fn show_rubber() {
    todo!();
}
/// SHOW_SCORE: Show your single player scores on the HUD?
pub fn show_score() {
    todo!();
}
/// SHOW_SPEED: Show the speed meter on the HUD?
pub fn show_speed() {
    todo!();
}
/// SHOW_TIME: Show the current time in the top- right corner?
pub fn show_time() {
    todo!();
}
/// SHOW_TIME_24: Show the time in 24 hour format?
pub fn show_time_24() {
    todo!();
}
/// SHUFFLE_SPAM_MESSAGES_PER_ROUND: Per round, per player limit on the number of shuffle messages displayed. A negative or zero value disables this check.
pub fn shuffle_spam_messages_per_round() {
    todo!();
}
/// SILENCE: Silence a specific player so they can't use public chat any more (/msg and /team still work)
pub fn silence() {
    todo!();
}
/// SILENCE_DEFAULT: If set to 1, new players will be silenced
pub fn silence_default() {
    todo!();
}
/// SIMPLE_TRAIL: If set to 1, trails are rendered simplified for better performance.
pub fn simple_trail() {
    todo!();
}
/// SINCLUDE: Includes the following file silently, without error message if it is not found
pub fn sinclude(path: &Path) {
    if let Some(path) = path.to_str() {
        println!("{} {:?}", Command::Sinclude, path);
    }
}
/// SIZE_FACTOR: Arena size modifier
pub fn size_factor(factor: i64) {
    println!("{} {}", Command::SizeFactor, factor);
}
/// SKY_WOBBLE: Sky animation
pub fn sky_wobble() {
    todo!();
}
/// SLAP: Slaps the given player by penalizing them with the specified amount of points. Be smart and you may be able to hug your users, too ;)
pub fn slap() {
    todo!();
}
/// SMART_GLANCE_CUSTOM_1: Use custom camera settings when glancing with the smart camera
pub fn smart_glance_custom_1() {
    todo!();
}
/// SMART_GLANCE_CUSTOM_2: Use custom camera settings when glancing with the smart camera
pub fn smart_glance_custom_2() {
    todo!();
}
/// SMART_GLANCE_CUSTOM_3: Use custom camera settings when glancing with the smart camera
pub fn smart_glance_custom_3() {
    todo!();
}
/// SMART_GLANCE_CUSTOM_4: Use custom camera settings when glancing with the smart camera
pub fn smart_glance_custom_4() {
    todo!();
}
/// SMOOTH_SHADING: Enable smooth shading
pub fn smooth_shading() {
    todo!();
}
/// SOFTWARE_RENDERER: Is the OpenGL renderer not hardware accelerated?
pub fn software_renderer() {
    todo!();
}
/// SOUND_BUFFER_SHIFT: Buffer size multiplier
pub fn sound_buffer_shift() {
    todo!();
}
/// SOUND_QUALITY: Sound quality [0=off, 3=high]
pub fn sound_quality() {
    todo!();
}
/// SOUND_SOURCES: Number of sound sources to be heard at the same time
pub fn sound_sources() {
    todo!();
}
/// SPAM_AUTOKICK: Spam score that causes you to get kicked instantly.
pub fn spam_autokick() {
    todo!();
}
/// SPAM_AUTOKICK_COUNT: Number of spam warnings before a player gets spamkicked.
pub fn spam_autokick_count() {
    todo!();
}
/// SPAM_MAXLEN: Maximal length of chat message.
pub fn spam_maxlen() {
    todo!();
}
/// SPAM_MAXLEN_OVERRIDE: Block out older clients when SPAM_MAXLEN differs from its default?
pub fn spam_maxlen_override() {
    todo!();
}
/// SPAM_PENALTY: Number of seconds to silence a spammer.
pub fn spam_penalty() {
    todo!();
}
/// SPAM_PROTECTION: Harshness of spam protection; determines min delay between chat messages accepted.
pub fn spam_protection() {
    todo!();
}
/// SPAM_PROTECTION_CHAT: Extra factor for SPAM_PROTECTION for chat messages.
pub fn spam_protection_chat() {
    todo!();
}
/// SPAM_PROTECTION_REPEAT: Minimum time between identical chat messages.
pub fn spam_protection_repeat() {
    todo!();
}
/// SPAM_PROTECTION_VOTE: Extra factor for SPAM_PROTECTION for votes.
pub fn spam_protection_vote() {
    todo!();
}
/// SPARKS: Draw sparks when going too close to a wall
pub fn sparks() {
    todo!();
}
/// SPAWN_WINGMEN_BACK: Determines how much each wingman is placed backwards in a team.
pub fn spawn_wingmen_back() {
    todo!();
}
/// SPAWN_WINGMEN_SIDE: Determines how much each wingman is placed sidewards in a team.
pub fn spawn_wingmen_side() {
    todo!();
}
/// SPECTATOR_MODE_1: Sets spectator mode for this player
pub fn spectator_mode_1() {
    todo!();
}
/// SPECTATOR_MODE_2: Sets spectator mode for this player
pub fn spectator_mode_2() {
    todo!();
}
/// SPECTATOR_MODE_3: Sets spectator mode for this player
pub fn spectator_mode_3() {
    todo!();
}
/// SPECTATOR_MODE_4: Sets spectator mode for this player
pub fn spectator_mode_4() {
    todo!();
}
/// SPEED_FACTOR: Speed modifier for the cycles
pub fn speed_factor() {
    todo!();
}
/// SPEED_GAUGE_LOCX: Horizontal position of the speed meter
pub fn speed_gauge_locx() {
    todo!();
}
/// SPEED_GAUGE_LOCY: Vertical position of the speed meter
pub fn speed_gauge_locy() {
    todo!();
}
/// SPEED_GAUGE_SIZE: Size of the speed meter
pub fn speed_gauge_size() {
    todo!();
}
/// SP_AI_IQ: IQ of the AI opponents in single player mode
pub fn sp_ai_iq() {
    todo!();
}
/// SP_AUTO_AIS: Automatically spawn AI players in single player mode?
pub fn sp_auto_ais() {
    todo!();
}
/// SP_AUTO_IQ: Automatically adjust AI IQ in single player mode?
pub fn sp_auto_iq() {
    todo!();
}
/// SP_EXPLOSION_RADIUS: Blast radius of the cycle explosions in single player mode
pub fn sp_explosion_radius() {
    todo!();
}
/// SP_FINISH_TYPE: What happens when the last human is dead in single player mode?
pub fn sp_finish_type() {
    todo!();
}
/// SP_GAME_TYPE: Type of game played in single player mode. 0 for freestyle, 1 for last team standing and 2 for humans vs. AIs.
pub fn sp_game_type() {
    todo!();
}
/// SP_LIMIT_ROUNDS: End the match after this number of rounds in single player mode
pub fn sp_limit_rounds() {
    todo!();
}
/// SP_LIMIT_SCORE: End the match when a player reaches this score in single player mode
pub fn sp_limit_score() {
    todo!();
}
/// SP_LIMIT_TIME: End the match after this number of minutes in single player mode
pub fn sp_limit_time() {
    todo!();
}
/// SP_MIN_PLAYERS: Minimum number of players in single player mode
pub fn sp_min_players() {
    todo!();
}
/// SP_NUM_AIS: Number of AI players in single player mode
pub fn sp_num_ais() {
    todo!();
}
/// SP_SCORE_WIN: What you get for winning a round in single player mode
pub fn sp_score_win() {
    todo!();
}
/// SP_SIZE_FACTOR: Arena size modifier
pub fn sp_size_factor(factor: i64) {
    println!("{} {}", Command::SpSizeFactor, factor);
}
/// SP_SPEED_FACTOR: Speed modifier for the cycles
pub fn sp_speed_factor() {
    todo!();
}
/// SP_TEAMS_MAX: Maximum number of teams in single player mode
pub fn sp_teams_max() {
    todo!();
}
/// SP_TEAMS_MIN: Minimum number of teams in single player mode
pub fn sp_teams_min() {
    todo!();
}
/// SP_TEAM_BALANCE_ON_QUIT: Balance teams on player quit in single player mode?
pub fn sp_team_balance_on_quit() {
    todo!();
}
/// SP_TEAM_BALANCE_WITH_AIS: Balance teams with AI players in single player mode?
pub fn sp_team_balance_with_ais() {
    todo!();
}
/// SP_TEAM_MAX_IMBALANCE: Maximum allowed team imbalance in single player mode
pub fn sp_team_max_imbalance() {
    todo!();
}
/// SP_TEAM_MAX_PLAYERS: Maximum number of players per team in single player mode
pub fn sp_team_max_players() {
    todo!();
}
/// SP_TEAM_MIN_PLAYERS: Minimum number of players per team in single player mode
pub fn sp_team_min_players() {
    todo!();
}
/// SP_WALLS_LENGTH: Length of the cycle walls in meters; negative values will make the walls infinite.
pub fn sp_walls_length() {
    todo!();
}
/// SP_WALLS_STAY_UP_DELAY: Number of seconds the walls stay up after a player died; negative values will keep them up forever.
pub fn sp_walls_stay_up_delay() {
    todo!();
}
/// SP_WIN_ZONE_MIN_LAST_DEATH: Minimum number of seconds since the last death before the instant win zone is activated in single player mode
pub fn sp_win_zone_min_last_death() {
    todo!();
}
/// SP_WIN_ZONE_MIN_ROUND_TIME: Minimum number of seconds the round has to be going on before the instant win zone is activated in single player mode
pub fn sp_win_zone_min_round_time() {
    todo!();
}
/// START_CAM_1: Initial Camera
pub fn start_cam_1() {
    todo!();
}
/// START_CAM_2: Initial Camera
pub fn start_cam_2() {
    todo!();
}
/// START_CAM_3: Initial Camera
pub fn start_cam_3() {
    todo!();
}
/// START_CAM_4: Initial Camera
pub fn start_cam_4() {
    todo!();
}
/// START_FOV_1: Initial field of vision
pub fn start_fov_1() {
    todo!();
}
/// START_FOV_2: Initial field of vision
pub fn start_fov_2() {
    todo!();
}
/// START_FOV_3: Initial field of vision
pub fn start_fov_3() {
    todo!();
}
/// START_FOV_4: Initial field of vision
pub fn start_fov_4() {
    todo!();
}
/// START_NEW_MATCH: Initiates a new match
pub fn start_new_match() {
    println!("{}", Command::StartNewMatch)
}
/// STOP_RECORDING: Stops a currently running recording to save resources. Resuming is impossible.
pub fn stop_recording() {
    todo!();
}
/// SUSPEND: Suspend a player from playing for the following N rounds (default is set by SUSPEND_DEFAULT_ROUNDS)
pub fn suspend() {
    todo!();
}
/// SUSPEND_DEFAULT_ROUNDS: Sets default round timeout for SUSPEND.
pub fn suspend_default_rounds() {
    todo!();
}
/// SWAP_MODE: Determines the commands used to sync graphics and input. 0: do nothing, 1: call glFlush(), 2: call glFinish().
pub fn swap_mode() {
    todo!();
}
/// SWITCH_VIEW_TOOLTIP: switch_view_tooltip_help
pub fn switch_view_tooltip() {
    todo!();
}
/// TALK_TO_MASTER: Announce this server on the Internet?
pub fn talk_to_master() {
    todo!();
}
/// TEAMS: Get a list of all teams with a somewhat graphic representation of their formation. Same as saying /teams
pub fn teams() {
    todo!();
}
/// TEAMS_MAX: Maximum number of teams
pub fn teams_max(value: u64) {
    println!("{} {}", Command::TeamsMax, value);
}
/// TEAMS_MIN: Minimum number of teams
pub fn teams_min(value: u64) {
    println!("{} {}", Command::TeamsMax, value);
}
/// TEAM_BALANCE_ON_QUIT: Balance teams on player quit?
pub fn team_balance_on_quit(value: bool) {
    println!("{} {}", Command::TeamBalanceOnQuit, value);
}
/// TEAM_BALANCE_WITH_AIS: Balance teams with AI players?
pub fn team_balance_with_ais(value: bool) {
    println!("{} {}", Command::TeamBalanceOnQuit, value);
}
/// TEAM_BLUE_1: blue portion of team 1's color
pub fn team_blue_1() {
    todo!();
}
/// TEAM_BLUE_2: blue portion of team 2's color
pub fn team_blue_2() {
    todo!();
}
/// TEAM_BLUE_3: blue portion of team 3's color
pub fn team_blue_3() {
    todo!();
}
/// TEAM_BLUE_4: blue portion of team 4's color
pub fn team_blue_4() {
    todo!();
}
/// TEAM_BLUE_5: blue portion of team 5's color
pub fn team_blue_5() {
    todo!();
}
/// TEAM_BLUE_6: blue portion of team 6's color
pub fn team_blue_6() {
    todo!();
}
/// TEAM_BLUE_7: blue portion of team 7's color
pub fn team_blue_7() {
    todo!();
}
/// TEAM_BLUE_8: blue portion of team 8's color
pub fn team_blue_8() {
    todo!();
}
/// TEAM_CENTER_IS_BOSS: If set to 1, the center player is the team's boss. If at 0, it's the player who is on that team longest.
pub fn team_center_is_boss() {
    todo!();
}
/// TEAM_ELIMINATION_MODE: Defines the way ArmagetronAd should eliminate teams when there's more teams than TEAMS_MAX: Set to 0 it will try to keep as many players as possible, kicking teams that have the lowest score if teams are balanced; Set to 1 it will try to keep the best team colors (Team blue, then Team gold, then Team red, etc); Set to 2 it will kick out the teams that have the lowest score, regardless of balance.
pub fn team_elimination_mode() {
    todo!();
}
/// TEAM_GREEN_1: green portion of team 1's color
pub fn team_green_1() {
    todo!();
}
/// TEAM_GREEN_2: green portion of team 2's color
pub fn team_green_2() {
    todo!();
}
/// TEAM_GREEN_3: green portion of team 3's color
pub fn team_green_3() {
    todo!();
}
/// TEAM_GREEN_4: green portion of team 4's color
pub fn team_green_4() {
    todo!();
}
/// TEAM_GREEN_5: green portion of team 5's color
pub fn team_green_5() {
    todo!();
}
/// TEAM_GREEN_6: green portion of team 6's color
pub fn team_green_6() {
    todo!();
}
/// TEAM_GREEN_7: green portion of team 7's color
pub fn team_green_7() {
    todo!();
}
/// TEAM_GREEN_8: green portion of team 8's color
pub fn team_green_8() {
    todo!();
}
/// TEAM_MAX_IMBALANCE: Maximum allowed team imbalance
pub fn team_max_imbalance(value: u64) {
    println!("{} {}", Command::TeamMaxImbalance, value);
}
/// TEAM_MAX_PLAYERS: Maximum number of players per team
pub fn team_max_players(value: u64) {
    println!("{} {}", Command::TeamMaxPlayers, value);
}
/// TEAM_MIN_PLAYERS: Minimum number of players per team
pub fn team_min_players(value: u64) {
    println!("{} {}", Command::TeamMinPlayers, value);
}
/// TEAM_NAME_1: name of team 1
pub fn team_name_1() {
    todo!();
}
/// TEAM_NAME_2: name of team 2
pub fn team_name_2() {
    todo!();
}
/// TEAM_NAME_3: name of team 3
pub fn team_name_3() {
    todo!();
}
/// TEAM_NAME_4: name of team 4
pub fn team_name_4() {
    todo!();
}
/// TEAM_NAME_5: name of team 5
pub fn team_name_5() {
    todo!();
}
/// TEAM_NAME_6: name of team 6
pub fn team_name_6() {
    todo!();
}
/// TEAM_NAME_7: name of team 7
pub fn team_name_7() {
    todo!();
}
/// TEAM_NAME_8: name of team 8
pub fn team_name_8() {
    todo!();
}
/// TEAM_RED_1: red portion of team 1's color
pub fn team_red_1() {
    todo!();
}
/// TEAM_RED_2: red portion of team 2's color
pub fn team_red_2() {
    todo!();
}
/// TEAM_RED_3: red portion of team 3's color
pub fn team_red_3() {
    todo!();
}
/// TEAM_RED_4: red portion of team 4's color
pub fn team_red_4() {
    todo!();
}
/// TEAM_RED_5: red portion of team 5's color
pub fn team_red_5() {
    todo!();
}
/// TEAM_RED_6: red portion of team 6's color
pub fn team_red_6() {
    todo!();
}
/// TEAM_RED_7: red portion of team 7's color
pub fn team_red_7() {
    todo!();
}
/// TEAM_RED_8: red portion of team 8's color
pub fn team_red_8() {
    todo!();
}
/// TEXTURES_HI: Use high color textures
pub fn textures_hi() {
    todo!();
}
/// TEXTURE_MODE_0: Floor Texture:
pub fn texture_mode_0() {
    todo!();
}
/// TEXTURE_MODE_1: Wall Textures:
pub fn texture_mode_1() {
    todo!();
}
/// TEXTURE_MODE_2: Object Textures:
pub fn texture_mode_2() {
    todo!();
}
/// TEXTURE_MODE_3: Font:
pub fn texture_mode_3() {
    todo!();
}
/// TEXT_OUT: Enable console text output
pub fn text_out() {
    todo!();
}
/// TIMEBOT_ACTION_HIGH: Action to take on a high suspicion of timebottery. 0: do nothing, 1: log it, 2: message moderators, 3: message all players, 4: kick the offending player.
pub fn timebot_action_high() {
    todo!();
}
/// TIMEBOT_ACTION_MAX: Action to take on a very high suspicion of timebottery. 0: do nothing, 1: log it, 2: message moderators, 3: message all players, 4: kick the offending player.
pub fn timebot_action_max() {
    todo!();
}
/// TIMEBOT_ACTION_MEDIUM: Action to take on a medium suspicion of timebottery. 0: do nothing, 1: log it, 2: message moderators, 3: message all players, 4: kick the offending player.
pub fn timebot_action_medium() {
    todo!();
}
/// TIMEBOT_KICK_SEVERITY: If players get kicked by the timebot detection, it's done with this severity level.
pub fn timebot_kick_severity() {
    todo!();
}
/// TIMEBOT_SENSITIVITY: The sensitivity of the timebot detection code. 1.0 is the default and you probably shouldn't deviate more than .5 from that.
pub fn timebot_sensitivity() {
    todo!();
}
/// TITLE_OF_DAY: If fullscreen display is supported, this will be the title above message_of_day
pub fn title_of_day() {
    todo!();
}
/// TOGGLE_SPECTATOR_TOOLTIP: toggle_spectator_tooltip_help
pub fn toggle_spectator_tooltip() {
    todo!();
}
/// TOPOLOGY_POLICE: The topology police does posterior checks to determine whether game moves were legal.
pub fn topology_police() {
    todo!();
}
/// TOPOLOGY_POLICE_PARALLEL: Extra topology police flag to check for walls that are put into the grid data-structure exactly parallel to each other. Requites TOPOLOGY_POLICE to be active.
pub fn topology_police_parallel() {
    todo!();
}
/// TRUST_LAN: If set to 1, the server assumes that your LAN is safe and that nobody can run a pharming server on it.
pub fn trust_lan() {
    todo!();
}
/// UNBAN_IP: Revokes the ban of the specified IP address.
pub fn unban_ip() {
    todo!();
}
/// UNBAN_USER: Undoes BAN_USER.
pub fn unban_user() {
    todo!();
}
/// UNLOCK_ALL_TEAMS: Unlocks all teams.
pub fn unlock_all_teams() {
    todo!();
}
/// UNSILENCE: Reverts a SILENCE command
pub fn unsilence() {
    todo!();
}
/// UNSUSPEND: Removes a player suspension.
pub fn unsuspend() {
    todo!();
}
/// UPPER_SKY: Draw upper sky plane
pub fn upper_sky() {
    todo!();
}
/// URL: HTTP URI associated with a server
pub fn url() {
    todo!();
}
/// USER_1: Global player ID
pub fn user_1() {
    todo!();
}
/// USER_2: Global player ID
pub fn user_2() {
    todo!();
}
/// USER_3: Global player ID
pub fn user_3() {
    todo!();
}
/// USER_4: Global player ID
pub fn user_4() {
    todo!();
}
/// USER_ALIAS: Allows bending authenticated names around: a player authenticated as X originally can appear as y.
pub fn user_alias() {
    todo!();
}
/// USER_LEVEL: Changes the access level of a user.
pub fn user_level() {
    todo!();
}
/// USER_REMOVE: Removes an password account for a user or team.
pub fn user_remove() {
    todo!();
}
/// USE_DISPLAYLISTS: Use display lists for rendering the cycles?
pub fn use_displaylists() {
    todo!();
}
/// VIEWPORT_CONF: Viewport configuration; decides how many players can play on this computer
pub fn viewport_conf() {
    todo!();
}
/// VIEWPORT_TO_PLAYER_1: Assign this viewport to a player
pub fn viewport_to_player_1() {
    todo!();
}
/// VIEWPORT_TO_PLAYER_2: Assign this viewport to a player
pub fn viewport_to_player_2() {
    todo!();
}
/// VIEWPORT_TO_PLAYER_3: Assign this viewport to a player
pub fn viewport_to_player_3() {
    todo!();
}
/// VIEWPORT_TO_PLAYER_4: Assign this viewport to a player
pub fn viewport_to_player_4() {
    todo!();
}
/// VOICE: Reverse of SILENCE
pub fn voice() {
    todo!();
}
/// VOTES_CANCEL: Cancels all running polls.
pub fn votes_cancel() {
    todo!();
}
/// VOTES_SUSPEND: Suspends voting for n minutes.
pub fn votes_suspend() {
    todo!();
}
/// VOTES_SUSPEND_DEFAULT: Default value for VOTES_SUSPEND.
pub fn votes_suspend_default() {
    todo!();
}
/// VOTES_UNSUSPEND: Allows voting again.
pub fn votes_unsuspend() {
    todo!();
}
/// VOTE_KICK_REASON: Default reason given to players when they're vote-kicked.
pub fn vote_kick_reason() {
    todo!();
}
/// VOTE_KICK_TO_PORT: Default server port a player is redirected to by vote kicks.
pub fn vote_kick_to_port() {
    todo!();
}
/// VOTE_KICK_TO_SERVER: Server IP/name a player is redirected to by vote kicks.
pub fn vote_kick_to_server() {
    todo!();
}
/// VOTE_USE_SERVER_CONTROLLED_KICK: Set to 1 to use the enhanced server controlled vote items for kick votes. Does not work for clients prior to 0.2.8.0_rc1.
pub fn vote_use_server_controlled_kick() {
    todo!();
}
/// VOTING_BIAS: Add virtual voters that oppose every change.
pub fn voting_bias() {
    todo!();
}
/// VOTING_BIAS_COMMAND: Add virtual voters that oppose every command vote.
pub fn voting_bias_command() {
    todo!();
}
/// VOTING_BIAS_INCLUDE: Add virtual voters that oppose every include vote.
pub fn voting_bias_include() {
    todo!();
}
/// VOTING_BIAS_KICK: Add virtual voters that oppose every kick vote.
pub fn voting_bias_kick() {
    todo!();
}
/// VOTING_BIAS_SILENCE: Add virtual voters that oppose every silence vote.
pub fn voting_bias_silence() {
    todo!();
}
/// VOTING_BIAS_SUSPEND: Add virtual voters that oppose every suspend vote.
pub fn voting_bias_suspend() {
    todo!();
}
/// VOTING_BIAS_VOICE: Add virtual voters that oppose every voice vote.
pub fn voting_bias_voice() {
    todo!();
}
/// VOTING_DECAY: One non-voter is ignored every time this many seconds pass.
pub fn voting_decay() {
    todo!();
}
/// VOTING_HARM_TIME: The minimum time in seconds between two harmful votes against the same player.
pub fn voting_harm_time() {
    todo!();
}
/// VOTING_KICK_MINHARM: Minimal number of harmful votes (suspension, kick,..) that need to have been issued (success is not required) against a player before a kick vote issued via the menu really results in a kick; otherwise, the result is a simple suspension.
pub fn voting_kick_minharm() {
    todo!();
}
/// VOTING_KICK_TIME: The minimum time in seconds between two kick votes against the same player.
pub fn voting_kick_time() {
    todo!();
}
/// VOTING_MATURITY: The minimum time in seconds a player needs to be online with the same name before they can issue votes.
pub fn voting_maturity() {
    todo!();
}
/// VOTING_PRIVACY: Controls logging of voting process. 2: nothing gets logged 1: vote submission is logged for the server admin 0: voting is logged for the server admin -1: vote submission is made public -2: everything is made public
pub fn voting_privacy() {
    todo!();
}
/// VOTING_SPAM_ISSUE: The spam level of issuing a vote.
pub fn voting_spam_issue() {
    todo!();
}
/// VOTING_SPAM_REJECT: The spam level of getting your vote rejected.
pub fn voting_spam_reject() {
    todo!();
}
/// VOTING_START_DECAY: Number of seconds after that the non-voters start to get ignored.
pub fn voting_start_decay() {
    todo!();
}
/// VOTING_SUSPEND_ROUNDS: The number of rounds "/vote suspend <player>" suspends a player for.
pub fn voting_suspend_rounds() {
    todo!();
}
/// VOTING_TIMEOUT: Votes older than this time out and are rejected.
pub fn voting_timeout() {
    todo!();
}
/// VOTING_TIMEOUT_PER_VOTER: Additional value for VOTING_TIMEOUT for every voter present.
pub fn voting_timeout_per_voter() {
    todo!();
}
/// WAIT_FOR_EXTERNAL_SCRIPT: Let the server wait for an external script between two rounds until the script switches this setting back to 0.
pub fn wait_for_external_script(value: bool) {
    println!("{} {}", Command::WaitForExternalScript, value.byte());
}
/// WAIT_FOR_EXTERNAL_SCRIPT_TIMEOUT: If the server has been paused by WAIT_FOR_EXTERNAL_SCRIPT for more seconds than this, kickstart the game.
pub fn wait_for_external_script_timeout(seconds: u64) {
    println!("{} {}", Command::WaitForExternalScriptTimeout, seconds)
}
/// WALLS_LENGTH: Length of the cycle walls in meters; negative values will make the walls infinite.
pub fn walls_length() {
    todo!();
}
/// WALLS_STAY_UP_DELAY: Number of seconds the walls stay up after a player died; negative values will keep them up forever.
pub fn walls_stay_up_delay() {
    todo!();
}
/// WHITELIST_ENEMIES_IP: Allow any players from the specified IP address to be enemies. Usage: WHITELIST_ENEMIES_IP <ip1> ...
pub fn whitelist_enemies_ip() {
    todo!();
}
/// WHITELIST_ENEMIES_USERNAME: Allow players from the same IP address to be enemies if one of them is authenticated, and in this list. WHITELIST_ENEMIES_USERNAME <authenticated name1> <authenticated name2> ...
pub fn whitelist_enemies_username() {
    todo!();
}
/// WHITE_SPARKS: Draw sparks in white (instead of cycle colors).
pub fn white_sparks() {
    todo!();
}
/// WIN_ZONE_DEATHS: A value of 1 turns it into a death zone.
pub fn win_zone_deaths() {
    todo!();
}
/// WIN_ZONE_EXPANSION: Expansion speed of the instant win zone
pub fn win_zone_expansion() {
    todo!();
}
/// WIN_ZONE_INITIAL_SIZE: Initial size of the instant win zone
pub fn win_zone_initial_size() {
    todo!();
}
/// WIN_ZONE_MIN_LAST_DEATH: Minimum number of seconds since the last death before the instant win zone is activated
pub fn win_zone_min_last_death() {
    todo!();
}
/// WIN_ZONE_MIN_ROUND_TIME: Minimum number of seconds the round has to be going on before the instant win zone is activated
pub fn win_zone_min_round_time() {
    todo!();
}
/// WIN_ZONE_RANDOMNESS: Randomness factor of the initial win zone position. 0 fixes it at the arena center, 1 spreads the zone all over it.
pub fn win_zone_randomness() {
    todo!();
}
/// WORD_DELIMITERS: Characters that count as word delimiters when skipping over words in a text input field.
pub fn word_delimiters() {
    todo!();
}
/// WRAP_MENU: If set, you leave a menu to the top and reenter it at the bottom.
pub fn wrap_menu() {
    todo!();
}
/// ZDEPTH: z buffer depth to use (0: 16 1: from color depth 2: 32)
pub fn zdepth() {
    todo!();
}
/// ZONE_ALPHA: Transparency factor for zone rendering. 1.0 gives full strength.
pub fn zone_alpha() {
    todo!();
}
/// ZONE_ALPHA_SERVER: Transparency factor for zone rendering, controlled by the server. 1.0 gives full strength.
pub fn zone_alpha_server() {
    todo!();
}
/// ZONE_ALPHA_TOGGLE: This is XORd with ALPHA_BLEND to determine the way to draw zones
pub fn zone_alpha_toggle() {
    todo!();
}
/// ZONE_BOTTOM: Where to put the zone along the Z axis. Default is 0.0
pub fn zone_bottom() {
    todo!();
}
/// ZONE_HEIGHT: The zone segments' height. Default is 5.0
pub fn zone_height() {
    todo!();
}
/// ZONE_SEGMENTS: How many segments the zone is formed with. Default is 11
pub fn zone_segments() {
    todo!();
}
/// ZONE_SEG_LENGTH: The rendered fraction of every segment. Default is .5
pub fn zone_seg_length() {
    todo!();
}
