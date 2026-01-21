use std::{net::IpAddr, net::SocketAddr, path::Path};

pub mod prelude;

mod extension;
use extension::*;

pub mod extra;

pub mod model;
pub use crate::model::{command::*, ladderlog::*, *};

pub mod runtime;

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
/// ALIVE_SIZE: Size of the alive headcount display
pub fn alive_size(size: f32) {
    println!("{} {}", Command::AliveSize, size)
}
/// ALLOW_CAM_1_0: Allow/forbid the different camera modes
pub fn allow_cam_1_0(allowed: bool) {
    println!("{} {}", Command::AllowCam1_0, allowed.byte());
}
/// ALLOW_CAM_1_1: Allow/forbid the different camera modes
pub fn allow_cam_1_1(allowed: bool) {
    println!("{} {}", Command::AllowCam1_1, allowed.byte());
}
/// ALLOW_CAM_1_2: Allow/forbid the different camera modes
pub fn allow_cam_1_2(allowed: bool) {
    println!("{} {}", Command::AllowCam1_2, allowed.byte());
}
/// ALLOW_CAM_1_3: Allow/forbid the different camera modes
pub fn allow_cam_1_3(allowed: bool) {
    println!("{} {}", Command::AllowCam1_3, allowed.byte());
}
/// ALLOW_CAM_1_4: Allow/forbid the different camera modes
pub fn allow_cam_1_4(allowed: bool) {
    println!("{} {}", Command::AllowCam1_4, allowed.byte());
}
/// ALLOW_CAM_1_5: Allow/forbid the different camera modes
pub fn allow_cam_1_5(allowed: bool) {
    println!("{} {}", Command::AllowCam1_5, allowed.byte());
}
/// ALLOW_CAM_1_6: Allow/forbid the different camera modes
pub fn allow_cam_1_6(allowed: bool) {
    println!("{} {}", Command::AllowCam1_6, allowed.byte());
}
/// ALLOW_CAM_2_0: Allow/forbid the different camera modes
pub fn allow_cam_2_0(allowed: bool) {
    println!("{} {}", Command::AllowCam2_0, allowed.byte());
}
/// ALLOW_CAM_2_1: Allow/forbid the different camera modes
pub fn allow_cam_2_1(allowed: bool) {
    println!("{} {}", Command::AllowCam2_1, allowed.byte());
}
/// ALLOW_CAM_2_2: Allow/forbid the different camera modes
pub fn allow_cam_2_2(allowed: bool) {
    println!("{} {}", Command::AllowCam2_2, allowed.byte());
}
/// ALLOW_CAM_2_3: Allow/forbid the different camera modes
pub fn allow_cam_2_3(allowed: bool) {
    println!("{} {}", Command::AllowCam2_3, allowed.byte());
}
/// ALLOW_CAM_2_4: Allow/forbid the different camera modes
pub fn allow_cam_2_4(allowed: bool) {
    println!("{} {}", Command::AllowCam2_4, allowed.byte());
}
/// ALLOW_CAM_2_5: Allow/forbid the different camera modes
pub fn allow_cam_2_5(allowed: bool) {
    println!("{} {}", Command::AllowCam2_5, allowed.byte());
}
/// ALLOW_CAM_2_6: Allow/forbid the different camera modes
pub fn allow_cam_2_6(allowed: bool) {
    println!("{} {}", Command::AllowCam2_6, allowed.byte());
}
/// ALLOW_CAM_3_0: Allow/forbid the different camera modes
pub fn allow_cam_3_0(allowed: bool) {
    println!("{} {}", Command::AllowCam3_0, allowed.byte());
}
/// ALLOW_CAM_3_1: Allow/forbid the different camera modes
pub fn allow_cam_3_1(allowed: bool) {
    println!("{} {}", Command::AllowCam3_1, allowed.byte());
}
/// ALLOW_CAM_3_2: Allow/forbid the different camera modes
pub fn allow_cam_3_2(allowed: bool) {
    println!("{} {}", Command::AllowCam3_2, allowed.byte());
}
/// ALLOW_CAM_3_3: Allow/forbid the different camera modes
pub fn allow_cam_3_3(allowed: bool) {
    println!("{} {}", Command::AllowCam3_3, allowed.byte());
}
/// ALLOW_CAM_3_4: Allow/forbid the different camera modes
pub fn allow_cam_3_4(allowed: bool) {
    println!("{} {}", Command::AllowCam3_4, allowed.byte());
}
/// ALLOW_CAM_3_5: Allow/forbid the different camera modes
pub fn allow_cam_3_5(allowed: bool) {
    println!("{} {}", Command::AllowCam3_5, allowed.byte());
}
/// ALLOW_CAM_3_6: Allow/forbid the different camera modes
pub fn allow_cam_3_6(allowed: bool) {
    println!("{} {}", Command::AllowCam3_6, allowed.byte());
}
/// ALLOW_CAM_4_0: Allow/forbid the different camera modes
pub fn allow_cam_4_0(allowed: bool) {
    println!("{} {}", Command::AllowCam4_0, allowed.byte());
}
/// ALLOW_CAM_4_1: Allow/forbid the different camera modes
pub fn allow_cam_4_1(allowed: bool) {
    println!("{} {}", Command::AllowCam4_1, allowed.byte());
}
/// ALLOW_CAM_4_2: Allow/forbid the different camera modes
pub fn allow_cam_4_2(allowed: bool) {
    println!("{} {}", Command::AllowCam4_2, allowed.byte());
}
/// ALLOW_CAM_4_3: Allow/forbid the different camera modes
pub fn allow_cam_4_3(allowed: bool) {
    println!("{} {}", Command::AllowCam4_3, allowed.byte());
}
/// ALLOW_CAM_4_4: Allow/forbid the different camera modes
pub fn allow_cam_4_4(allowed: bool) {
    println!("{} {}", Command::AllowCam4_4, allowed.byte());
}
/// ALLOW_CAM_4_5: Allow/forbid the different camera modes
pub fn allow_cam_4_5(allowed: bool) {
    println!("{} {}", Command::AllowCam4_5, allowed.byte());
}
/// ALLOW_CAM_4_6: Allow/forbid the different camera modes
pub fn allow_cam_4_6(allowed: bool) {
    println!("{} {}", Command::AllowCam4_6, allowed.byte());
}
/// ALLOW_CONTROL_DURING_CHAT: If set to 1, this allows a player to issue cycle and camera control commands during chat (losing the chatbot and the yellow chat pyramid).
pub fn allow_control_during_chat(allowed: bool) {
    println!("{} {}", Command::AllowControlDuringChat, allowed.byte());
}
/// ALLOW_ENEMIES_SAME_CLIENT: If set to 1, this allows two players that play on the same client to fight for points with each other.
pub fn allow_enemies_same_client(allowed: bool) {
    println!("{} {}", Command::AllowEnemiesSameClient, allowed.byte());
}
/// ALLOW_ENEMIES_SAME_IP: If set to 1, this allows two players that apparently come from the same machine to fight for points with each other.
pub fn allow_enemies_same_ip(allowed: bool) {
    println!("{} {}", Command::AllowEnemiesSameIp, allowed.byte());
}
/// ALLOW_IMPOSTERS: If set to 1, players with identical names are tolerated. If set to 0, all but one will be renamed.
pub fn allow_imposters(allowed: bool) {
    println!("{} {}", Command::AllowImposters, allowed.byte());
}
/// ALLOW_RENAME_PLAYER: Gives the given player the ability to rename.
pub fn allow_rename_player(player: &Player, allowed: bool) {
    println!(
        "{} {} {}",
        Command::AllowRenamePlayer,
        player,
        allowed.byte()
    );
}
/// ALLOW_TEAM_CHANGE: If set to 1, all players can change teams. If set to 0, players can only change teams if they've been specifically allowed to by ALLOW_TEAM_CHANGE_PLAYER
pub fn allow_team_change(allowed: bool) {
    println!("{} {}", Command::AllowTeamChange, allowed.byte());
}
/// ALLOW_TEAM_CHANGE_PLAYER: Allow a specific player to change teams even if ALLOW_TEAM_CHANGE is disabled
pub fn allow_team_change_player(player: &Player, allowed: bool) {
    println!(
        "{} {} {}",
        Command::AllowTeamChangePlayer,
        player,
        allowed.byte()
    );
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
    println!("{} {}", Command::AllowVoting, allowed.byte());
}
/// ALLOW_VOTING_SPECTATOR: If set to 1, voting will be allowed for spectators.
pub fn allow_voting_spectator(allowed: bool) {
    println!("{} {}", Command::AllowVotingSpectator, allowed.byte());
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
pub fn auto_iq(enabled: bool) {
    println!("{} {}", Command::AutoIq, enabled.byte());
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
pub fn player_message(player: &Player, message: &str) {
    println!("{} {} \"{}\"", Command::PlayerMessage, player, message);
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
/// ACCELZONE_RATE               accelzone_rate_help
#[cfg(feature = "ap")]
pub fn accelzone_rate() {
    todo!();
}
/// ACCESS_LEVEL_ADMIN           Minimal access level for /admin command.
#[cfg(feature = "ap")]
pub fn access_level_admin() {
    todo!();
}
/// ACCESS_LEVEL_OP              Minimal access level for /op and /deop co-admin management commands commands.
#[cfg(feature = "ap")]
pub fn access_level_op() {
    todo!();
}
/// ACCESS_LEVEL_OP_MAX          Maximal access level directly attainable by /op commands.
#[cfg(feature = "ap")]
pub fn access_level_op_max() {
    todo!();
}
/// ACCESS_LEVEL_OP_MIN          Minimal access level directly attainable by /op commands.
#[cfg(feature = "ap")]
pub fn access_level_op_min() {
    todo!();
}
/// ACCESS_LEVEL_QUEUE_CONFIGS   Sets the access level required to use chat commands /cq add & remove.
#[cfg(feature = "ap")]
pub fn access_level_queue_configs() {
    todo!();
}
/// ACCESS_LEVEL_QUEUE_MAPS      Sets the access level required to use chat commands /mq add & remove.
#[cfg(feature = "ap")]
pub fn access_level_queue_maps() {
    todo!();
}
/// ACCESS_LEVEL_REPORTS_CLEAR   access_level_reports_clear_help
#[cfg(feature = "ap")]
pub fn access_level_reports_clear() {
    todo!();
}
/// ACCESS_LEVEL_REPORTS_READ    access_level_reports_read_help
#[cfg(feature = "ap")]
pub fn access_level_reports_read() {
    todo!();
}
/// ACCESS_LEVEL_SPY_CONSOLE     Minimal access level you need for seeing console input from other (in-game) admins.
#[cfg(feature = "ap")]
pub fn access_level_spy_console() {
    todo!();
}
/// ACCESS_LEVEL_SUBSTITUTE      Required access level to switch with another player.
#[cfg(feature = "ap")]
pub fn access_level_substitute() {
    todo!();
}
/// ACCESS_LEVEL_TEAM            Minimal access level for /lock, /unlock, /invite and /uninvite team management.
#[cfg(feature = "ap")]
pub fn access_level_team() {
    todo!();
}
/// ACCESS_LEVEL_VIEW_CHATS      Players with access level equal to or lower than this are able to see messages sent from the same access leveled player.
#[cfg(feature = "ap")]
pub fn access_level_view_chats() {
    todo!();
}
/// ADD_SCORE_PLAYER             Give/Take points for that player. Usage: ADD_SCORE_PLAYER [name] [points] [message].
#[cfg(feature = "ap")]
pub fn add_score_player() {
    todo!();
}
/// ADD_SCORE_TEAM               Give/Take points for that team. Usage: ADD_SCORE_PLAYER [name] [points] [message].
#[cfg(feature = "ap")]
pub fn add_score_team() {
    todo!();
}
/// ADD_ZONE_ID_ROUTE            add to a zones route. Usage: ADD_ZONE_ROUTE <name> <x1> <y1> [<x2> <y2> ...]
#[cfg(feature = "ap")]
pub fn add_zone_id_route() {
    todo!();
}
/// ADD_ZONE_ROUTE               add to a zones route. Usage: ADD_ZONE_ROUTE <name> <x1> <y1> [<x2> <y2> ...]
#[cfg(feature = "ap")]
pub fn add_zone_route() {
    todo!();
}
/// ADMIN_KILL_MESSAGE           If set to 1, announce when players get killed due to the command "KILL"
#[cfg(feature = "ap")]
pub fn admin_kill_message() {
    todo!();
}
/// ADMIN_LOG                    Write all admin chat commands to var/adminlog.txt (Works only in a server)
#[cfg(feature = "ap")]
pub fn admin_log() {
    todo!();
}
/// ADMIN_NAME                   The name to speak as when using the command "SAY".
#[cfg(feature = "ap")]
pub fn admin_name() {
    todo!();
}
/// AI_BYPASS                    ai_bypass_help
#[cfg(feature = "ap")]
pub fn ai_bypass() {
    todo!();
}
/// AI_RELOAD                    ai_reload_help
#[cfg(feature = "ap")]
pub fn ai_reload() {
    todo!();
}
/// ALLOW_TEAM_NAME_LEADER       Allow team leader to set a team.
#[cfg(feature = "ap")]
pub fn allow_team_name_leader() {
    todo!();
}
/// ANNOUNCE                     Use like a public announcement. Displays Announcement: [message]
#[cfg(feature = "ap")]
pub fn announce() {
    todo!();
}
/// APPLY_ROTATION               Applies current round's map or next map, depending on ROTATION_TYPE.
#[cfg(feature = "ap")]
pub fn apply_rotation() {
    todo!();
}
/// APPLY_SPEC_FORCE             apply_spec_force_help
#[cfg(feature = "ap")]
pub fn apply_spec_force() {
    todo!();
}
/// APPLY_TEAM_FORCE             apply_team_force_help
#[cfg(feature = "ap")]
pub fn apply_team_force() {
    todo!();
}
/// ARENA_BOUNDARY               This is the distance players can travel safely outside the arena boundary.
#[cfg(feature = "ap")]
pub fn arena_boundary() {
    todo!();
}
/// ARENA_BOUNDARY_KILL          Determines what action to take when players are beyond the ARENA_BOUNDARY. 1: Kills players instantly. 2: Depletes their rubber until they're dead.
#[cfg(feature = "ap")]
pub fn arena_boundary_kill() {
    todo!();
}
/// AUTO_SUBSTITUTION            If set to 1, players will be substituted when leaving.
#[cfg(feature = "ap")]
pub fn auto_substitution() {
    todo!();
}
/// BALLS_INTERACT               Flag indicating whether balls can bounce off one another
#[cfg(feature = "ap")]
pub fn balls_interact() {
    todo!();
}
/// BALL_AUTORESPAWN             Flag indicating whether balls should automatically respawn when goal is scored
#[cfg(feature = "ap")]
pub fn ball_autorespawn() {
    todo!();
}
/// BALL_CYCLE_ACCEL_BOOST       Boost Cycle gives the ball when colliding
#[cfg(feature = "ap")]
pub fn ball_cycle_accel_boost() {
    todo!();
}
/// BALL_KILLS                   Flag indicating if a team owned ball can kill opposing team players
#[cfg(feature = "ap")]
pub fn ball_kills() {
    todo!();
}
/// BALL_SPEED_DECAY             Rate at which the ball slows down
#[cfg(feature = "ap")]
pub fn ball_speed_decay() {
    todo!();
}
/// BALL_SPEED_HIT_DECAY         Amount the ball slows down when hitting an object
#[cfg(feature = "ap")]
pub fn ball_speed_hit_decay() {
    todo!();
}
/// BALL_TEAM_MODE               Flag 0=ball score other team, 1=ball score only team owner
#[cfg(feature = "ap")]
pub fn ball_team_mode() {
    todo!();
}
/// BANNED_WORDS                 The list of words banned for various reasons.
#[cfg(feature = "ap")]
pub fn banned_words() {
    todo!();
}
/// BANNED_WORDS_ADD             Add a word to the banned words list.
#[cfg(feature = "ap")]
pub fn banned_words_add() {
    todo!();
}
/// BANNED_WORDS_DELIMITERS      The delimiters to remove from in the messages in case people encased banned words in them.
#[cfg(feature = "ap")]
pub fn banned_words_delimiters() {
    todo!();
}
/// BANNED_WORDS_LIST            Display the list of words currently banned.
#[cfg(feature = "ap")]
pub fn banned_words_list() {
    todo!();
}
/// BANNED_WORDS_OPTIONS         0: disable. 1: Block and alert message to sender. 2: Replace bad word with chosen character(s).
#[cfg(feature = "ap")]
pub fn banned_words_options() {
    todo!();
}
/// BANNED_WORDS_REMOVE          Remove a word from the banned words list.
#[cfg(feature = "ap")]
pub fn banned_words_remove() {
    todo!();
}
/// BANNED_WORDS_WHOLE           0: Shorten bad words to first and last letters. 1: All bad words get censored.
#[cfg(feature = "ap")]
pub fn banned_words_whole() {
    todo!();
}
/// BASE_ENEMY_KILL              Flag indicating whether a base will kill enemy players
#[cfg(feature = "ap")]
pub fn base_enemy_kill() {
    todo!();
}
/// BASE_ENEMY_RESPAWN           Flag indicating whether a base will respawn team if an enemy player enters it
#[cfg(feature = "ap")]
pub fn base_enemy_respawn() {
    todo!();
}
/// BASE_RESPAWN                 Flag indicating whether a base will respawn team if a team player enters it
#[cfg(feature = "ap")]
pub fn base_respawn() {
    todo!();
}
/// BASE_RESPAWN_REMIND_TIME     Time between respawn reminders
#[cfg(feature = "ap")]
pub fn base_respawn_remind_time() {
    todo!();
}
/// BOOT                         Kicks the specified player from the server.
#[cfg(feature = "ap")]
pub fn boot() {
    todo!();
}
/// CAMERA_FORBID_MER            Forbids the use of Meriton's camera
#[cfg(feature = "ap")]
pub fn camera_forbid_mer() {
    todo!();
}
/// CAMERA_GLANCE_FORWARD_SNAP   camera_glance_forward_snap_help
#[cfg(feature = "ap")]
pub fn camera_glance_forward_snap() {
    todo!();
}
/// CAMERA_GLANCE_HOLD           Hold the glance even after turning in a different direction, like 0.4 does.
#[cfg(feature = "ap")]
pub fn camera_glance_hold() {
    todo!();
}
/// CAMERA_GLANCE_SNAP           Essentially ups the turn speed while glancing. Helpful for immediately glancing in a direction after turning.
#[cfg(feature = "ap")]
pub fn camera_glance_snap() {
    todo!();
}
/// CENTER_PLAYER_MESSAGE        Sends a center message to a specified player.
#[cfg(feature = "ap")]
pub fn center_player_message() {
    todo!();
}
/// CFG_USER_SAVE                Can the user.cfg be saved after work?
#[cfg(feature = "ap")]
pub fn cfg_user_save() {
    todo!();
}
/// CHATBOT_CONTROLLED_BY_SERVER If enabled, the server controls the chatbot and ignores client input when chatting
#[cfg(feature = "ap")]
pub fn chatbot_controlled_by_server() {
    todo!();
}
/// CHATBOT_ENABLED              chatbot_enabled_help
#[cfg(feature = "ap")]
pub fn chatbot_enabled() {
    todo!();
}
/// CHATLOG_WRITE_TEAM           Write /team messages to chatlog [1: on | 0:off]
#[cfg(feature = "ap")]
pub fn chatlog_write_team() {
    todo!();
}
/// CHATTERS_KILL                All players in chat mode are killed by an administrator.
#[cfg(feature = "ap")]
pub fn chatters_kill() {
    todo!();
}
/// CHATTERS_LIST                All players in chat mode are listed.
#[cfg(feature = "ap")]
pub fn chatters_list() {
    todo!();
}
/// CHATTERS_SILENCE             All players in chat mode are silenced by an administrator.
#[cfg(feature = "ap")]
pub fn chatters_silence() {
    todo!();
}
/// CHATTERS_SUSPEND             All players in chat mode are suspended for this-many rounds by an administrator.
#[cfg(feature = "ap")]
pub fn chatters_suspend() {
    todo!();
}
/// CHAT_LOG_COLORS              Writes chat messages to var/chatlog_colors.txt
#[cfg(feature = "ap")]
pub fn chat_log_colors() {
    todo!();
}
/// CLEAR_CHATLOG                Clear all data from chatlog.txt located in ./var folder.
#[cfg(feature = "ap")]
pub fn clear_chatlog() {
    todo!();
}
/// CLEAR_LADDERLOG              Clear all data from ladderlog.txt located in ./var folder.
#[cfg(feature = "ap")]
pub fn clear_ladderlog() {
    todo!();
}
/// CLEAR_REPORTS                Clear all data from reports.txt located in ./var folder.
#[cfg(feature = "ap")]
pub fn clear_reports() {
    todo!();
}
/// CLEAR_SCORELOG               Clear all data from scorelog.txt located in ./var folder.
#[cfg(feature = "ap")]
pub fn clear_scorelog() {
    todo!();
}
/// CLIENT_DOWNLOAD_SETTINGS     If set to 1, clients with the supported feature can download the server settings.
#[cfg(feature = "ap")]
pub fn client_download_settings() {
    todo!();
}
/// COLLAPSE_ALL                 Causes all zones to vanish smoothly.
#[cfg(feature = "ap")]
pub fn collapse_all() {
    todo!();
}
/// COLLAPSE_ZONE                collapse a zone by name (or, if no arguments, collapses all zones without assigned names)
#[cfg(feature = "ap")]
pub fn collapse_zone() {
    todo!();
}
/// COLLAPSE_ZONE_ID             Collapse the zone by the given ID.
#[cfg(feature = "ap")]
pub fn collapse_zone_id() {
    todo!();
}
/// COLOR_DEATHZONE_BLUE         Default: 0, blue portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_deathzone_blue() {
    todo!();
}
/// COLOR_DEATHZONE_GREEN        Default: 0, green portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_deathzone_green() {
    todo!();
}
/// COLOR_DEATHZONE_RED          Default: 15, red portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_deathzone_red() {
    todo!();
}
/// COLOR_RUBBERZONE_BLUE        Blue portion of the color for rubber zone from 0 to 15.
#[cfg(feature = "ap")]
pub fn color_rubberzone_blue() {
    todo!();
}
/// COLOR_RUBBERZONE_GREEN       Green portion of the color for rubber zone from 0 to 15.
#[cfg(feature = "ap")]
pub fn color_rubberzone_green() {
    todo!();
}
/// COLOR_RUBBERZONE_RED         Red portion of the color for rubber zone from 0 to 15.
#[cfg(feature = "ap")]
pub fn color_rubberzone_red() {
    todo!();
}
/// COLOR_TELEPORTZONE_BLUE      Default: 0, blue portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_teleportzone_blue() {
    todo!();
}
/// COLOR_TELEPORTZONE_GREEN     Default: 15, green portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_teleportzone_green() {
    todo!();
}
/// COLOR_TELEPORTZONE_RED       Default: 0, red portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_teleportzone_red() {
    todo!();
}
/// COLOR_WINZONE_BLUE           Default: 0, blue portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_winzone_blue() {
    todo!();
}
/// COLOR_WINZONE_GREEN          Default: 15, green portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_winzone_green() {
    todo!();
}
/// COLOR_WINZONE_RED            Default: 0, red portion of the zone's color
#[cfg(feature = "ap")]
pub fn color_winzone_red() {
    todo!();
}
/// CONDENSE_CONQUEST_OUTPUT     Condense fort zone conquered output into one line for multiple wiiners.
#[cfg(feature = "ap")]
pub fn condense_conquest_output() {
    todo!();
}
/// CONFIG_ROTATION              A list of config files to rotate through, with values separated by semicolons. Optionally you can enter in the round like this: config|round_number;
#[cfg(feature = "ap")]
pub fn config_rotation() {
    todo!();
}
/// CONFIG_ROTATION_ADD          Add a config item to the CONFIG_ROTATION list of items. Optionally you can also add in the round of selection. Usage: CONFIG_ROTATION_ADD <config>{|round_number}
#[cfg(feature = "ap")]
pub fn config_rotation_add() {
    todo!();
}
/// CONFIG_ROTATION_LOAD         Loads the selected config from it's designated id from the list of CONFIG_ROTATION items. Usage: CONFIG_ROTATION_LOAD <config_id>
#[cfg(feature = "ap")]
pub fn config_rotation_load() {
    todo!();
}
/// CONFIG_ROTATION_REMOVE       Removed the selected config from the list of CONFIG_ROTATION items. Usage: CONFIG_ROTATION_REMOVE <config>
#[cfg(feature = "ap")]
pub fn config_rotation_remove() {
    todo!();
}
/// CONFIG_ROTATION_SET          Set the selected config to the round provided. Usage: CONFIG_ROTATION_SET <config> <round>
#[cfg(feature = "ap")]
pub fn config_rotation_set() {
    todo!();
}
/// CONFIG_ROTATION_TYPE         How will the CONFIG_ROTATION files load? 0-INCLUDE or 1-RINCLUDE?; Default: 0
#[cfg(feature = "ap")]
pub fn config_rotation_type() {
    todo!();
}
/// CONFIG_STORAGE               Is mainly use for non-rotation purposes, queue mainly. Usage is similar to CONFIG_ROTATION, except without round.
#[cfg(feature = "ap")]
pub fn config_storage() {
    todo!();
}
/// CONSOLE_DECORATE_ID          Decorates every line of console output with the client ID
#[cfg(feature = "ap")]
pub fn console_decorate_id() {
    todo!();
}
/// CONSOLE_DECORATE_IP          Decorates every line of console output with the client IP
#[cfg(feature = "ap")]
pub fn console_decorate_ip() {
    todo!();
}
/// CONSOLE_DECORATE_TIMESTAMP   Decorates every line of console output with the current date and time
#[cfg(feature = "ap")]
pub fn console_decorate_timestamp() {
    todo!();
}
/// CONSOLE_LOG_COLOR            Write color console messages to var/consolelogcolor.txt
#[cfg(feature = "ap")]
pub fn console_log_color() {
    todo!();
}
/// CONSOLE_LOG_COLOR_DECORATE_TIMESTAMP console_log_color_decorate_timestamp_help
#[cfg(feature = "ap")]
pub fn console_log_color_decorate_timestamp() {
    todo!();
}
/// CUSTOM_AUTHORITY             The custom authority to trigger when a player tries to login.
#[cfg(feature = "ap")]
pub fn custom_authority() {
    todo!();
}
/// CUSTOM_AUTHORITY_CONNECTION  The link to connect to when using custom authority. Do not include "http://".
#[cfg(feature = "ap")]
pub fn custom_authority_connection() {
    todo!();
}
/// CUSTOM_AUTHORITY_ENABLED     If set to 1 and CUSTOM_AUTHORITY is found, then CUSTOM_AUTHORITY_CONNECTION will be used to connect.
#[cfg(feature = "ap")]
pub fn custom_authority_enabled() {
    todo!();
}
/// CUSTOM_CENTER_MESSAGE        Send custom message in the form of a center message.
/// USAGE: CUSTOM_CENTER_MESSAGE ${language_string} param1 param2 param3 ...
#[cfg(feature = "ap")]
pub fn custom_center_message() {
    todo!();
}
/// CUSTOM_CENTER_PLAYER_MESSAGE custom_center_player_message_help
#[cfg(feature = "ap")]
pub fn custom_center_player_message() {
    todo!();
}
/// CUSTOM_CONFIGS               List of configs, seperated by ;, to load during the star-up of the client/server.
#[cfg(feature = "ap")]
pub fn custom_configs() {
    todo!();
}
/// CUSTOM_INVALID_COMMANDS      Contains the list of commands to be executed as chat commands: Usage: CUSTOM_INVALID_COMMANDS {command_method1};{command_method2};
#[cfg(feature = "ap")]
pub fn custom_invalid_commands() {
    todo!();
}
/// CUSTOM_MESSAGE               Send custom message using language string commands. Have spaces between each parameter.
/// Usage: CUSTOM_MESSAGE ${language_string} param1 param2 param3 ...
#[cfg(feature = "ap")]
pub fn custom_message() {
    todo!();
}
/// CUSTOM_PLAYER_CENTER_MESSAGE custom_player_center_message_help
#[cfg(feature = "ap")]
pub fn custom_player_center_message() {
    todo!();
}
/// CUSTOM_PLAYER_MESSAGE        Send custom message to player using language string commands. Have spaces between each parameter.
#[cfg(feature = "ap")]
pub fn custom_player_message() {
    todo!();
}
/// CUSTOM_SHORTHAND_ADD         Adding a custom shorthand with its link to store many links. USAGE: CUSTOM_SHORTHAND_ADD <shorthand> <link>
#[cfg(feature = "ap")]
pub fn custom_shorthand_add() {
    todo!();
}
/// CUSTOM_SHORTHAND_ALLOWED     You are able to enable or disable an individual shorthand. USAGE: CUSTOM_SHORTHAND_ALLOWED <shorthand> <0-disable/1-enable>
#[cfg(feature = "ap")]
pub fn custom_shorthand_allowed() {
    todo!();
}
/// CUSTOM_SHORTHAND_ENABLED     If set to 1 and CUSTOM_SHORTHAND is found, then CUSTOM_SHORTHAND_CONNECTION will be used to connect.
#[cfg(feature = "ap")]
pub fn custom_shorthand_enabled() {
    todo!();
}
/// CUSTOM_SHORTHAND_LINKS_LIST  Displays the list of links connected to the specified shorthand. USAGE: CUSTOM_SHORTHAND_LINKS_LIST <shorthand>
#[cfg(feature = "ap")]
pub fn custom_shorthand_links_list() {
    todo!();
}
/// CUSTOM_SHORTHAND_LIST        Displays a list of shorthands registered with server.
#[cfg(feature = "ap")]
pub fn custom_shorthand_list() {
    todo!();
}
/// CUSTOM_SHORTHAND_REMOVE      Removing a custom shorthand. USAGE: CUSTOM_SHORTHAND_REMOVE <shorthand>
#[cfg(feature = "ap")]
pub fn custom_shorthand_remove() {
    todo!();
}
/// CYCLE_DEATH_TELEPORT         Teleport a player instead of killing them. 1=Start position; 2=Reverse direction;
#[cfg(feature = "ap")]
pub fn cycle_death_teleport() {
    todo!();
}
/// CYCLE_DEATH_TELEPORT_EXPLOSION Spawn an explosion?
#[cfg(feature = "ap")]
pub fn cycle_death_teleport_explosion() {
    todo!();
}
/// CYCLE_DEATH_TELEPORT_RESET   Reset cycle parameters such as rubber and brakes?
#[cfg(feature = "ap")]
pub fn cycle_death_teleport_reset() {
    todo!();
}
/// CYCLE_DELAY_BONUS            Extra fudge factor to CYCLE_DELAY applied on the dedicated server only.
#[cfg(feature = "ap")]
pub fn cycle_delay_bonus() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE           Set to 1 to spawn a zone to respawn player of their death.
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE_ENEMY     Set to 1 to enable enemies entering respawn zone to respawn player.
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone_enemy() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE_ENEMY_KILL Set to 1 to enable respawn zone to kill enemies for entering its zone.
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone_enemy_kill() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE_GROWTH    The growth rate of respawn zone. Can increase(value>0) or decrease(value<0).
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone_growth() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE_RADIUS    The radius of respawn zone to spawn when player dies.
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone_radius() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE_RESPAWN   Set to 1 to enable respawn zone to reappear after vanishing.
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone_respawn() {
    todo!();
}
/// CYCLE_RESPAWN_ZONE_TYPE      The type of respawn occurs. 0-spawn on the location of death; 1-spawn on the starting location. Default: 0
#[cfg(feature = "ap")]
pub fn cycle_respawn_zone_type() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_ENEMY   If set to 1, rubber depletes for players when hitting enemys' tails.
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_enemy() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_ENEMY_OVERRIDE cycle_rubber_deplete_enemy_override_help
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_enemy_override() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_RIM     If set to 1, rubber depletes for players when hitting rim walls.
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_rim() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_RIM_OVERRIDE cycle_rubber_deplete_rim_override_help
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_rim_override() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_SELF    If set to 1, rubber depletes for players when hitting their own tails.
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_self() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_SELF_OVERRIDE cycle_rubber_deplete_self_override_help
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_self_override() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_TEAM    If set to 1, rubber depletes for players when hitting their teams' tails.
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_team() {
    todo!();
}
/// CYCLE_RUBBER_DEPLETE_TEAM_OVERRIDE cycle_rubber_deplete_team_override_help
#[cfg(feature = "ap")]
pub fn cycle_rubber_deplete_team_override() {
    todo!();
}
/// CYCLE_TURN                   Make the cycle turn. Usage: CYCLE_TURN [times to turn] <turn: left or right>.
#[cfg(feature = "ap")]
pub fn cycle_turn() {
    todo!();
}
/// CYCLE_ZONES_AIM              cycle_zones_aim_help
#[cfg(feature = "ap")]
pub fn cycle_zones_aim() {
    todo!();
}
/// CYCLE_ZONES_AIM_CHATBOT      cycle_zones_aim_chatbot_help
#[cfg(feature = "ap")]
pub fn cycle_zones_aim_chatbot() {
    todo!();
}
/// CYCLE_ZONES_AIM_TYPES        cycle_zones_aim_types_help
#[cfg(feature = "ap")]
pub fn cycle_zones_aim_types() {
    todo!();
}
/// CYCLE_ZONES_APPROACH         cycle_zones_approach_help
#[cfg(feature = "ap")]
pub fn cycle_zones_approach() {
    todo!();
}
/// CYCLE_ZONES_APPROCH          The distance a cycle can approch the zone without trigging the OnNear() event.
#[cfg(feature = "ap")]
pub fn cycle_zones_approch() {
    todo!();
}
/// CYCLE_ZONES_AVOID            If set to 1, cycles will do their best at avoiding the zone. Is slightly buggy but works often.
#[cfg(feature = "ap")]
pub fn cycle_zones_avoid() {
    todo!();
}
/// CYCLE_ZONES_AVOID_AIM_ORDER  cycle_zones_avoid_aim_order_help
#[cfg(feature = "ap")]
pub fn cycle_zones_avoid_aim_order() {
    todo!();
}
/// CYCLE_ZONES_AVOID_CHATBOT    If enabled, chatbots are also included in CYCLE_ZONES_AVOID.
#[cfg(feature = "ap")]
pub fn cycle_zones_avoid_chatbot() {
    todo!();
}
/// CYCLE_ZONES_AVOID_OLD        cycle_zones_avoid_old_help
#[cfg(feature = "ap")]
pub fn cycle_zones_avoid_old() {
    todo!();
}
/// CYCLE_ZONES_AVOID_RANGE      cycle_zones_avoid_range_help
#[cfg(feature = "ap")]
pub fn cycle_zones_avoid_range() {
    todo!();
}
/// DEADLY_EXPLOSIONS            Should cycles in the blast radius of an explosion be destroyed?
#[cfg(feature = "ap")]
pub fn deadly_explosions() {
    todo!();
}
/// DEATHZONE_RANDOM_COLORS      Default: 0; If set to 1, deathzones will have their colors by randomness.
#[cfg(feature = "ap")]
pub fn deathzone_random_colors() {
    todo!();
}
/// DEATHZONE_ROTATION           If set to 1, DEATHZONE_ROTATION_SPEED will be used for the speed of deathzones.
#[cfg(feature = "ap")]
pub fn deathzone_rotation() {
    todo!();
}
/// DEATHZONE_ROTATION_SPEED     The speed at which the rotation of the deathzones. Negative values cause it to spin in the other way.
#[cfg(feature = "ap")]
pub fn deathzone_rotation_speed() {
    todo!();
}
/// DEATH_SHOT                   If set to 1, killed players will release a death shot if they had been about to shoot.
#[cfg(feature = "ap")]
pub fn death_shot() {
    todo!();
}
/// DEDICATED_FPS                Maximum simulation steps per second the dedicated server will perform
#[cfg(feature = "ap")]
pub fn dedicated_fps() {
    todo!();
}
/// DEDICATED_FPS_IDLE_FACTOR    Number of times per frame the server should check whether simulation can be done if no network input is coming
#[cfg(feature = "ap")]
pub fn dedicated_fps_idle_factor() {
    todo!();
}
/// DEFAULT_MAP_FILE             The default map to revert to when no players are active
#[cfg(feature = "ap")]
pub fn default_map_file() {
    todo!();
}
/// DEFAULT_MAP_FILE_ON_EMPTY    If set to 1, the DEFAULT_MAP_FILE is selected when no players are active
#[cfg(feature = "ap")]
pub fn default_map_file_on_empty() {
    todo!();
}
/// DELAY_COMMAND                A command to execute at given time. Usage: DELAY_COMMAND [time] [command] [parameters] ...
#[cfg(feature = "ap")]
pub fn delay_command() {
    todo!();
}
/// DELAY_COMMAND_CLEAR          Clears all delayed command from cache.
#[cfg(feature = "ap")]
pub fn delay_command_clear() {
    todo!();
}
/// DELAY_COMMAND_REMOVE         Removes a delay command at the specified id number. Usage: DELAY_COMMAND_REMOVE [id] ...
#[cfg(feature = "ap")]
pub fn delay_command_remove() {
    todo!();
}
/// DEOP                         Reverses /op; it takes away a player's access level, effectively making them unauthenticated again.
#[cfg(feature = "ap")]
pub fn deop() {
    todo!();
}
/// DESTROY_ALL                  Causes all zones to vanish instantly.
#[cfg(feature = "ap")]
pub fn destroy_all() {
    todo!();
}
/// DESTROY_ZONE                 Destroy, simply meaning: causes the zone with the given name to disappear instantly.
#[cfg(feature = "ap")]
pub fn destroy_zone() {
    todo!();
}
/// DESTROY_ZONE_ID              Destroy, simply meaning: causes the zone with the given id to disappear instantly.
#[cfg(feature = "ap")]
pub fn destroy_zone_id() {
    todo!();
}
/// DISPLAY_SCORES_DURING_CHAT   If enabled, scores will continue showing while in chat.
#[cfg(feature = "ap")]
pub fn display_scores_during_chat() {
    todo!();
}
/// ENABLE_FRIENDS_CASING        If set to 1, matching friends will appear. If set to 0, no matter what casing it is, names with our friends will appear.
#[cfg(feature = "ap")]
pub fn enable_friends_casing() {
    todo!();
}
/// EXIT                         Shuts the dedicated server down and quits.
#[cfg(feature = "ap")]
pub fn exit() {
    todo!();
}
/// FLAG_BLINK_END               Percentage of the flag radius to end the flag blink at.
#[cfg(feature = "ap")]
pub fn flag_blink_end() {
    todo!();
}
/// FLAG_BLINK_ESTIMATE_POSITION 0 to start the flag blink at the current player position, 1 to start the flag blink where the player would be at the end of the blink at current speed and direction
#[cfg(feature = "ap")]
pub fn flag_blink_estimate_position() {
    todo!();
}
/// FLAG_BLINK_ON_TIME           Time in seconds that flag is on in a blink (not recommended to set this below 0.1)
#[cfg(feature = "ap")]
pub fn flag_blink_on_time() {
    todo!();
}
/// FLAG_BLINK_START             Percentage of the flag radius to start the flag blink at.
#[cfg(feature = "ap")]
pub fn flag_blink_start() {
    todo!();
}
/// FLAG_BLINK_TIME              Time in seconds between flag blinking over player with the flag, -1 to disable
#[cfg(feature = "ap")]
pub fn flag_blink_time() {
    todo!();
}
/// FLAG_BLINK_TRACK_TIME        If set above zero, this tracks the cycle position and speed at the rate defined by this setting.  it is not recommended to set this below 0.1 for lag reasons.
#[cfg(feature = "ap")]
pub fn flag_blink_track_time() {
    todo!();
}
/// FLAG_CHAT_BLINK_TIME         Time in seconds that the chat triangle above a player with a flag will blink, -1 to disable
#[cfg(feature = "ap")]
pub fn flag_chat_blink_time() {
    todo!();
}
/// FLAG_COLOR_B                 (0-15) blue color for a neutral flag
#[cfg(feature = "ap")]
pub fn flag_color_b() {
    todo!();
}
/// FLAG_COLOR_G                 (0-15) green color for a neutral flag
#[cfg(feature = "ap")]
pub fn flag_color_g() {
    todo!();
}
/// FLAG_COLOR_R                 (0-15) red color for a neutral flag
#[cfg(feature = "ap")]
pub fn flag_color_r() {
    todo!();
}
/// FLAG_CONQUEST_WINS_ROUND     Flag indicating whether capturing the flag wins the round or not
#[cfg(feature = "ap")]
pub fn flag_conquest_wins_round() {
    todo!();
}
/// FLAG_CONTROLS                Is a player allowed to use flag commands (/drop, /pass) ?
#[cfg(feature = "ap")]
pub fn flag_controls() {
    todo!();
}
/// FLAG_DROP_HOME               Flag indicating whether dropping the flag sends it home
#[cfg(feature = "ap")]
pub fn flag_drop_home() {
    todo!();
}
/// FLAG_DROP_TIME               If positive, enables player to drop flag by chatting "/drop". value is the number of seconds they can't pick up the flag afterwards, 2-3 recommended.
#[cfg(feature = "ap")]
pub fn flag_drop_time() {
    todo!();
}
/// FLAG_HOLD_SCORE              Points given for holding the flag see FLAG_HOLD_SCORE_TIME
#[cfg(feature = "ap")]
pub fn flag_hold_score() {
    todo!();
}
/// FLAG_HOLD_SCORE_TIME         Seconds until points are awarded for holding the flag see FLAG_HOLD_SCORE
#[cfg(feature = "ap")]
pub fn flag_hold_score_time() {
    todo!();
}
/// FLAG_HOLD_TIME               Time in seconds that the player can hold the flag before it is returned home, -1 to disable
#[cfg(feature = "ap")]
pub fn flag_hold_time() {
    todo!();
}
/// FLAG_HOLD_TIME_DROP          Whether the flag is sent back home or dropped where it is after FLAG_HOLD_TIME
#[cfg(feature = "ap")]
pub fn flag_hold_time_drop() {
    todo!();
}
/// FLAG_HOME_RANDOMNESS_X       Y direction the flag can vary from its starting spot when returned.
#[cfg(feature = "ap")]
pub fn flag_home_randomness_x() {
    todo!();
}
/// FLAG_HOME_RANDOMNESS_Y       X direction the flag can vary from its starting spot when returned.
#[cfg(feature = "ap")]
pub fn flag_home_randomness_y() {
    todo!();
}
/// FLAG_PASS_DISTANCE           The distance in which the team member should be in order to receive the flag.
#[cfg(feature = "ap")]
pub fn flag_pass_distance() {
    todo!();
}
/// FLAG_PASS_MODE               The mode of selection for passing the flag; o-disable, 1-closest, 2-furthest, 3-distance, 4-name.
#[cfg(feature = "ap")]
pub fn flag_pass_mode() {
    todo!();
}
/// FLAG_PASS_SPEED              The speed at which the flag should be passed (+ the speed the receive is travelling at).
#[cfg(feature = "ap")]
pub fn flag_pass_speed() {
    todo!();
}
/// FLAG_REQUIRED_HOME           Flag indicating whether flags need to be home to score
#[cfg(feature = "ap")]
pub fn flag_required_home() {
    todo!();
}
/// FLAG_TEAM                    0 - Flags are neutral, 1 Flags have team that own them
#[cfg(feature = "ap")]
pub fn flag_team() {
    todo!();
}
/// FORBID_HUD_MAP               Disallow clients to display the HUD minimap?
#[cfg(feature = "ap")]
pub fn forbid_hud_map() {
    todo!();
}
/// FORCE_RESPAWN_SCRIPT         Spawns an external script from a scripts/ subdirectory on the data path after killing the other possibly running instance.
#[cfg(feature = "ap")]
pub fn force_respawn_script() {
    todo!();
}
/// FULLSCREEN_PLAYER_MESSAGE    Prints a big message all over the screen only to the specified player without pausing the game. Use with care.
#[cfg(feature = "ap")]
pub fn fullscreen_player_message() {
    todo!();
}
/// GAME_SP_HUMANS               game_sp_humans_help
#[cfg(feature = "ap")]
pub fn game_sp_humans() {
    todo!();
}
/// GAME_WAIT_PLAYERS_ENABLED    game_wait_players_enabled_help
#[cfg(feature = "ap")]
pub fn game_wait_players_enabled() {
    todo!();
}
/// GET_CURRENT_MAP              Displays the current map players are playing in.
#[cfg(feature = "ap")]
pub fn get_current_map() {
    todo!();
}
/// GIVE_POINTS                  "Hugs" a player, giving them a specified amount of points.
#[cfg(feature = "ap")]
pub fn give_points() {
    todo!();
}
/// GLANCE_FORWARD_TOOLTIP       glance_forward_tooltip_help
#[cfg(feature = "ap")]
pub fn glance_forward_tooltip() {
    todo!();
}
/// GOAL_ROUND_END               Flag indicating whether the round ends when a goal is shot
#[cfg(feature = "ap")]
pub fn goal_round_end() {
    todo!();
}
/// HELP                         /help
#[cfg(feature = "ap")]
pub fn help() {
    todo!();
}
/// HELP_MESSAGE                 A help message sent to those calling it. Works through "/help" as well.
#[cfg(feature = "ap")]
pub fn help_message() {
    todo!();
}
/// HELP_MESSAGE_TYPE            Set 0 to use HELP_{ADD|REMOVE}_TOPIC commands. Set 1 to use HELP_MESSAGE. Default: 0;
#[cfg(feature = "ap")]
pub fn help_message_type() {
    todo!();
}
/// IDLE_KICK_EXEMPT             Exempt the access_level from being idle kicked. USAGE: IDLE_KICK_EXEMPT [access_level]
#[cfg(feature = "ap")]
pub fn idle_kick_exempt() {
    todo!();
}
/// INTERCEPT_COMMANDS           List of chat commands to accept and log to stdout.
#[cfg(feature = "ap")]
pub fn intercept_commands() {
    todo!();
}
/// INTERCEPT_UNKNOWN_COMMANDS   If 1, accept and log all unknown chat commands.
#[cfg(feature = "ap")]
pub fn intercept_unknown_commands(enabled: bool) {
    println!("{} {}", Command::InterceptUnknownCommands, enabled.byte());
}
/// KILL_ALL                     Kills everyone on the grid.
#[cfg(feature = "ap")]
pub fn kill_all() {
    todo!();
}
/// KILL_ALL_SCRIPTS             Kills all active scripts.
#[cfg(feature = "ap")]
pub fn kill_all_scripts() {
    todo!();
}
/// KILL_ID                      Kill a specific player using their id.
#[cfg(feature = "ap")]
pub fn kill_id() {
    todo!();
}
/// KILL_SCRIPT                  Kills a script. Argument must match the SPAWN_SCRIPT argument.
#[cfg(feature = "ap")]
pub fn kill_script() {
    todo!();
}
/// KOH_SCORE                    Score given for being the only one in a zone for KOH_SCORE_TIME
#[cfg(feature = "ap")]
pub fn koh_score() {
    todo!();
}
/// KOH_SCORE_TIME               The interval that KOH_SCORE is added
#[cfg(feature = "ap")]
pub fn koh_score_time() {
    todo!();
}
/// LADDERLOG_ENABLED            If set to 1, ladderlog output is enabled.
#[cfg(feature = "ap")]
pub fn ladderlog_enabled() {
    todo!();
}
/// LADDERLOG_OBJECTZONE_PLAYER_ENTERED_INSIDE ladderlog_objectzone_player_entered_inside_help
#[cfg(feature = "ap")]
pub fn ladderlog_objectzone_player_entered_inside() {
    todo!();
}
/// LADDERLOG_OBJECTZONE_ZONE_ENTERED_POLLRATE Rate at which zones are checked for interactions with objectzones. Lower values increase accuracy at the cost of performance. -1 disables.
#[cfg(feature = "ap")]
pub fn ladderlog_objectzone_zone_entered_pollrate() {
    todo!();
}
/// LADDERLOG_WRITE_ADMIN_COMMAND Write to ladderlog: ADMIN_COMMAND <name> <setting>
#[cfg(feature = "ap")]
pub fn ladderlog_write_admin_command() {
    todo!();
}
/// LADDERLOG_WRITE_ADMIN_LOGIN  Write to ladderlog: ADMIN_LOGIN [login_name] [ip_address]
#[cfg(feature = "ap")]
pub fn ladderlog_write_admin_login() {
    todo!();
}
/// LADDERLOG_WRITE_ADMIN_LOGOUT Write to ladderlog: ADMIN_LOGOUT [login_name] [ip_address]
#[cfg(feature = "ap")]
pub fn ladderlog_write_admin_logout() {
    todo!();
}
/// LADDERLOG_WRITE_AI_POSITIONS If set to 1, the team positions for AI Teams will output under "POSITIONS"
#[cfg(feature = "ap")]
pub fn ladderlog_write_ai_positions() {
    todo!();
}
/// LADDERLOG_WRITE_BALL_VANISH  Write to ladderlog: BALL_VANISH <object id> <zone_name> <cx> <cy>
#[cfg(feature = "ap")]
pub fn ladderlog_write_ball_vanish() {
    todo!();
}
/// LADDERLOG_WRITE_BASEZONE_CONQUERER_TEAM Write to ladderlog: BASEZONE_CONQUERER_TEAM <team> <score>
#[cfg(feature = "ap")]
pub fn ladderlog_write_basezone_conquerer_team() {
    todo!();
}
/// LADDERLOG_WRITE_BASE_ENEMY_RESPAWN Write to ladderlog: BASE_ENEMY_RESPAWN  <spawner> <spawned>
#[cfg(feature = "ap")]
pub fn ladderlog_write_base_enemy_respawn() {
    todo!();
}
/// LADDERLOG_WRITE_BASE_RESPAWN Write to ladderlog: BASE_RESPAWN <spawner> <spawned>
#[cfg(feature = "ap")]
pub fn ladderlog_write_base_respawn() {
    todo!();
}
/// LADDERLOG_WRITE_BLASTZONE_PLAYER_ENTER Write to ladderlog: DEATH_BLASTZONE_PLAYER_ENTER <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_blastzone_player_enter() {
    todo!();
}
/// LADDERLOG_WRITE_COMMAND      ladderlog_write_command_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_command() {
    todo!();
}
/// LADDERLOG_WRITE_CURRENT_MAP  Write to ladderlog: CURRENT_MAP [size_factor] [size_multiplier] [MAP_FILE]
#[cfg(feature = "ap")]
pub fn ladderlog_write_current_map() {
    todo!();
}
/// LADDERLOG_WRITE_CUSTOM_INVALID_COMMAND Write to ladderlog: CUSTOM_INVALID_COMMAND <command> <player_log> <ip> <access_level> <params>
#[cfg(feature = "ap")]
pub fn ladderlog_write_custom_invalid_command() {
    todo!();
}
/// LADDERLOG_WRITE_CYCLE_CREATED Write to ladderlog: CYCLE_CREATED [auth_name] [posx] [posy] [dirx] [diry] [team_name] [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_cycle_created() {
    todo!();
}
/// LADDERLOG_WRITE_CYCLE_DEATH_TELEPORT Write to ladderlog: CYCLE_DEATH_TELEPORT [auth_name] [posx] [posy] [dirx] [diry] [team_name] [time] [reason] [predator]
#[cfg(feature = "ap")]
pub fn ladderlog_write_cycle_death_teleport() {
    todo!();
}
/// LADDERLOG_WRITE_CYCLE_DESTROYED Write to ladderlog: CYCLE_DESTROYED [auth_name] [posx] [posy] [dirx] [diry] [team_name] [time] [reason] [predator]
#[cfg(feature = "ap")]
pub fn ladderlog_write_cycle_destroyed() {
    todo!();
}
/// LADDERLOG_WRITE_DEATHZONE_ACTIVATED Write to ladderlog: DEATHZONE_ACTIVATED [id] [name] [xpos] [ypos]
#[cfg(feature = "ap")]
pub fn ladderlog_write_deathzone_activated() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_BASEZONE_CONQUERED Write to ladderlog: DEATH_BASEZONE_CONQUERED <player> [NO_ENEMIES]
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_basezone_conquered() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_DEATHSHOT Write to ladderlog: DEATH_DEATHSHOT <prey> <predator>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_deathshot() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_DEATHZONE Write to ladderlog: DEATH_DEATHZONE <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_deathzone() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_DEATHZONE_TEAM Write to ladderlog: DEATH_DEATHZONE_TEAM <team> <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_deathzone_team() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_EXPLOSION Write to ladderlog: DEATH_EXPLOSION <prey> <predator>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_explosion() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_RUBBERZONE Write to ladderlog: DEATH_RUBBERZONE <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_rubberzone() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_SELF_DESTRUCT Write to ladderlog: DEATH_SELF_DESTRUCT <prey> <predator>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_self_destruct() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_SHOT_FRAG Write to ladderlog: DEATH_SHOT_FRAG <prey> <predator>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_shot_frag() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_SHOT_SUICIDE Write to ladderlog: DEATH_SHOT_SUICIDE <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_shot_suicide() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_SHOT_TEAMKILL Write to ladderlog: DEATH_SHOT_TEAMKILL <prey> <predator>
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_shot_teamkill() {
    todo!();
}
/// LADDERLOG_WRITE_DEATH_ZOMBIEZONE Write to ladderlog: DEATH_ZOMBIEZONE <prey> [predator]
#[cfg(feature = "ap")]
pub fn ladderlog_write_death_zombiezone() {
    todo!();
}
/// LADDERLOG_WRITE_END_CHALLENGE Write to ladderlog: END_CHALLENGE [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_end_challenge() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_CONQUEST_ROUND_WIN Write to ladderlog: FLAG_CONQUEST_ROUND_WIN <player> <flag team>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_conquest_round_win() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_DROP    Write to ladderlog: FLAG_DROP <player> <flag team>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_drop() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_HELD    Write to ladderlog: FLAG_HELD <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_held() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_RETURN  Write to ladderlog: FLAG_RETURN <player>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_return() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_SCORE   Write to ladderlog: FLAG_SCORE <player> <flag team>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_score() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_TAKE    Write to ladderlog: FLAG_TAKE <player> <flag team>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_take() {
    todo!();
}
/// LADDERLOG_WRITE_FLAG_TIMEOUT Write to ladderlog: FLAG_TIMEOUT <player> <flag team>
#[cfg(feature = "ap")]
pub fn ladderlog_write_flag_timeout() {
    todo!();
}
/// LADDERLOG_WRITE_INVALID_COMMAND Write to ladderlog: INVALID_COMMAND [command] [player_username] [ip_address] [access_level] [params]
#[cfg(feature = "ap")]
pub fn ladderlog_write_invalid_command() {
    todo!();
}
/// LADDERLOG_WRITE_MATCH_ENDED  Write to ladderlog: MATCH_ENDED [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_match_ended() {
    todo!();
}
/// LADDERLOG_WRITE_MATCH_SCORE  Write to ladderlog: MATCH_SCORE [player_score] [player_username] [team_name]
#[cfg(feature = "ap")]
pub fn ladderlog_write_match_score() {
    todo!();
}
/// LADDERLOG_WRITE_MATCH_SCORE_TEAM Write to ladderlog: MATCH_SCORE_TEAM [team_score] [team_name] [sets_won]
#[cfg(feature = "ap")]
pub fn ladderlog_write_match_score_team() {
    todo!();
}
/// LADDERLOG_WRITE_NEW_SET      Write to ladderlog: NEW_SET [current_set] [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_new_set() {
    todo!();
}
/// LADDERLOG_WRITE_NEXT_ROUND   Write to ladderlog: NEXT_ROUND [next_round_number] [total_rounds] [map_file] [center_message]
#[cfg(feature = "ap")]
pub fn ladderlog_write_next_round() {
    todo!();
}
/// LADDERLOG_WRITE_OBJECTZONE_PLAYER_ENTERED Write to ladderlog: OBJECTZONE_PLAYER_ENTERED [zone_id] [zone_name] [zone_pos_x] [zone_pos_y] [player_name] [player_pos_x] [player_pos_y] [player_direction_x] [player_direction_y] [game_time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_objectzone_player_entered() {
    todo!();
}
/// LADDERLOG_WRITE_OBJECTZONE_PLAYER_LEFT Write to ladderlog: OBJECTZONE_PLAYER_LEFT [zone_id] [zone_name] [zone_pos_x] [zone_pos_y] [player_name] [player_pos_x] [player_pos_y] [player_direction_x] [player_direction_y] [game_time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_objectzone_player_left() {
    todo!();
}
/// LADDERLOG_WRITE_OBJECTZONE_SPAWNED Write to ladderlog: OBJECTZONE_SPAWNED [id] [name] [pos_x] [pos_y] [xdir] [ydir]
#[cfg(feature = "ap")]
pub fn ladderlog_write_objectzone_spawned() {
    todo!();
}
/// LADDERLOG_WRITE_OBJECTZONE_ZONE_ENTERED Write to ladderlog: OBJECTZONE_ZONE_ENTERED [zone_id] [zone_name] [zone_posx] [zone_posy] [target_id] [target_name] [target_pos_x] [target_pos_y] [target_dir_x] [target_dir_y] [game_time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_objectzone_zone_entered() {
    todo!();
}
/// LADDERLOG_WRITE_OBJECTZONE_ZONE_LEFT ladderlog_write_objectzone_zone_left_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_objectzone_zone_left() {
    todo!();
}
/// LADDERLOG_WRITE_ONLINE_AI    Write to ladderlog: ONLINE_AI <name> <team> <score>
#[cfg(feature = "ap")]
pub fn ladderlog_write_online_ai() {
    todo!();
}
/// LADDERLOG_WRITE_ONLINE_PLAYERS_ALIVE Write to ladderlog: ONLINE_PLAYERS_ALIVE <player1> <player2> <player3> ...
#[cfg(feature = "ap")]
pub fn ladderlog_write_online_players_alive() {
    todo!();
}
/// LADDERLOG_WRITE_ONLINE_PLAYERS_COUNT Write to ladderlog: ONLINE_PLAYERS_COUNT <humans> <ais> <humans alive> <ai alive> <humans dead> <ai dead>
#[cfg(feature = "ap")]
pub fn ladderlog_write_online_players_count() {
    todo!();
}
/// LADDERLOG_WRITE_ONLINE_PLAYERS_DEAD Write to ladderlog: ONLINE_PLAYERS_DEAD <player1> <player2> <player3> ...
#[cfg(feature = "ap")]
pub fn ladderlog_write_online_players_dead() {
    todo!();
}
/// LADDERLOG_WRITE_ONLINE_TEAM  Write to ladderlog: ONLINE_TEAM <name> <screen name>
#[cfg(feature = "ap")]
pub fn ladderlog_write_online_team() {
    todo!();
}
/// LADDERLOG_WRITE_ONLINE_ZONE  ladderlog_write_online_zone_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_online_zone() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_AI_ENTERED Write to ladderlog: PLAYER_AI_ENTERED <name> <screen name>
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_ai_entered() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_AI_LEFT Write to ladderlog: PLAYER_AI_LEFT [ai_name]
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_ai_left() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_COLORED_NAME Write to ladderlog: PLAYER_COLORED_NAME [player_useranme] [player_colored_name]
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_colored_name() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_ENTERED_GRID Write to ladderlog: PLAYER_ENTERED_GRID <name> <IP> <screen name>
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_entered_grid() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_ENTERED_SPECTATOR Write to ladderlog: PLAYER_ENTERED_SPECTATOR <name> <IP> <screen name>
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_entered_spectator() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_GRIDPOS Write to ladderlog: PLAYER_GRIDPOS [player_username] [pos_x] [pos_y] [dir_x] [dir_y] [cycle_speed] [player_rubber] [cycle_rubber] [team] [player_braking] [player_brake_reservoir]
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_gridpos() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_JOINS_SPECTATORS ladderlog_write_player_joins_spectators_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_joins_spectators() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_KILLED Write to ladderlog: PLAYER_KILLED [player_username] [ip_address] [pos_x] [pos_y] [dir_x] [dir_y]
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_killed() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_LEAVES_SPECTATORS ladderlog_write_player_leaves_spectators_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_leaves_spectators() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_LOGIN ladderlog_write_player_login_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_login() {
    todo!();
}
/// LADDERLOG_WRITE_PLAYER_LOGOUT ladderlog_write_player_logout_help
#[cfg(feature = "ap")]
pub fn ladderlog_write_player_logout() {
    todo!();
}
/// LADDERLOG_WRITE_QUEUE_FINISHED Write to ladderlog: QUEUE_FINISHED [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_queue_finished() {
    todo!();
}
/// LADDERLOG_WRITE_QUEUE_STARTED Write to ladderlog: QUEUE_STARTED [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_queue_started() {
    todo!();
}
/// LADDERLOG_WRITE_ROUND_COMMENCING Write to ladderlog: ROUND_COMMENCING [current_round] [total_rounds]
#[cfg(feature = "ap")]
pub fn ladderlog_write_round_commencing() {
    todo!();
}
/// LADDERLOG_WRITE_ROUND_ENDED  Write to ladderlog: ROUND_ENDED [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_round_ended() {
    todo!();
}
/// LADDERLOG_WRITE_ROUND_FINISHED Write to ladderlog: ROUND_FINISHED [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_round_finished() {
    todo!();
}
/// LADDERLOG_WRITE_ROUND_STARTED Write to ladderlog: ROUND_STARTED [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_round_started() {
    todo!();
}
/// LADDERLOG_WRITE_SET_WINNER   Write to ladderlog: SET_WINNER [team_name]
#[cfg(feature = "ap")]
pub fn ladderlog_write_set_winner() {
    todo!();
}
/// LADDERLOG_WRITE_SHUTDOWN     Write to ladderlog: SHUTDOWN <time> when the server has been shut down using exit/quit commands
#[cfg(feature = "ap")]
pub fn ladderlog_write_shutdown() {
    todo!();
}
/// LADDERLOG_WRITE_SOCCER_BALL_PLAYER_ENTERED Write to ladderlog: SOCCER_BALL_PLAYER_ENTERED [player_auth_name] [team]
#[cfg(feature = "ap")]
pub fn ladderlog_write_soccer_ball_player_entered() {
    todo!();
}
/// LADDERLOG_WRITE_SOCCER_GOAL_PLAYER_ENTERED Write to ladderlog: SOCCER_GOAL_PLAYER_ENTERED [player_auth_name] [player_team] [team owner of the goal]
#[cfg(feature = "ap")]
pub fn ladderlog_write_soccer_goal_player_entered() {
    todo!();
}
/// LADDERLOG_WRITE_SOCCER_GOAL_SCORED Write to ladderlog: SOCCER_GOAL_SCORED <goal's team> <scored team> <scored player> <time>
#[cfg(feature = "ap")]
pub fn ladderlog_write_soccer_goal_scored() {
    todo!();
}
/// LADDERLOG_WRITE_SPAWN_POSITION_TEAM Write to ladderlog: SPAWN_POSITION_TEAM [team_name] [new_position]
#[cfg(feature = "ap")]
pub fn ladderlog_write_spawn_position_team() {
    todo!();
}
/// LADDERLOG_WRITE_START_CHALLENGE Write to ladderlog: START_CHALLENGE [time]
#[cfg(feature = "ap")]
pub fn ladderlog_write_start_challenge() {
    todo!();
}
/// LADDERLOG_WRITE_SVG_CREATED  Write to ladderlog: SVG_CREATED
#[cfg(feature = "ap")]
pub fn ladderlog_write_svg_created() {
    todo!();
}
/// LADDERLOG_WRITE_TACTICAL_POSITION Write to ladderlog: TACTICAL_POSITION [time] [name] [tact_pos]
#[cfg(feature = "ap")]
pub fn ladderlog_write_tactical_position() {
    todo!();
}
/// LADDERLOG_WRITE_TACTICAL_STATISTICS Write to ladderlog: TACTICAL_STATISTICS [tact_pos] [name] [time] [state] [kills]
#[cfg(feature = "ap")]
pub fn ladderlog_write_tactical_statistics() {
    todo!();
}
/// LADDERLOG_WRITE_TARGETZONE_CONQUERED Write to ladderlog: TARGETZONE_CONQUERED <object_id> <zone_name> <cx> <cy> [<player> [<team>]]
#[cfg(feature = "ap")]
pub fn ladderlog_write_targetzone_conquered() {
    todo!();
}
/// LADDERLOG_WRITE_TARGETZONE_PLAYER_ENTER Write to ladderlog: TARGETZONE_PLAYER_ENTER <object_id> <zone_name> <cx> <cy> <player> <x> <y> <xdir> <ydir> <time>
#[cfg(feature = "ap")]
pub fn ladderlog_write_targetzone_player_enter() {
    todo!();
}
/// LADDERLOG_WRITE_TARGETZONE_PLAYER_LEFT Write to ladderlog: TARGETZONE_PLAYER_LEFT <object_id> <zone_name> <cx> <cy> <player> <x> <y> <xdir> <ydir>
#[cfg(feature = "ap")]
pub fn ladderlog_write_targetzone_player_left() {
    todo!();
}
/// LADDERLOG_WRITE_TARGETZONE_TIMEOUT Write to ladderlog: TARGETZONE_TIMEOUT <object_id> <zone_name> <cx> <cy>
#[cfg(feature = "ap")]
pub fn ladderlog_write_targetzone_timeout() {
    todo!();
}
/// LADDERLOG_WRITE_TEAM_COLORED_NAME Write to ladderlog: TEAM_COLORED_NAME [team_name] [team_colored_name]
#[cfg(feature = "ap")]
pub fn ladderlog_write_team_colored_name() {
    todo!();
}
/// LADDERLOG_WRITE_VOTER        Write to ladderlog: VOTER [player_name] [0-against|1-for] [description]
#[cfg(feature = "ap")]
pub fn ladderlog_write_voter() {
    todo!();
}
/// LADDERLOG_WRITE_VOTE_CREATED Write to ladderlog: VOTE_CREATED [suggestor] [description]
#[cfg(feature = "ap")]
pub fn ladderlog_write_vote_created() {
    todo!();
}
/// LADDERLOG_WRITE_WINZONE_ACTIVATED Write to ladderlog: WINZONE_ACTIVATED [id] [name] [xpos] [ypos]
#[cfg(feature = "ap")]
pub fn ladderlog_write_winzone_activated() {
    todo!();
}
/// LADDERLOG_WRITE_WINZONE_PLAYER_ENTER Write to ladderlog: WINZONE_PLAYER_ENTER <player> <x> <y> <xdir> <ydir> <time>
#[cfg(feature = "ap")]
pub fn ladderlog_write_winzone_player_enter() {
    todo!();
}
/// LADDERLOG_WRITE_ZONE_COLLAPSED Write to ladderlog: ZONE_COLLAPSED <zone_id> <object_id> <cx> <cy>
#[cfg(feature = "ap")]
pub fn ladderlog_write_zone_collapsed() {
    todo!();
}
/// LADDERLOG_WRITE_ZONE_CREATED Write to ladderlog: ZONE_CREATED [effect] [id] [name] [xpos] [ypos] [xdir] [ydir]
#[cfg(feature = "ap")]
pub fn ladderlog_write_zone_created() {
    todo!();
}
/// LADDERLOG_WRITE_ZONE_GRIDPOS Write to ladderlog: ZONE_GRIDPOS [effect] [id] [name] [radius] [growth] [posx] [posy] [velx] [vely] [r] [g] [b]
#[cfg(feature = "ap")]
pub fn ladderlog_write_zone_gridpos() {
    todo!();
}
/// LADDERLOG_WRITE_ZONE_ROUTE_STOPPED Write to ladderlog: ZONE_ROUTE_STOPPED [effect] [id] [name] [posx] [posy] [velx] [vely]
#[cfg(feature = "ap")]
pub fn ladderlog_write_zone_route_stopped() {
    todo!();
}
/// LADDERLOG_WRITE_ZONE_SHOT_RELEASED Write to ladderlog: ZONE_SHOT_RELEASED [0-shot|1-deathshot] [id] [player_name] [zone_pos_x] [zone_pos_y] [zone_dir_x] [zone_dir_y]
#[cfg(feature = "ap")]
pub fn ladderlog_write_zone_shot_released() {
    todo!();
}
/// LADDERLOG_WRITE_ZONE_SPAWNED Write to ladderlog: ZONE_SPAWNED <zone_effect> <object id> <zone_name> <x> <y> <xdir> <ydir>
#[cfg(feature = "ap")]
pub fn ladderlog_write_zone_spawned() {
    todo!();
}
/// LADDER_HIGHSCORE_OUTPUT      If set to >1, high scores will be announced to all players.
#[cfg(feature = "ap")]
pub fn ladder_highscore_output() {
    todo!();
}
/// LANGUAGE_RELOAD              Immediately reload languages from file.
#[cfg(feature = "ap")]
pub fn language_reload() {
    todo!();
}
/// LEGACY_LADDERLOG_COMMAND     If set to 1, COMMAND will output similar things to INVALID_COMMAND.
#[cfg(feature = "ap")]
pub fn legacy_ladderlog_command() {
    todo!();
}
/// LIMIT_ADVANCE                End the match when the first team in score is this number of points ahead of the second team
#[cfg(feature = "ap")]
pub fn limit_advance() {
    todo!();
}
/// LIMIT_SCORE_MIN_LEAD         Only consider LIMIT_SCORE when this lead is achieved by the winning team
#[cfg(feature = "ap")]
pub fn limit_score_min_lead() {
    todo!();
}
/// LIMIT_SETS                   Set the match set limit. Teams winning more sets win the match.
#[cfg(feature = "ap")]
pub fn limit_sets() {
    todo!();
}
/// LIST_ALL_COMMANDS            All commands and their values are stored in ./var/commands_list.txt
#[cfg(feature = "ap")]
pub fn list_all_commands() {
    todo!();
}
/// LIST_ALL_COMMANDS_LEVELS     All commands are their access levels are stored in ./var/commands_levels_list.txt
#[cfg(feature = "ap")]
pub fn list_all_commands_levels() {
    todo!();
}
/// LIST_SCRIPTS                 Lists active scripts.
#[cfg(feature = "ap")]
pub fn list_scripts() {
    todo!();
}
/// LOAD_CUSTOM_CONFIGS          Load the custom configs loaded in CUSTOM_CONFIGS command.
#[cfg(feature = "ap")]
pub fn load_custom_configs() {
    todo!();
}
/// LOGIN                        Using this command you can prompt/login the selected player under the <name> with the given <username>.
/// Usage: LOGIN <name> <username>.
#[cfg(feature = "ap")]
pub fn login() {
    todo!();
}
/// LOGOUT                       Using the given <name>, find the player and logs them out if they already logged in.
/// Usage: LOGOUT <name>.
#[cfg(feature = "ap")]
pub fn logout() {
    todo!();
}
/// LOG_PLAYERS_ACTIVITIES       log_players_activities_help
#[cfg(feature = "ap")]
pub fn log_players_activities() {
    todo!();
}
/// LOG_TURNS                    If set to 1, this setting will log the spawned time, death time and the positions of which players move to in the file ./var/log_turns/<name>.txt
#[cfg(feature = "ap")]
pub fn log_turns() {
    todo!();
}
/// LOG_TURNS_TIMESTAMP          If set to 1, [TIME-STAMP] <message> will be sent to all the players logging file in ./var/log_turns/<name>.txt
#[cfg(feature = "ap")]
pub fn log_turns_timestamp() {
    todo!();
}
/// LOG_TURNS_WINNER             If set to 1, spawned and finished position, direction when a player enters a win zone or a target zone for the first time to the file ./var/log_turns/winner/<name>.txt
#[cfg(feature = "ap")]
pub fn log_turns_winner() {
    todo!();
}
/// LOG_ZONE_GRIDPOS             log_zone_gridpos_help
#[cfg(feature = "ap")]
pub fn log_zone_gridpos() {
    todo!();
}
/// LOG_ZONE_GRIDPOS_ID          log_zone_gridpos_id_help
#[cfg(feature = "ap")]
pub fn log_zone_gridpos_id() {
    todo!();
}
/// MAP_ONCHANGE_INCLUDE         configuration file included before verifying the new map
#[cfg(feature = "ap")]
pub fn map_onchange_include() {
    todo!();
}
/// MAP_ROTATION                 A list of map files to rotate through, with values separated by semicolons. Optionally you can enter in the round like this: map|round_number;
#[cfg(feature = "ap")]
pub fn map_rotation() {
    todo!();
}
/// MAP_ROTATION_ADD             Add a map item to the MAP_ROTATION list of items. Optionally you can also add in the round of selection. Usage: MAP_ROTATION_ADD <map>{|round_number}
#[cfg(feature = "ap")]
pub fn map_rotation_add() {
    todo!();
}
/// MAP_ROTATION_LOAD            Loads the selected map from it's designated id from the list of MAP_ROTATION items. Usage: MAP_ROTATION_LOAD <map_id>
#[cfg(feature = "ap")]
pub fn map_rotation_load() {
    todo!();
}
/// MAP_ROTATION_REMOVE          Removed the selected map from the list of MAP_ROTATION items. Usage: MAP_ROTATION_REMOVE <map>
#[cfg(feature = "ap")]
pub fn map_rotation_remove() {
    todo!();
}
/// MAP_ROTATION_SET             Set the selected map to the round provided. Usage: MAP_ROTATION_SET <map> <round>
#[cfg(feature = "ap")]
pub fn map_rotation_set() {
    todo!();
}
/// MAP_STORAGE                  Is mainly use for non-rotation purposes, queue mainly. Usage is similar to MAP_ROTATION, except without round.
#[cfg(feature = "ap")]
pub fn map_storage() {
    todo!();
}
/// MEGA_SHOT_DIR                The number of shots released after full brake release (depending on MEGA_SHOT_THRESH value).
#[cfg(feature = "ap")]
pub fn mega_shot_dir() {
    todo!();
}
/// MEGA_SHOT_EXPLOSION          If set to 1>, explosions occur when a mega shot is released.
#[cfg(feature = "ap")]
pub fn mega_shot_explosion() {
    todo!();
}
/// MEGA_SHOT_MULT               The boost for the mega shot after being released.
#[cfg(feature = "ap")]
pub fn mega_shot_mult() {
    todo!();
}
/// MEGA_SHOT_THRESH             The amount of braking to do before ready to shoot mega shot. If set >1, mega shot is disabled. SHOT_THRESH needs to be enabled for this to work.
#[cfg(feature = "ap")]
pub fn mega_shot_thresh() {
    todo!();
}
/// MIN_FLAGS_HOME               Number of flags that must be home in order to capture a flag
#[cfg(feature = "ap")]
pub fn min_flags_home() {
    todo!();
}
/// MOVE_HERE                    move_here_help
#[cfg(feature = "ap")]
pub fn move_here() {
    todo!();
}
/// NUM_AIS_PER_ROUND            Controls how many AI players can enter at once each round.
#[cfg(feature = "ap")]
pub fn num_ais_per_round() {
    todo!();
}
/// ONLINE_STATS_INTERVAL        The time between previous "online_players_*" output to the next.
#[cfg(feature = "ap")]
pub fn online_stats_interval() {
    todo!();
}
/// OP                           Gives another player a higher or a lower access level. OP <player> [+|-]<optional access level>
#[cfg(feature = "ap")]
pub fn op() {
    todo!();
}
/// PLAYER_CENTER_MESSAGE        Sends a center message to a specified player.
#[cfg(feature = "ap")]
pub fn player_center_message() {
    todo!();
}
/// PLAYER_FULLSCREEN_MESSAGE    Prints a big message all over the screen only to the specified player without pausing the game. Use with care.
#[cfg(feature = "ap")]
pub fn player_fullscreen_message() {
    todo!();
}
/// PLAYER_GRIDPOS_INTERVAL      The time between previous "player_gridpos" output to the next.
#[cfg(feature = "ap")]
pub fn player_gridpos_interval() {
    todo!();
}
/// PLAYER_GRIDPOS_ON_TURN       Write PLAYER_GRIDPOS event when someone turns?
#[cfg(feature = "ap")]
pub fn player_gridpos_on_turn() {
    todo!();
}
/// PLAYER_UNIQUE_COLOR          Gives a player a semi-random unique color every round. Attempts to generate colors that are different from other players.
#[cfg(feature = "ap")]
pub fn player_unique_color() {
    todo!();
}
/// PORT_MAX                     The highest network port that is scanned when looking for a LAN server.
#[cfg(feature = "ap")]
pub fn port_max() {
    todo!();
}
/// PORT_MIN                     The lowest network port that is scanned when looking for a LAN server.
#[cfg(feature = "ap")]
pub fn port_min() {
    todo!();
}
/// PREDICT_WALLS                Predict cycle walls. Useful for avoiding instant kills
#[cfg(feature = "ap")]
pub fn predict_walls() {
    todo!();
}
/// QUEUERS_LIST                 Displays the list of queuers and their queues.
#[cfg(feature = "ap")]
pub fn queuers_list() {
    todo!();
}
/// QUEUE_CONFIG                 Stores config that exists in CONFIG_ROTATION. Stops rotation temporarly to complete the listed maps.
#[cfg(feature = "ap")]
pub fn queue_config() {
    todo!();
}
/// QUEUE_ENABLED                Should players be allowed to queue maps?
#[cfg(feature = "ap")]
pub fn queue_enabled() {
    todo!();
}
/// QUEUE_GIVE                   Give a set of queues to the given player's name. Usage: QUEUE_GIVE <name> <amount>
#[cfg(feature = "ap")]
pub fn queue_give() {
    todo!();
}
/// QUEUE_INCREMENT              If set to >0, players will get their queues increased during refill by this amount.
#[cfg(feature = "ap")]
pub fn queue_increment() {
    todo!();
}
/// QUEUE_LIMIT                  This is the amount that players can use up for queueing maps or configs.
#[cfg(feature = "ap")]
pub fn queue_limit() {
    todo!();
}
/// QUEUE_LIMIT_ENABLED          Should the people have limits when queueing?
#[cfg(feature = "ap")]
pub fn queue_limit_enabled() {
    todo!();
}
/// QUEUE_LIMIT_EXCEMPT          Access level equal to or below this do not have queue limit.
#[cfg(feature = "ap")]
pub fn queue_limit_excempt() {
    todo!();
}
/// QUEUE_LOG                    If set to 1, players queueing maps/configs will get written to queuelog.txt
#[cfg(feature = "ap")]
pub fn queue_log() {
    todo!();
}
/// QUEUE_MAP                    Stores map that exist in MAP_ROTATION. Stops rotation temporarly to complete the listed maps.
#[cfg(feature = "ap")]
pub fn queue_map() {
    todo!();
}
/// QUEUE_MAX                    The maximum queues allowed due to the increase in their slots.
#[cfg(feature = "ap")]
pub fn queue_max() {
    todo!();
}
/// QUEUE_REFILL                 Refill the queue fuel of the given player's name. Usage: QUEUE_REFILL <name>
#[cfg(feature = "ap")]
pub fn queue_refill() {
    todo!();
}
/// QUEUE_REFILL_ACTIVE          Should players be in server to have their queue refill active?
#[cfg(feature = "ap")]
pub fn queue_refill_active() {
    todo!();
}
/// QUEUE_REFILL_TIME            How long each time should players refill take? This is measured in hours.
#[cfg(feature = "ap")]
pub fn queue_refill_time() {
    todo!();
}
/// QUIT                         Shuts the dedicated server down and quits.
#[cfg(feature = "ap")]
pub fn quit() {
    todo!();
}
/// RACE_CHANCES                 The number of chances player get to play again in the same round after death. Depletes each time you use it up and resets for next round.
#[cfg(feature = "ap")]
pub fn race_chances() {
    todo!();
}
/// RACE_CHECKPOINT_COUNTDOWN    Number of seconds to give individual racers to complete the race.
#[cfg(feature = "ap")]
pub fn race_checkpoint_countdown() {
    todo!();
}
/// RACE_CHECKPOINT_LAPS         Default: 1; 0-won't do anything; 1-after each completed lap, your completed checkpoints data is cleared to do again.
#[cfg(feature = "ap")]
pub fn race_checkpoint_laps() {
    todo!();
}
/// RACE_CHECKPOINT_REQUIRE_HIT  Default: 1; 0-will let you finish regarless of doing the checkpoints; 1-MUST complete all checkpoints but not in order; 2-MUST complete all checkpoints in order;
#[cfg(feature = "ap")]
pub fn race_checkpoint_require_hit() {
    todo!();
}
/// RACE_END_DELAY               Number of seconds to give players to finish before the round is finished.
#[cfg(feature = "ap")]
pub fn race_end_delay() {
    todo!();
}
/// RACE_FINISH_COLLAPSE         If set to 1, all zones will collapse at the end of round.
#[cfg(feature = "ap")]
pub fn race_finish_collapse() {
    todo!();
}
/// RACE_FINISH_KILL             If set to 1, players crossing the finish line will get killed.
#[cfg(feature = "ap")]
pub fn race_finish_kill() {
    todo!();
}
/// RACE_IDLE_KILL               If set to 1, kills players that are idle for RACE_IDLE_TIME seconds.
#[cfg(feature = "ap")]
pub fn race_idle_kill() {
    todo!();
}
/// RACE_IDLE_SPEED              Set to >= 0, idle activates if players stay for RACE_IDLE_TIME under the set idle speed.
#[cfg(feature = "ap")]
pub fn race_idle_speed() {
    todo!();
}
/// RACE_IDLE_TIME               Number of seconds a player is idle on grid before being warned and then killed.
#[cfg(feature = "ap")]
pub fn race_idle_time() {
    todo!();
}
/// RACE_IDLE_WARNINGS           The number of times a player should be warned for being idle.
#[cfg(feature = "ap")]
pub fn race_idle_warnings() {
    todo!();
}
/// RACE_LAPS                    If set to >1, these are the number of laps to complete to finish the race.
#[cfg(feature = "ap")]
pub fn race_laps() {
    todo!();
}
/// RACE_LOG_LOGIN               If enabled, it will only log the time records of players that have logged in.
#[cfg(feature = "ap")]
pub fn race_log_login() {
    todo!();
}
/// RACE_LOG_TIME                If enabled, it displays the reached time and the position of arrival.
#[cfg(feature = "ap")]
pub fn race_log_time() {
    todo!();
}
/// RACE_LOG_UNFINISHED          It set to 1, logs in the players that have not yet finished that racing course. Time values will be set to -1.
#[cfg(feature = "ap")]
pub fn race_log_unfinished() {
    todo!();
}
/// RACE_NUM_RANKS_SHOW_END      The number of ranks to display at the end of round.
#[cfg(feature = "ap")]
pub fn race_num_ranks_show_end() {
    todo!();
}
/// RACE_NUM_RANKS_SHOW_START    The number of ranks to display at the start of round.
#[cfg(feature = "ap")]
pub fn race_num_ranks_show_start() {
    todo!();
}
/// RACE_POINTS_TYPE             If set to 0, players receive points depending on SCORE_RACE_FINISH. if set to 1, players receive points depending on RACE_SCORE_DEPLETE.
#[cfg(feature = "ap")]
pub fn race_points_type() {
    todo!();
}
/// RACE_RANKS_SHOW_END          If set to 1, ranks will appear at the end of the round; If set to 2, personal ranks will be shown to those players only.
#[cfg(feature = "ap")]
pub fn race_ranks_show_end() {
    todo!();
}
/// RACE_RANKS_SHOW_START        If set to 1, ranks will appear at the start of the round; If set to 2, personal ranks will be shown to those players only.
#[cfg(feature = "ap")]
pub fn race_ranks_show_start() {
    todo!();
}
/// RACE_RANK_HEADER_LENGTH      The length of the header "rank" should be.
#[cfg(feature = "ap")]
pub fn race_rank_header_length() {
    todo!();
}
/// RACE_RANK_HEADER_ORDER       race_rank_header_order_help
#[cfg(feature = "ap")]
pub fn race_rank_header_order() {
    todo!();
}
/// RACE_RANK_HEADER_PLAYER_LENGTH The length of the header "player" should be.
#[cfg(feature = "ap")]
pub fn race_rank_header_player_length() {
    todo!();
}
/// RACE_RANK_HEADER_PLAYER_ORDER race_rank_header_player_order_help
#[cfg(feature = "ap")]
pub fn race_rank_header_player_order() {
    todo!();
}
/// RACE_RANK_HEADER_TIME_LENGTH The length of the header "time" should be.
#[cfg(feature = "ap")]
pub fn race_rank_header_time_length() {
    todo!();
}
/// RACE_RANK_HEADER_TIME_ORDER  race_rank_header_time_order_help
#[cfg(feature = "ap")]
pub fn race_rank_header_time_order() {
    todo!();
}
/// RACE_RANK_SHOW_LENGTH        The length of the name of the rank should the rank be aligned by.
#[cfg(feature = "ap")]
pub fn race_rank_show_length() {
    todo!();
}
/// RACE_RANK_SHOW_PLAYER_LENGTH The limit length of players to display in the display of ranks.
#[cfg(feature = "ap")]
pub fn race_rank_show_player_length() {
    todo!();
}
/// RACE_RECORDS_LOAD            Default: 1, if set to 0, race records will not load.
#[cfg(feature = "ap")]
pub fn race_records_load() {
    todo!();
}
/// RACE_RECORDS_SAVE            Default: 1, if set to 0, race records will not save.
#[cfg(feature = "ap")]
pub fn race_records_save() {
    todo!();
}
/// RACE_SAFE_ANGLES             These are the angles that are safe to travel in. Anything else and your dead. Usage: degrees1,degrees2,degrees3,...
#[cfg(feature = "ap")]
pub fn race_safe_angles() {
    todo!();
}
/// RACE_SCORE_DEPLETE           Number the score depletes by everytime a player enters the win zone.
#[cfg(feature = "ap")]
pub fn race_score_deplete() {
    todo!();
}
/// RACE_SMART_TIMER             If set to 1, timer is decided depending on the top 3 racing ranks.
#[cfg(feature = "ap")]
pub fn race_smart_timer() {
    todo!();
}
/// RACE_SMART_TIMER_FACTOR      The factor by which countdown is multiplied when smart timer is enabled.
#[cfg(feature = "ap")]
pub fn race_smart_timer_factor() {
    todo!();
}
/// RACE_SMART_TIMER_NUM         The number of records to look to obtain the average time for the countdown.
#[cfg(feature = "ap")]
pub fn race_smart_timer_num() {
    todo!();
}
/// RACE_TIMER_ENABLED           0 = Disable, 1 = Enable) race timer. Don't change during round.
#[cfg(feature = "ap")]
pub fn race_timer_enabled() {
    todo!();
}
/// RACE_UNSAFE_ANGLES_KILL      Default: 0; If set to 1, kills all players that are in the unsafe angles as they finish the race.
#[cfg(feature = "ap")]
pub fn race_unsafe_angles_kill() {
    todo!();
}
/// RELOAD_CONFIG                Reload the initial settings that are loaded during the beginning of the client/server.
#[cfg(feature = "ap")]
pub fn reload_config() {
    todo!();
}
/// RESET_CONFIG_QUEUEING        Reset config queueing.
#[cfg(feature = "ap")]
pub fn reset_config_queueing() {
    todo!();
}
/// RESET_MAP_QUEUEING           Reset map queueing.
#[cfg(feature = "ap")]
pub fn reset_map_queueing() {
    todo!();
}
/// RESET_ROTATION               Resets map and config rotation
#[cfg(feature = "ap")]
pub fn reset_rotation() {
    todo!();
}
/// RESET_ROTATION_ON_START_NEW_MATCH If enabled, map and config rotation will be reset when a START_NEW_MATCH command is issued
#[cfg(feature = "ap")]
pub fn reset_rotation_on_start_new_match() {
    todo!();
}
/// RESPAWN                      Respawns a player that had been killed.
/// USAGE: RESPAWN <player> <xpos> <ypos> <xdir> <ydir>.
#[cfg(feature = "ap")]
pub fn respawn() {
    todo!();
}
/// RESPAWN_ALL                  Respawns all players that were killed during the round at a random spot.
#[cfg(feature = "ap")]
pub fn respawn_all() {
    todo!();
}
/// RESPAWN_DEFAULT_POSITION     The default position to respawn players at. 0="Least dangerous"; 1=Original spawnpoint; 2=Last position
#[cfg(feature = "ap")]
pub fn respawn_default_position() {
    todo!();
}
/// RESPAWN_MESSAGE              Display the "You've been respawned" message for RESPAWN?
#[cfg(feature = "ap")]
pub fn respawn_message() {
    todo!();
}
/// RESPAWN_PLAYER               Respawns a player that had been killed.
/// USAGE: RESPAWN_PLAYER <player> <xpos> <ypos> <xdir> <ydir>.
#[cfg(feature = "ap")]
pub fn respawn_player() {
    todo!();
}
/// RESPAWN_SCRIPT               Spawns an external script from a scripts/ subdirectory on the data path if no already running instance is found.
#[cfg(feature = "ap")]
pub fn respawn_script() {
    todo!();
}
/// RESPAWN_STRICT               If enabled, players in spectator or going into spectator cannot be respawned.
#[cfg(feature = "ap")]
pub fn respawn_strict() {
    todo!();
}
/// RESPAWN_TIME                 Seconds greater than 0 makes sure any dead player will be respawned within that time of them being dead. Default: -1.
#[cfg(feature = "ap")]
pub fn respawn_time() {
    todo!();
}
/// REVERT_MAP_FILE              behaviour of the map verficiation
#[cfg(feature = "ap")]
pub fn revert_map_file() {
    todo!();
}
/// ROTATION_MAX                 The maximum number of rounds the currently loaded map should remain before new map should be selected and loaded.
#[cfg(feature = "ap")]
pub fn rotation_max() {
    todo!();
}
/// ROTATION_MAX_TYPE            The type of rotation to occur at the end of ROTATION_MAX: ordered rotation or random rotation.
#[cfg(feature = "ap")]
pub fn rotation_max_type() {
    todo!();
}
/// ROTATION_MESSAGE             Display a message every round with stats about the rotation?
#[cfg(feature = "ap")]
pub fn rotation_message() {
    todo!();
}
/// ROTATION_TYPE                Determines when map and config rotation should occur. Possible values: (0) Do not do any rotation, (1) Ordered Rotate every round, (2) Ordered Rotate every match, (3) Random Rotate every round, (4) Random Rotate every match, (5) Activates ROTATION_MAX, (6) Activates for rotation where maps and configs load depending on the round they are set for.
#[cfg(feature = "ap")]
pub fn rotation_type() {
    todo!();
}
/// RUBBERZONE_RATE              Rate multiplier at which rubber zones take rubber.
#[cfg(feature = "ap")]
pub fn rubberzone_rate() {
    todo!();
}
/// SCORE_BLASTZONE              What you get for hitting the Blast Zone
#[cfg(feature = "ap")]
pub fn score_blastzone() {
    todo!();
}
/// SCORE_DEATHZONE_TEAM         What you get for hitting a team Death Zone
#[cfg(feature = "ap")]
pub fn score_deathzone_team() {
    todo!();
}
/// SCORE_DEATH_SHOT             Number of points a player gets for shooting someone with their deathshot
#[cfg(feature = "ap")]
pub fn score_death_shot() {
    todo!();
}
/// SCORE_DIFF_WIN               The number of points after SCORE_WIN to declare round winner.
#[cfg(feature = "ap")]
pub fn score_diff_win() {
    todo!();
}
/// SCORE_EXPLOSION              Points the enemy cycle destroyed in an explosion gains.
#[cfg(feature = "ap")]
pub fn score_explosion() {
    todo!();
}
/// SCORE_EXPLOSION_OWNER        Points the owner of an explosion gains for destroying another enemy cycle.
#[cfg(feature = "ap")]
pub fn score_explosion_owner() {
    todo!();
}
/// SCORE_FLAG                   Number of points a player scores on returning a captured flag to their base
#[cfg(feature = "ap")]
pub fn score_flag() {
    todo!();
}
/// SCORE_FLAG_HOME_BASE         Points to get for returning your flag home.
#[cfg(feature = "ap")]
pub fn score_flag_home_base() {
    todo!();
}
/// SCORE_GOAL                   Number of points a player scores on kicking the ball into the enemy goal
#[cfg(feature = "ap")]
pub fn score_goal() {
    todo!();
}
/// SCORE_RACE                   What you get for reaching the win zone in a race
#[cfg(feature = "ap")]
pub fn score_race() {
    todo!();
}
/// SCORE_RACE_FINISH            Points players get awarded for crossing the finish line.
#[cfg(feature = "ap")]
pub fn score_race_finish() {
    todo!();
}
/// SCORE_RUBBERZONE             Score player is given for dieing on a rubber zone.
#[cfg(feature = "ap")]
pub fn score_rubberzone() {
    todo!();
}
/// SCORE_SELF_DESTRUCT          Number of points a player gets
#[cfg(feature = "ap")]
pub fn score_self_destruct() {
    todo!();
}
/// SCORE_SHOT                   Number of points a player shoots another player
#[cfg(feature = "ap")]
pub fn score_shot() {
    todo!();
}
/// SCORE_SHOT_BASE              Points player's team receives for shooting at a base.
#[cfg(feature = "ap")]
pub fn score_shot_base() {
    todo!();
}
/// SCORE_SHOT_SUICIDE           Number of points a player shoots themselves of their teammates
#[cfg(feature = "ap")]
pub fn score_shot_suicide() {
    todo!();
}
/// SCORE_ZOMBIE_ZONE            Number of points a player gets for killing a zombie zone
#[cfg(feature = "ap")]
pub fn score_zombie_zone() {
    todo!();
}
/// SCORE_ZOMBIE_ZONE_REVENGE    Number of points a player gets for having their zombie kill someone
#[cfg(feature = "ap")]
pub fn score_zombie_zone_revenge() {
    todo!();
}
/// SCRIPT_ENV                   Set custom environment variables for scripts. Usage: SCRIPT_ENV <variable name> <value>
#[cfg(feature = "ap")]
pub fn script_env() {
    todo!();
}
/// SELF_DESTRUCT                If set to 1, once a player gets killed, a large zone will appear at the spot and kill inside of it.
#[cfg(feature = "ap")]
pub fn self_destruct() {
    todo!();
}
/// SELF_DESTRUCT_FALL           The speed at which zone's radius falls after increasing.
#[cfg(feature = "ap")]
pub fn self_destruct_fall() {
    todo!();
}
/// SELF_DESTRUCT_RADIUS         The initial radius of the destruct zone.
#[cfg(feature = "ap")]
pub fn self_destruct_radius() {
    todo!();
}
/// SELF_DESTRUCT_RISE           The speed at which zone's radius increases initially.
#[cfg(feature = "ap")]
pub fn self_destruct_rise() {
    todo!();
}
/// SELF_DESTRUCT_ROT            The speed at which the zone rotates.
#[cfg(feature = "ap")]
pub fn self_destruct_rot() {
    todo!();
}
/// SELF_DESTRUCT_VANISH         Flag for if a self distruct zone should vanish
#[cfg(feature = "ap")]
pub fn self_destruct_vanish() {
    todo!();
}
/// SERVER_OPTIONS               Short description of the options on this server
#[cfg(feature = "ap")]
pub fn server_options() {
    todo!();
}
/// SET_AI_POSITION              Set the route at which the ai player should follow. Usage: SET_AI_POSITION [name] [x1] [y1] [x2] [y2] ...
#[cfg(feature = "ap")]
pub fn set_ai_position() {
    todo!();
}
/// SET_COMMANDS_ACCESSLEVEL     Set the access level of ALL the commands to the given level.
#[cfg(feature = "ap")]
pub fn set_commands_accesslevel() {
    todo!();
}
/// SET_CYCLE_BRAKING            Sets whether a player is braking. Usage: SET_CYCLE_BRAKING <name> <braking>
#[cfg(feature = "ap")]
pub fn set_cycle_braking() {
    todo!();
}
/// SET_CYCLE_RUBBER             Set the current used up rubber of the owner: Usage: SET_CYCLE_RUBBER <name> <rubber>
#[cfg(feature = "ap")]
pub fn set_cycle_rubber() {
    todo!();
}
/// SET_CYCLE_SPEED              Set the current travelling speed of the owner: Usage: SET_CYCLE_SPEED <name> <speed>
#[cfg(feature = "ap")]
pub fn set_cycle_speed() {
    todo!();
}
/// SET_PLAYER_TEAM              Forcably place the selected player into the given team. Usage: SET_PLAYER_TEAM [name] [team]
#[cfg(feature = "ap")]
pub fn set_player_team() {
    todo!();
}
/// SET_TARGET_COMMAND           Add commands for a target zone to issue when someone enters it.
#[cfg(feature = "ap")]
pub fn set_target_command() {
    todo!();
}
/// SET_ZONE_COLOR               Change the color of a zone (out of 15 by default). Usage: SET_ZONE_COLOR <name> <r> <g> <b>
#[cfg(feature = "ap")]
pub fn set_zone_color() {
    todo!();
}
/// SET_ZONE_COORD               change a zones actual position. Usage: SET_ZONE_COORD <name> <x> <y>
#[cfg(feature = "ap")]
pub fn set_zone_coord() {
    todo!();
}
/// SET_ZONE_DIR                 change a zones direction. Usage: SET_ZONE_DIR <name> <xdir> <ydir>
#[cfg(feature = "ap")]
pub fn set_zone_dir() {
    todo!();
}
/// SET_ZONE_EXPANSION           Change the Expansion rate of a zone. Usage: SET_ZONE_EXPANSION <name> <growth>
#[cfg(feature = "ap")]
pub fn set_zone_expansion() {
    todo!();
}
/// SET_ZONE_ID_COLOR            Change the color of a zone (out of 15 by default). Usage: SET_ZONE_ID_COLOR <id> <r> <g> <b>
#[cfg(feature = "ap")]
pub fn set_zone_id_color() {
    todo!();
}
/// SET_ZONE_ID_COORD            change a zones actual position. Usage: SET_ZONE_ID_COORD <id> <x> <y>
#[cfg(feature = "ap")]
pub fn set_zone_id_coord() {
    todo!();
}
/// SET_ZONE_ID_DIR              change a zones direction. Usage: SET_ZONE_ID_DIR <id> <xdir> <ydir>
#[cfg(feature = "ap")]
pub fn set_zone_id_dir() {
    todo!();
}
/// SET_ZONE_ID_EXPANSION        Change the Expansion rate of a zone. Usage: SET_ZONE_ID_EXPANSION <id> <growth>
#[cfg(feature = "ap")]
pub fn set_zone_id_expansion() {
    todo!();
}
/// SET_ZONE_ID_PENETRATE        Change whether the zone penetrates through walls. Usage: SET_ZONE_ID_PENETRATE <id> <bool>
#[cfg(feature = "ap")]
pub fn set_zone_id_penetrate() {
    todo!();
}
/// SET_ZONE_ID_RADIUS           change zones radius. Usage: SET_ZONE_ID_RADIUS <id> <radius> <rate>
#[cfg(feature = "ap")]
pub fn set_zone_id_radius() {
    todo!();
}
/// SET_ZONE_ID_ROTATION         Change the rotation speed of a zone. Usage: SET_ZONE_ID_ROTATION <id> <rotation>
#[cfg(feature = "ap")]
pub fn set_zone_id_rotation() {
    todo!();
}
/// SET_ZONE_ID_ROUTE            Sets a zones route. Usage: SET_ZONE_ROUTE <name> <x1> <y1> [<x2> <y2> ...]
#[cfg(feature = "ap")]
pub fn set_zone_id_route() {
    todo!();
}
/// SET_ZONE_ID_SPEED            change a zones speed. Usage: SET_ZONE_ID_SPEED <id> <speed>
#[cfg(feature = "ap")]
pub fn set_zone_id_speed() {
    todo!();
}
/// SET_ZONE_PENETRATE           Change whether the zone penetrates through walls. Usage: SET_ZONE_PENETRATE <name> <bool>
#[cfg(feature = "ap")]
pub fn set_zone_penetrate() {
    todo!();
}
/// SET_ZONE_RADIUS              change zones radius. Usage: SET_ZONE_RADIUS <name> <radius> <rate>
#[cfg(feature = "ap")]
pub fn set_zone_radius() {
    todo!();
}
/// SET_ZONE_ROTATION            Change the rotation speed of a zone. Usage: SET_ZONE_ROTATION <name> <rotation>
#[cfg(feature = "ap")]
pub fn set_zone_rotation() {
    todo!();
}
/// SET_ZONE_ROUTE               Sets a zones route. Usage: SET_ZONE_ROUTE <name> <x1> <y1> [<x2> <y2> ...]
#[cfg(feature = "ap")]
pub fn set_zone_route() {
    todo!();
}
/// SET_ZONE_SPEED               change a zones speed. Usage: SET_ZONE_SPEED <name> <speed>
#[cfg(feature = "ap")]
pub fn set_zone_speed() {
    todo!();
}
/// SHOT_BASE_ENEMY_RESPAWN      If a shot enters into an enemy's base, respawn their dead team mates.
#[cfg(feature = "ap")]
pub fn shot_base_enemy_respawn() {
    todo!();
}
/// SHOT_BASE_RESPAWN            If a shot enters into their own base, respawn their dead team mates.
#[cfg(feature = "ap")]
pub fn shot_base_respawn() {
    todo!();
}
/// SHOT_COLLISION               Flag for if shots can collide and bounce off one another.
#[cfg(feature = "ap")]
pub fn shot_collision() {
    todo!();
}
/// SHOT_DISCARD_TIME            Time, in seconds, to wait before ready to shoot.
#[cfg(feature = "ap")]
pub fn shot_discard_time() {
    todo!();
}
/// SHOT_EXPLOSION               If set to 1>, explosions take place after every normal shot.
#[cfg(feature = "ap")]
pub fn shot_explosion() {
    todo!();
}
/// SHOT_KILL_ENEMIES            If set to 1, player's shot kills enemies for entering into it.
#[cfg(feature = "ap")]
pub fn shot_kill_enemies() {
    todo!();
}
/// SHOT_KILL_INVULNERABLE       Flag for if Shot can kill invulnerable cycles
#[cfg(feature = "ap")]
pub fn shot_kill_invulnerable() {
    todo!();
}
/// SHOT_KILL_SELF               Flag for if a player can shot themself or their team
#[cfg(feature = "ap")]
pub fn shot_kill_self() {
    todo!();
}
/// SHOT_KILL_VANISH             Flag for if a shot should vanish
#[cfg(feature = "ap")]
pub fn shot_kill_vanish() {
    todo!();
}
/// SHOT_PENETRATE_WALLS         Flag for if a shot should go through walls when its not bouncing
#[cfg(feature = "ap")]
pub fn shot_penetrate_walls() {
    todo!();
}
/// SHOT_RADIUS_MAX              The maximum radius of the shot zone.
#[cfg(feature = "ap")]
pub fn shot_radius_max() {
    todo!();
}
/// SHOT_RADIUS_MIN              The minimum radius of the shot zone.
#[cfg(feature = "ap")]
pub fn shot_radius_min() {
    todo!();
}
/// SHOT_ROT_MAX                 The maximum rotation of the shot zone.
#[cfg(feature = "ap")]
pub fn shot_rot_max() {
    todo!();
}
/// SHOT_ROT_MIN                 The minimum rotation of the shot zone.
#[cfg(feature = "ap")]
pub fn shot_rot_min() {
    todo!();
}
/// SHOT_SEEK_UPDATE_TIME        The interval in which the shot seeking is updated.
#[cfg(feature = "ap")]
pub fn shot_seek_update_time() {
    todo!();
}
/// SHOT_START_DIST              The distance from which the shot is released from the owner's bike.
#[cfg(feature = "ap")]
pub fn shot_start_dist() {
    todo!();
}
/// SHOT_THRESH                  The amount of braking necessary to make a shot. If >1, shooting is disabled.
#[cfg(feature = "ap")]
pub fn shot_thresh() {
    todo!();
}
/// SHOT_VELOCITY_MULT           The velocity at which the shot's velocity multiplies after being released.
#[cfg(feature = "ap")]
pub fn shot_velocity_mult() {
    todo!();
}
/// SHOT_WALL_BOUNCE             Flag for if shots can bounce off walls
#[cfg(feature = "ap")]
pub fn shot_wall_bounce() {
    todo!();
}
/// SHOW_MAP_AXES                If set to 1, shows the map's axes.
#[cfg(feature = "ap")]
pub fn show_map_axes() {
    todo!();
}
/// SHOW_MAP_CREATION            If set to 1, shows the map's name and creator's name.
#[cfg(feature = "ap")]
pub fn show_map_creation() {
    todo!();
}
/// SHOW_MAP_DETAILS             Display the map's details for everyone to view.
#[cfg(feature = "ap")]
pub fn show_map_details() {
    todo!();
}
/// SHUFFLE_PLAYER               shuffle_player_help
#[cfg(feature = "ap")]
pub fn shuffle_player() {
    todo!();
}
/// SHUTDOWN                     This command activates the shutdown process for the game. Usage: SHUTDOWN <optional: seconds>
#[cfg(feature = "ap")]
pub fn shutdown() {
    todo!();
}
/// SHUTDOWN_STOP                This command automatically stops the shutdown process if it is currently active.
#[cfg(feature = "ap")]
pub fn shutdown_stop() {
    todo!();
}
/// SHUTDOWN_TIMEOUT             This command sets the default seconds timeout before game is closed.
#[cfg(feature = "ap")]
pub fn shutdown_timeout() {
    todo!();
}
/// SILENCE_ALL                  Silence everyone present in the server.
#[cfg(feature = "ap")]
pub fn silence_all() {
    todo!();
}
/// SILENCE_DEAD                 Silence all the players that have died.
#[cfg(feature = "ap")]
pub fn silence_dead() {
    todo!();
}
/// SILENCE_ENEMIES              When enabled, chat sent from enemies is not displayed on your client if you are alive. If you are dead all chat is displayed.
#[cfg(feature = "ap")]
pub fn silence_enemies() {
    todo!();
}
/// SOCCER_BALL_FIRST_WIN        If set to 1, first team to shot the ball into other team's goal wubs the round.
#[cfg(feature = "ap")]
pub fn soccer_ball_first_win() {
    todo!();
}
/// SOCCER_BALL_SCORE_OWN_GOAL   If enabled, counts scoring your own goal as points to the enemy teams through the usual systems. If disabled, the ball will just move back to where it started.
#[cfg(feature = "ap")]
pub fn soccer_ball_score_own_goal() {
    todo!();
}
/// SOCCER_BALL_SHOTS_WIN        If set to > 0, the number of times the ball must enter other team's goal. Sending the ball in their own goal does not count.
#[cfg(feature = "ap")]
pub fn soccer_ball_shots_win() {
    todo!();
}
/// SOCCER_BALL_SLOWDOWN         If set to 1, soccer balls will slow down.
#[cfg(feature = "ap")]
pub fn soccer_ball_slowdown() {
    todo!();
}
/// SOCCER_BALL_SLOWDOWN_HACKYMETHOD soccer_ball_slowdown_hackymethod_help
#[cfg(feature = "ap")]
pub fn soccer_ball_slowdown_hackymethod() {
    todo!();
}
/// SOCCER_BALL_SLOWDOWN_SPEED   The speed at which the ball show slow down at.
#[cfg(feature = "ap")]
pub fn soccer_ball_slowdown_speed() {
    todo!();
}
/// SOCCER_BALL_SLOWDOWN_SYNC_INTERVAL Time in seconds between synchronizations when slowing down soccerballs.
#[cfg(feature = "ap")]
pub fn soccer_ball_slowdown_sync_interval() {
    todo!();
}
/// SOCCER_GOAL_KILL_ENEMIES     If set to 1, enemy players entering other team's base will get killed.
#[cfg(feature = "ap")]
pub fn soccer_goal_kill_enemies() {
    todo!();
}
/// SOCCER_GOAL_RESPAWN_ALLIES   If set to 1, teammates entering their own goal will respawn dead teammates.
#[cfg(feature = "ap")]
pub fn soccer_goal_respawn_allies() {
    todo!();
}
/// SOCCER_GOAL_RESPAWN_ENEMIES  If set to 1, players entering opponent's goal will respawn enemy dead players.
#[cfg(feature = "ap")]
pub fn soccer_goal_respawn_enemies() {
    todo!();
}
/// SOCCER_GOAL_SCORE            The points the team get for scoring a goal.
#[cfg(feature = "ap")]
pub fn soccer_goal_score() {
    todo!();
}
/// SPAWN_ALTERNATE              If set to 1, switch positions each round.
#[cfg(feature = "ap")]
pub fn spawn_alternate() {
    todo!();
}
/// SPAWN_ENABLED                spawn_enabled_help
#[cfg(feature = "ap")]
pub fn spawn_enabled() {
    todo!();
}
/// SPAWN_EXPLOSION              Spawns an explosion. Usage: SPAWN_EXPLOSION [x] [y] [radius] [r] [g] [b].
#[cfg(feature = "ap")]
pub fn spawn_explosion() {
    todo!();
}
/// SPAWN_OBJECTZONE             Spawns an object zone. Usage: SPAWN_OBJECTZONE [x] [y] [growth] [radius] [xdir] [ydir] [interact] [r] [g] [b] [target_radius]
#[cfg(feature = "ap")]
pub fn spawn_objectzone() {
    todo!();
}
/// SPAWN_SCRIPT                 Spawns an external script from a scripts/ subdirectory on the data path.
#[cfg(feature = "ap")]
pub fn spawn_script() {
    todo!();
}
/// SPAWN_SOCCER                 Spawns a soccer zone.
#[cfg(feature = "ap")]
pub fn spawn_soccer() {
    todo!();
}
/// SPAWN_WINNERS_FIRST          If set to 1, winners from previous round will be spawned first in the next round.
#[cfg(feature = "ap")]
pub fn spawn_winners_first() {
    todo!();
}
/// SPAWN_WRAP                   Number of spawns after which to start wrapping new spawns at the beginning.
#[cfg(feature = "ap")]
pub fn spawn_wrap() {
    todo!();
}
/// SPAWN_ZONE                   spawn a zone onto the grid
#[cfg(feature = "ap")]
pub fn spawn_zone() {
    todo!();
}
/// SPEAK_AS_ADMIN               Let's you speak as admin. Output:= Admin: {message}
#[cfg(feature = "ap")]
pub fn speak_as_admin() {
    todo!();
}
/// SPEAK_TO_ENEMIES             Let's you speak as admin to enemies. Output:= Admin --> Enemies: {message}
#[cfg(feature = "ap")]
pub fn speak_to_enemies() {
    todo!();
}
/// SPEAK_TO_EVERYONE            Let's you speak as admin to everyone. Output:= Admin --> Everyone: {message}
#[cfg(feature = "ap")]
pub fn speak_to_everyone() {
    todo!();
}
/// SP_LIMIT_ADVANCE             End the match in single player mode when the first team in score is this number of points ahead of the second team
#[cfg(feature = "ap")]
pub fn sp_limit_advance() {
    todo!();
}
/// SP_LIMIT_SCORE_MIN_LEAD      Only consider LIMIT_SCORE when this lead is achieved by the winning team in single player mode
#[cfg(feature = "ap")]
pub fn sp_limit_score_min_lead() {
    todo!();
}
/// SP_LIMIT_SETS                Set the match set limit in single player mode. Teams winning more sets win the match.
#[cfg(feature = "ap")]
pub fn sp_limit_sets() {
    todo!();
}
/// SP_SCORE_DIFF_WIN            The number of points after SP_SCORE_WIN to declare round winner
#[cfg(feature = "ap")]
pub fn sp_score_diff_win() {
    todo!();
}
/// STYCT_COMPATIBILITY_LADDERLOG_PLAYER_GRIDPOS PLAYER_GRIDPOS becomes: PLAYER_GRIDPOS [player_username] [pos_x] [pos_y] [dir_x] [dir_y] [team] [cycle_speed] [player_rubber] [cycle_rubber] [player_braking] [player_brake_reservoir]
#[cfg(feature = "ap")]
pub fn styct_compatibility_ladderlog_player_gridpos() {
    todo!();
}
/// STYCT_COMPATIBILITY_SET_ZONE_COLOR If eanbled, SET_ZONE_COLOR will take colors out of 1 rather than 15.
#[cfg(feature = "ap")]
pub fn styct_compatibility_set_zone_color() {
    todo!();
}
/// STYCT_COMPATIBILITY_SPAWN_ZONE If eanbled, SPAWN_ZONE will take colors out of 1 rather than 15.
#[cfg(feature = "ap")]
pub fn styct_compatibility_spawn_zone() {
    todo!();
}
/// SUICIDE_MESSAGE              If set to 1, announce when player kills themselves.
#[cfg(feature = "ap")]
pub fn suicide_message() {
    todo!();
}
/// SUSPEND_ALL                  Suspends all active players by the SUSPEND_DEFAULT_ROUNDS or <rounds> specified.
/// Usage: SUSPEND_ALL <rounds>.
#[cfg(feature = "ap")]
pub fn suspend_all() {
    todo!();
}
/// SUSPEND_LIST                 One execution and it displays the list of currently suspended players.
#[cfg(feature = "ap")]
pub fn suspend_list() {
    todo!();
}
/// SVG_ZONE_ROTATION_ANIMATE    svg_zone_rotation_animate_help
#[cfg(feature = "ap")]
pub fn svg_zone_rotation_animate() {
    todo!();
}
/// SWAP_WINZONE_DEATHZONE_COLORS 0:Default, original colors and 1:Swap, swaps their colors with each other.
#[cfg(feature = "ap")]
pub fn swap_winzone_deathzone_colors() {
    todo!();
}
/// TAKE_POINTS                  Slaps the given player by punishing them with the specified amound of points. Be careful or you will lose a of points :P
#[cfg(feature = "ap")]
pub fn take_points() {
    todo!();
}
/// TARGETZONE_COLOR_B           The blue portion of the target zone's color.
#[cfg(feature = "ap")]
pub fn targetzone_color_b() {
    todo!();
}
/// TARGETZONE_COLOR_G           The green portion of the target zone's color.
#[cfg(feature = "ap")]
pub fn targetzone_color_g() {
    todo!();
}
/// TARGETZONE_COLOR_R           The red portion of the target zone's color.
#[cfg(feature = "ap")]
pub fn targetzone_color_r() {
    todo!();
}
/// TARGET_DECLARE_WINNER        Last target zone is a winzone ?;
#[cfg(feature = "ap")]
pub fn target_declare_winner() {
    todo!();
}
/// TARGET_INITIAL_SCORE         Score for the first player entering the zone
#[cfg(feature = "ap")]
pub fn target_initial_score() {
    todo!();
}
/// TARGET_LIFETIME              Time in seconds before the zone vanished. -1 for infinite
#[cfg(feature = "ap")]
pub fn target_lifetime() {
    todo!();
}
/// TARGET_PLAYER_MULTIUSE       If enabled, a player can get points from the same target zone multiple times. If disabled, each player can only use a target zone only once.
#[cfg(feature = "ap")]
pub fn target_player_multiuse() {
    todo!();
}
/// TARGET_SCORE_DEPLETE         Score suppress to the zone score each time a player entered
#[cfg(feature = "ap")]
pub fn target_score_deplete() {
    todo!();
}
/// TARGET_SURVIVE_TIME          Time in sec before the zone vanished once a player entered. -1 for infinite
#[cfg(feature = "ap")]
pub fn target_survive_time() {
    todo!();
}
/// TELEPORT                     Teleports player to a new location. Usage: TELEPORT [player] [xpos] [ypos] [rel|abs] [xdir] [ydir]
#[cfg(feature = "ap")]
pub fn teleport() {
    todo!();
}
/// TELEPORT_PLAYER              Teleports player to a new location. Usage: TELEPORT_PLAYER [player] [xpos] [ypos] [rel|abs] [xdir] [ydir]
#[cfg(feature = "ap")]
pub fn teleport_player() {
    todo!();
}
/// TEXT_BRIGHT_BACKGROUND       Enable the bright white background behind text considered dark
#[cfg(feature = "ap")]
pub fn text_bright_background() {
    todo!();
}
/// TEXT_SHADOW                  text_shadow_help
#[cfg(feature = "ap")]
pub fn text_shadow() {
    todo!();
}
/// TIMER_MAX                    The maximum time for timer to reach.
#[cfg(feature = "ap")]
pub fn timer_max() {
    todo!();
}
/// TIMER_MIN                    The minimum time for timer to reach.
#[cfg(feature = "ap")]
pub fn timer_min() {
    todo!();
}
/// TIMER_MODE                   0-countdown ticks down, 1-countdown ticks up, 2-countdown depends on the target time.
#[cfg(feature = "ap")]
pub fn timer_mode() {
    todo!();
}
/// TIMER_RESET                  Resets the ingame timer back to default.
#[cfg(feature = "ap")]
pub fn timer_reset() {
    todo!();
}
/// TIMER_RESUME                 If the timer was previously stopped, it resumes from where it last stopped.
#[cfg(feature = "ap")]
pub fn timer_resume() {
    todo!();
}
/// TIMER_START                  Starts a ingame timer, giving players <seconds> to do something. Usage: TIMER_START <seconds> <target>
#[cfg(feature = "ap")]
pub fn timer_start() {
    todo!();
}
/// TIMER_STOP                   Perfectly stops the ingame timer.
#[cfg(feature = "ap")]
pub fn timer_stop() {
    todo!();
}
/// TIMER_TYPE                   0-do nothing, 1-kill all cycles, 2-kill all zones, 3-kill everything.
#[cfg(feature = "ap")]
pub fn timer_type() {
    todo!();
}
/// UNSILENCE_ALL                Reverts a SILENCE_ALL command
#[cfg(feature = "ap")]
pub fn unsilence_all() {
    todo!();
}
/// UNSUSPEND_ALL                Unsuspends all players that were suspended.
#[cfg(feature = "ap")]
pub fn unsuspend_all() {
    todo!();
}
/// VERIFY_COLOR_STRICT          Verify color codes before interpreting them if this is set to 1. Capital letters and anything not 0-9, a-f is not considered a color code then. If set to 0 (the in code default), out of range ascii characters are accepted and interpreted as 0.
#[cfg(feature = "ap")]
pub fn verify_color_strict() {
    todo!();
}
/// VOICE_ALL                    Reverse of SILENCE_ALL
#[cfg(feature = "ap")]
pub fn voice_all() {
    todo!();
}
/// VOTING_BIAS_CHALLENGE        Vote-specific extra bias
#[cfg(feature = "ap")]
pub fn voting_bias_challenge() {
    todo!();
}
/// WINZONE_PLAYER_ENTER_WIN     If set to 1, first player to enter the winzone will win the round.
#[cfg(feature = "ap")]
pub fn winzone_player_enter_win() {
    todo!();
}
/// ZOMBIE_ZONE                  If set to 1, zombie zones are enabled.
#[cfg(feature = "ap")]
pub fn zombie_zone() {
    todo!();
}
/// ZOMBIE_ZONE_FALL             How quickly should a zombie zone shrink in size?
#[cfg(feature = "ap")]
pub fn zombie_zone_fall() {
    todo!();
}
/// ZOMBIE_ZONE_RADIUS           The initial radius of a zombie zone.
#[cfg(feature = "ap")]
pub fn zombie_zone_radius() {
    todo!();
}
/// ZOMBIE_ZONE_RISE             How quickly should a zombie zone rise initially?
#[cfg(feature = "ap")]
pub fn zombie_zone_rise() {
    todo!();
}
/// ZOMBIE_ZONE_ROT              The speed at which the zombie zone rotates.
#[cfg(feature = "ap")]
pub fn zombie_zone_rot() {
    todo!();
}
/// ZOMBIE_ZONE_SHOOT            How much zone to take away from a zombie when a shot enters it
#[cfg(feature = "ap")]
pub fn zombie_zone_shoot() {
    todo!();
}
/// ZOMBIE_ZONE_SPEED            The speed at which the zombie zone moves as it chases after players.
#[cfg(feature = "ap")]
pub fn zombie_zone_speed() {
    todo!();
}
/// ZOMBIE_ZONE_VANISH           Flag for if a zombie zone should vanish
#[cfg(feature = "ap")]
pub fn zombie_zone_vanish() {
    todo!();
}
/// ZONES_BOUNCE_ON_CYCLE_WALLS  Flag indicating whether zones can bounce off cycle walls
#[cfg(feature = "ap")]
pub fn zones_bounce_on_cycle_walls() {
    todo!();
}
/// ZONE_DELAY_CLEAR             Clears all delayed zones from cache.
#[cfg(feature = "ap")]
pub fn zone_delay_clear() {
    todo!();
}
/// ZONE_GRIDPOS_INTERVAL        The time between previous "zone_gridpos" output to the next.
#[cfg(feature = "ap")]
pub fn zone_gridpos_interval() {
    todo!();
}
/// ZONE_HEIGHT_FORT             zone_height_fort_help
#[cfg(feature = "ap")]
pub fn zone_height_fort() {
    todo!();
}
/// ZONE_HEIGHT_KOH              zone_height_koh_help
#[cfg(feature = "ap")]
pub fn zone_height_koh() {
    todo!();
}
/// ZONE_NO_FADE_IN_SERVER       zone_no_fade_in_server_help
#[cfg(feature = "ap")]
pub fn zone_no_fade_in_server() {
    todo!();
}
/// ZONE_SEG_STEPS               zone_seg_steps_help
#[cfg(feature = "ap")]
pub fn zone_seg_steps() {
    todo!();
}
/// ZONE_SPEED_DECAY             Rate at which the zone slows down
#[cfg(feature = "ap")]
pub fn zone_speed_decay() {
    todo!();
}
/// ZONE_SPEED_HIT_DECAY         Amount the zone slows down when hitting an object
#[cfg(feature = "ap")]
pub fn zone_speed_hit_decay() {
    todo!();
}
/// ZONE_WALL_BOUNDARY           Values <= 0 is the boundary limit in which zone_wall_death will activate.
#[cfg(feature = "ap")]
pub fn zone_wall_boundary() {
    todo!();
}
/// ZONE_WALL_BOUNDARY_VALUE_RESTRICTED zone_wall_boundary_value_restricted_help
#[cfg(feature = "ap")]
pub fn zone_wall_boundary_value_restricted() {
    todo!();
}
/// ZONE_WALL_DEATH              Set to 1 to enable zones to vanish after hitting the wall boundary.
#[cfg(feature = "ap")]
pub fn zone_wall_death() {
    todo!();
}
