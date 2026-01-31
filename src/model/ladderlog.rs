use crate::{extension::*, model::*};
use std::net::IpAddr;
#[cfg(feature = "ap")]
use std::path::PathBuf;

/// An entry from the ladder log
#[derive(Debug)]
pub enum LadderLogEntry {
    /// LADDERLOG_WRITE_ADMIN_COMMAND Write to ladderlog: ADMIN_COMMAND \<name\> \<setting\>
    #[cfg(feature = "ap")]
    AdminCommand(String, String),
    /// LADDERLOG_WRITE_ADMIN_LOGIN  Write to ladderlog: ADMIN_LOGIN \<login_name\> \<ip_address\>
    #[cfg(feature = "ap")]
    AdminLogin(Player, IpAddr),
    /// LADDERLOG_WRITE_ADMIN_LOGOUT Write to ladderlog: ADMIN_LOGOUT \<login_name\> \<ip_address\>
    #[cfg(feature = "ap")]
    AdminLogout(Player, IpAddr),
    /// AUTHORITY_BLURB \<blurb> \<player> \<text>
    AuthorityBlurb(String, Player, String),
    /// LADDERLOG_WRITE_BALL_VANISH  Write to ladderlog: BALL_VANISH \<object id\> \<zone_name\> \<cx\> \<cy\>
    #[cfg(feature = "ap")]
    BallVanish(String, String, (f32, f32)),
    /// LADDERLOG_WRITE_BASE_ENEMY_RESPAWN Write to ladderlog: BASE_ENEMY_RESPAWN  \<spawner\> \<spawned\>
    #[cfg(feature = "ap")]
    BaseEnemyRespawn(Player, Player),
    /// LADDERLOG_WRITE_BASE_RESPAWN Write to ladderlog: BASE_RESPAWN \<spawner\> \<spawned\>
    #[cfg(feature = "ap")]
    BaseRespawn(Player, Player),
    /// BASEZONE_CONQUERED \<team> \<cx> \<cy>
    BasezoneConquered(Team, (f32, f32)),
    /// BASEZONE_CONQUERER \<player>
    BasezoneConquerer(Player),
    /// LADDERLOG_WRITE_BASEZONE_CONQUERER_TEAM Write to ladderlog: BASEZONE_CONQUERER_TEAM \<team\> \<score\>
    #[cfg(feature = "ap")]
    BasezoneConquererTeam(Team, i32),
    /// LADDERLOG_WRITE_BLASTZONE_PLAYER_ENTER Write to ladderlog: DEATH_BLASTZONE_PLAYER_ENTER \<player\>
    #[cfg(feature = "ap")]
    BlastzonePlayerEnter(Player),
    /// CHAT \<chatter> [/me] \<chat string>
    Chat(Player, SlashMe, String),
    /// COMMAND /\<command> \<player> \<args>
    #[cfg(feature = "styct")]
    Command(String, Player, Vec<String>),
    /// LADDERLOG_WRITE_CURRENT_MAP  Write to ladderlog: CURRENT_MAP \<size_factor\> \<size_multiplier\> \<MAP_FILE\>
    #[cfg(feature = "ap")]
    CurrentMap(i32, f32, PathBuf),
    /// LADDERLOG_WRITE_CUSTOM_INVALID_COMMAND Write to ladderlog: CUSTOM_INVALID_COMMAND \<command\> \<player_log\> \<ip\> \<access_level\> \<params\>
    #[cfg(feature = "ap")]
    CustomInvalidCommand(String, Player, IpAddr, AccessLevel, Vec<String>),
    /// LADDERLOG_WRITE_CYCLE_CREATED Write to ladderlog: CYCLE_CREATED \<auth_name\> \<posx\> \<posy\> \<dirx\> \<diry\> \<team_name\> \<time\>
    #[cfg(feature = "ap")]
    CycleCreated(String, (f32, f32), (f32, f32), Team, Time),
    /// LADDERLOG_WRITE_CYCLE_DEATH_TELEPORT Write to ladderlog: CYCLE_DEATH_TELEPORT \<auth_name\> \<posx\> \<posy\> \<dirx\> \<diry\> \<team_name\> \<time\> \<reason\> \<predator\>
    #[cfg(feature = "ap")]
    CycleDeathTeleport(String, (f32, f32), (f32, f32), Team, Time, String, Player),
    /// LADDERLOG_WRITE_CYCLE_DESTROYED Write to ladderlog: CYCLE_DESTROYED \<auth_name\> \<posx\> \<posy\> \<dirx\> \<diry\> \<team_name\> \<time\> \<reason\> \<predator\>
    #[cfg(feature = "ap")]
    CycleDestroyed(String, (f32, f32), (f32, f32), Team, Time, String, Player),
    /// LADDERLOG_WRITE_DEATH_BASEZONE_CONQUERED Write to ladderlog: DEATH_BASEZONE_CONQUERED \<player\> \<NO_ENEMIES\>
    #[cfg(feature = "ap")]
    DeathBasezoneConquered(Player, Option<bool>),
    /// LADDERLOG_WRITE_DEATH_DEATHSHOT Write to ladderlog: DEATH_DEATHSHOT \<prey\> \<predator\>
    #[cfg(feature = "ap")]
    DeathDeathshot(Player, Player),
    /// LADDERLOG_WRITE_DEATH_DEATHZONE Write to ladderlog: DEATH_DEATHZONE \<player\>
    #[cfg(feature = "ap")]
    DeathDeathzone(Player),
    /// LADDERLOG_WRITE_DEATH_DEATHZONE_TEAM Write to ladderlog: DEATH_DEATHZONE_TEAM \<team\> \<player\>
    #[cfg(feature = "ap")]
    DeathDeathzoneTeam(Team, Player),
    /// LADDERLOG_WRITE_DEATH_EXPLOSION Write to ladderlog: DEATH_EXPLOSION \<prey\> \<predator\>
    #[cfg(feature = "ap")]
    DeathExplosion(Player, Player),
    /// DEATH_FRAG \<prey> \<predator>
    DeathFrag(Player, Player),
    /// LADDERLOG_WRITE_DEATH_RUBBERZONE Write to ladderlog: DEATH_RUBBERZONE \<player\>
    #[cfg(feature = "ap")]
    DeathRubberzone(Player),
    /// LADDERLOG_WRITE_DEATH_SELF_DESTRUCT Write to ladderlog: DEATH_SELF_DESTRUCT \<prey\> \<predator\>
    #[cfg(feature = "ap")]
    DeathSelfDestruct(Player, Player),
    /// LADDERLOG_WRITE_DEATH_SHOT_FRAG Write to ladderlog: DEATH_SHOT_FRAG \<prey\> \<predator\>
    #[cfg(feature = "ap")]
    DeathShotFrag(Player, Player),
    /// LADDERLOG_WRITE_DEATH_SHOT_SUICIDE Write to ladderlog: DEATH_SHOT_SUICIDE \<player\>
    #[cfg(feature = "ap")]
    DeathShotSuicide(Player),
    /// LADDERLOG_WRITE_DEATH_SHOT_TEAMKILL Write to ladderlog: DEATH_SHOT_TEAMKILL \<prey\> \<predator\>
    #[cfg(feature = "ap")]
    DeathShotTeamkill(Player, Player),
    /// DEATH_SUICIDE \<player>
    DeathSuicide(Player),
    /// DEATH_TEAMKILL \<prey> \<predator>
    DeathTeamkill(Player, Player),
    /// LADDERLOG_WRITE_DEATH_ZOMBIEZONE Write to ladderlog: DEATH_ZOMBIEZONE \<prey\> \<predator\>
    #[cfg(feature = "ap")]
    DeathZombiezone(Player, Player),
    /// LADDERLOG_WRITE_DEATHZONE_ACTIVATED Write to ladderlog: DEATHZONE_ACTIVATED <id\> \<name\> \<xpos\> \<ypos\>
    #[cfg(feature = "ap")]
    DeathzoneActivated(String, String, (f32, f32)),
    /// ENCODING \<charset>. Specifies the encoding for data in ladderlog.txt.
    Encoding(String),
    /// LADDERLOG_WRITE_END_CHALLENGE Write to ladderlog: END_CHALLENGE \<time\>
    #[cfg(feature = "ap")]
    EndChallenge(Time),
    /// LADDERLOG_WRITE_FLAG_CONQUEST_ROUND_WIN Write to ladderlog: FLAG_CONQUEST_ROUND_WIN \<player\> \<flag team\>
    #[cfg(feature = "ap")]
    FlagConquestRoundWin(Player, Team),
    /// LADDERLOG_WRITE_FLAG_DROP    Write to ladderlog: FLAG_DROP \<player\> \<flag team\>
    #[cfg(feature = "ap")]
    FlagDrop(Player, Team),
    /// LADDERLOG_WRITE_FLAG_HELD    Write to ladderlog: FLAG_HELD \<player\>
    #[cfg(feature = "ap")]
    FlagHeld(Player),
    /// LADDERLOG_WRITE_FLAG_RETURN  Write to ladderlog: FLAG_RETURN \<player\>
    #[cfg(feature = "ap")]
    FlagReturn(Player),
    /// LADDERLOG_WRITE_FLAG_SCORE   Write to ladderlog: FLAG_SCORE \<player\> \<flag team\>
    #[cfg(feature = "ap")]
    FlagScore(Player, Team),
    /// LADDERLOG_WRITE_FLAG_TAKE    Write to ladderlog: FLAG_TAKE \<player\> \<flag team\>
    #[cfg(feature = "ap")]
    FlagTake(Player, Team),
    /// LADDERLOG_WRITE_FLAG_TIMEOUT Write to ladderlog: FLAG_TIMEOUT \<player\> \<flag team\>
    #[cfg(feature = "ap")]
    FlagTimeout(Player, Team),
    /// GAME_END \<date and time>
    GameEnd(Time),
    /// GAME_TIME \<time> (see also: GAME_TIME_INTERVAL)
    GameTime(i32, f32),
    /// LADDERLOG_WRITE_INVALID_COMMAND Write to ladderlog: INVALID_COMMAND \<command\> \<player_username\> \<ip_address\> \<access_level\> \<params\>
    #[cfg(feature = "ap")]
    InvalidCommand(String, Player, IpAddr, AccessLevel, Vec<String>),
    /// LADDERLOG_WRITE_MATCH_ENDED  Write to ladderlog: MATCH_ENDED \<time\>
    #[cfg(feature = "ap")]
    MatchEnded(Time),
    /// LADDERLOG_WRITE_MATCH_SCORE  Write to ladderlog: MATCH_SCORE \<player_score\> \<player_username\> \<team_name\>
    #[cfg(feature = "ap")]
    MatchScore(i32, Player, Team),
    /// LADDERLOG_WRITE_MATCH_SCORE_TEAM Write to ladderlog: MATCH_SCORE_TEAM \<team_score\> \<team_name\> \<sets_won\>
    #[cfg(feature = "ap")]
    MatchScoreTeam(i32, Team, u32),
    /// MATCH_WINNER \<team> \<players>
    MatchWinner(Team, TeamMembers),
    /// NEW_MATCH \<date and time>
    NewMatch(Time),
    /// NEW_ROUND \<date and time>
    NewRound(Time),
    /// LADDERLOG_WRITE_NEW_SET      Write to ladderlog: NEW_SET \<current_set\> \<time\>
    #[cfg(feature = "ap")]
    NewSet(String, Time),
    /// LADDERLOG_WRITE_NEXT_ROUND   Write to ladderlog: NEXT_ROUND \<next_round_number\> \<total_rounds\> \<map_file\> \<center_message\>
    #[cfg(feature = "ap")]
    NextRound(u32, u32, PathBuf, String),
    /// NUM_HUMANS \<number of humans>
    NumHumans(u8),
    /// LADDERLOG_WRITE_OBJECTZONE_PLAYER_ENTERED Write to ladderlog: OBJECTZONE_PLAYER_ENTERED \<zone_id\> \<zone_name\> \<zone_pos_x\> \<zone_pos_y\> \<player_name\> \<player_pos_x\> \<player_pos_y\> \<player_direction_x\> \<player_direction_y\> \<game_time\>
    #[cfg(feature = "ap")]
    ObjectzonePlayerEntered(
        String,
        String,
        (f32, f32),
        Player,
        (f32, f32),
        (f32, f32),
        Duration,
    ),
    /// LADDERLOG_WRITE_OBJECTZONE_PLAYER_LEFT Write to ladderlog: OBJECTZONE_PLAYER_LEFT \<zone_id\> \<zone_name\> \<zone_pos_x\> \<zone_pos_y\> \<player_name\> \<player_pos_x\> \<player_pos_y\> \<player_direction_x\> \<player_direction_y\> \<game_time\>
    #[cfg(feature = "ap")]
    ObjectzonePlayerLeft(
        String,
        String,
        (f32, f32),
        Player,
        (f32, f32),
        (f32, f32),
        Duration,
    ),
    /// LADDERLOG_WRITE_OBJECTZONE_SPAWNED Write to ladderlog: OBJECTZONE_SPAWNED <id\> \<name\> \<pos_x\> \<pos_y\> \<xdir\> \<ydir\>
    #[cfg(feature = "ap")]
    ObjectzoneSpawned(String, String, (f32, f32), (f32, f32)),
    /// LADDERLOG_WRITE_OBJECTZONE_ZONE_ENTERED Write to ladderlog: OBJECTZONE_ZONE_ENTERED \<zone_id\> \<zone_name\> \<zone_posx\> \<zone_posy\> \<target_id\> \<target_name\> \<target_pos_x\> \<target_pos_y\> \<target_dir_x\> \<target_dir_y\> \<game_time\>
    #[cfg(feature = "ap")]
    ObjectzoneZoneEntered(
        String,
        String,
        (f32, f32),
        Player,
        (f32, f32),
        (f32, f32),
        Duration,
    ),
    /// LADDERLOG_WRITE_OBJECTZONE_ZONE_LEFT ladderlog_write_objectzone_zone_left_help
    #[cfg(feature = "ap")]
    ObjectzoneZoneLeft(
        String,
        String,
        (f32, f32),
        Player,
        (f32, f32),
        (f32, f32),
        Duration,
    ),
    /// LADDERLOG_WRITE_ONLINE_AI    Write to ladderlog: ONLINE_AI \<name\> \<team\> \<score\>
    #[cfg(feature = "ap")]
    OnlineAi(Player, Team, i32),
    /// ONLINE_PLAYER \<name> [\<ping> [\<team>]]
    OnlinePlayer(Player, Option<Ping>, Option<Team>),
    /// LADDERLOG_WRITE_ONLINE_PLAYERS_ALIVE Write to ladderlog: ONLINE_PLAYERS_ALIVE \<player1\> \<player2\> \<player3\> ...
    #[cfg(feature = "ap")]
    OnlinePlayersAlive(Vec<Player>),
    /// LADDERLOG_WRITE_ONLINE_PLAYERS_COUNT Write to ladderlog: ONLINE_PLAYERS_COUNT \<humans\> \<ais\> \<humans alive\> \<ai alive\> \<humans dead\> \<ai dead\>
    #[cfg(feature = "ap")]
    OnlinePlayersCount(u32, u32, u32, u32, u32, u32),
    /// LADDERLOG_WRITE_ONLINE_PLAYERS_DEAD Write to ladderlog: ONLINE_PLAYERS_DEAD \<player1\> \<player2\> \<player3\> ...
    #[cfg(feature = "ap")]
    OnlinePlayersDead(Vec<Player>),
    /// LADDERLOG_WRITE_ONLINE_TEAM  Write to ladderlog: ONLINE_TEAM \<name\> \<screen name\>
    #[cfg(feature = "ap")]
    OnlineTeam(Player, ScreenName),
    /// LADDERLOG_WRITE_ONLINE_ZONE  ladderlog_write_online_zone_help
    #[cfg(feature = "ap")]
    OnlineZone,
    /// LADDERLOG_WRITE_PLAYER_AI_ENTERED Write to ladderlog: PLAYER_AI_ENTERED \<name\> \<screen name\>
    #[cfg(feature = "ap")]
    PlayerAiEntered(Player, ScreenName),
    /// LADDERLOG_WRITE_PLAYER_AI_LEFT Write to ladderlog: PLAYER_AI_LEFT \<ai_name\>
    #[cfg(feature = "ap")]
    PlayerAiLeft(Player),
    /// LADDERLOG_WRITE_PLAYER_COLORED_NAME Write to ladderlog: PLAYER_COLORED_NAME \<player_useranme\> \<player_colored_name\>
    #[cfg(feature = "ap")]
    PlayerColoredName(Player, String),
    /// PLAYER_ENTERED \<name> \<IP> \<screen name>
    PlayerEntered(Player, IpAddr, String),
    /// LADDERLOG_WRITE_PLAYER_ENTERED_GRID Write to ladderlog: PLAYER_ENTERED_GRID \<name\> \<IP\> \<screen name\>
    #[cfg(feature = "ap")]
    PlayerEnteredGrid(Player, IpAddr, ScreenName),
    /// LADDERLOG_WRITE_PLAYER_ENTERED_SPECTATOR Write to ladderlog: PLAYER_ENTERED_SPECTATOR \<name\> \<IP\> \<screen name\>
    #[cfg(feature = "ap")]
    PlayerEnteredSpectator(Player, IpAddr, ScreenName),
    /// LADDERLOG_WRITE_PLAYER_GRIDPOS Write to ladderlog: PLAYER_GRIDPOS \<player_username\> \<pos_x\> \<pos_y\> \<dir_x\> \<dir_y\> \<cycle_speed\> \<player_rubber\> \<cycle_rubber\> \<team\> \<player_braking\> \<player_brake_reservoir\>
    #[cfg(feature = "ap")]
    PlayerGridpos(
        Player,
        (f32, f32),
        (f32, f32),
        f32,
        f32,
        f32,
        Team,
        bool,
        f32,
    ),
    /// LADDERLOG_WRITE_PLAYER_JOINS_SPECTATORS ladderlog_write_player_joins_spectators_help
    #[cfg(feature = "ap")]
    PlayerJoinsSpectators,
    /// LADDERLOG_WRITE_PLAYER_KILLED Write to ladderlog: PLAYER_KILLED \<player_username\> \<ip_address\> \<pos_x\> \<pos_y\> \<dir_x\> \<dir_y\>
    #[cfg(feature = "ap")]
    PlayerKilled(Player, IpAddr, (f32, f32), (f32, f32)),
    /// LADDERLOG_WRITE_PLAYER_LEAVES_SPECTATORS ladderlog_write_player_leaves_spectators_help
    #[cfg(feature = "ap")]
    PlayerLeavesSpectators,
    /// PLAYER_LEFT \<name> \<IP>
    PlayerLeft(Player, IpAddr),
    /// LADDERLOG_WRITE_PLAYER_LOGIN ladderlog_write_player_login_help
    #[cfg(feature = "ap")]
    PlayerLogin,
    /// LADDERLOG_WRITE_PLAYER_LOGOUT ladderlog_write_player_logout_help
    #[cfg(feature = "ap")]
    PlayerLogout,
    /// PLAYER_RENAMED \<old name> \<new name> \<ip> \<screen name>
    PlayerRenamed(String, Player, IpAddr, ScreenName),
    /// POSITIONS \<team> \<player1 player2 ...>
    Positions(Team, TeamMembers),
    /// LADDERLOG_WRITE_QUEUE_FINISHED Write to ladderlog: QUEUE_FINISHED \<time\>
    #[cfg(feature = "ap")]
    QueueFinished(Time),
    /// LADDERLOG_WRITE_QUEUE_STARTED Write to ladderlog: QUEUE_STARTED \<time\>
    #[cfg(feature = "ap")]
    QueueStarted(Time),
    /// LADDERLOG_WRITE_ROUND_COMMENCING Write to ladderlog: ROUND_COMMENCING \<current_round\> \<total_rounds\>
    #[cfg(feature = "ap")]
    RoundCommencing(u32, u32),
    /// LADDERLOG_WRITE_ROUND_ENDED  Write to ladderlog: ROUND_ENDED \<time\>
    #[cfg(feature = "ap")]
    RoundEnded(Time),
    /// LADDERLOG_WRITE_ROUND_FINISHED Write to ladderlog: ROUND_FINISHED \<time\>
    #[cfg(feature = "ap")]
    RoundFinished(Time),
    /// ROUND_SCORE \<score difference> \<player> [\<team>]
    RoundScore(Score, Player, Option<Team>),
    /// ROUND_SCORE_TEAM \<score difference> \<team>
    RoundScoreTeam(Score, Team),
    /// LADDERLOG_WRITE_ROUND_STARTED Write to ladderlog: ROUND_STARTED \<time\>
    #[cfg(feature = "ap")]
    RoundStarted(Time),
    /// ROUND_WINNER \<team> \<players>
    RoundWinner(Team, TeamMembers),
    /// SACRIFICE \<player who used the hole> \<player who created the hole> \<player owning the wall the hole was made into>
    Sacrifice(Player, Player, Player),
    /// LADDERLOG_WRITE_SET_WINNER   Write to ladderlog: SET_WINNER \<team_name\>
    #[cfg(feature = "ap")]
    SetWinner(Team),
    /// LADDERLOG_WRITE_SHUTDOWN     Write to ladderlog: SHUTDOWN \<time\> when the server has been shut down using exit/quit commands
    #[cfg(feature = "ap")]
    Shutdown(Time),
    /// LADDERLOG_WRITE_SOCCER_BALL_PLAYER_ENTERED Write to ladderlog: SOCCER_BALL_PLAYER_ENTERED \<player_auth_name\> \<team\>
    #[cfg(feature = "ap")]
    SoccerBallPlayerEntered(Player, Team),
    /// LADDERLOG_WRITE_SOCCER_GOAL_PLAYER_ENTERED Write to ladderlog: SOCCER_GOAL_PLAYER_ENTERED \<player_auth_name\> \<player_team\> \<team owner of the goal\>
    #[cfg(feature = "ap")]
    SoccerGoalPlayerEntered(Player, Team, Team),
    /// LADDERLOG_WRITE_SOCCER_GOAL_SCORED Write to ladderlog: SOCCER_GOAL_SCORED \<goal's team\> \<scored team\> \<scored player\> \<time\>
    #[cfg(feature = "ap")]
    SoccerGoalScored(Team, Team, Player, Time),
    /// LADDERLOG_WRITE_SPAWN_POSITION_TEAM Write to ladderlog: SPAWN_POSITION_TEAM \<team_name\> \<new_position\>
    #[cfg(feature = "ap")]
    SpawnPositionTeam(Team, (f32, f32)),
    /// LADDERLOG_WRITE_START_CHALLENGE Write to ladderlog: START_CHALLENGE \<time\>
    #[cfg(feature = "ap")]
    StartChallenge(Time),
    /// LADDERLOG_WRITE_SVG_CREATED  Write to ladderlog: SVG_CREATED
    #[cfg(feature = "ap")]
    SvgCreated,
    /// LADDERLOG_WRITE_TACTICAL_POSITION Write to ladderlog: TACTICAL_POSITION \<time\> \<name\> \<tact_pos\>
    #[cfg(feature = "ap")]
    TacticalPosition(Time, Player, String),
    /// LADDERLOG_WRITE_TACTICAL_STATISTICS Write to ladderlog: TACTICAL_STATISTICS \<tact_pos\> \<name\> \<time\> \<state\> \<kills\>
    #[cfg(feature = "ap")]
    TacticalStatistics(String, Player, Time, String, u32),
    /// LADDERLOG_WRITE_TARGETZONE_CONQUERED Write to ladderlog: TARGETZONE_CONQUERED \<object_id\> \<zone_name\> \<cx\> \<cy\> [\<player\> [\<team\>]]
    #[cfg(feature = "ap")]
    TargetzoneConquered(String, String, (f32, f32), Option<Player>, Option<Team>),
    /// LADDERLOG_WRITE_TARGETZONE_PLAYER_ENTER Write to ladderlog: TARGETZONE_PLAYER_ENTER \<object_id\> \<zone_name\> \<cx\> \<cy\> \<player\> \<x\> \<y\> \<xdir\> \<ydir\> \<time\>
    #[cfg(feature = "ap")]
    TargetzonePlayerEnter(
        String,
        String,
        (f32, f32),
        Player,
        (f32, f32),
        (f32, f32),
        Time,
    ),
    /// LADDERLOG_WRITE_TARGETZONE_PLAYER_LEFT Write to ladderlog: TARGETZONE_PLAYER_LEFT \<object_id\> \<zone_name\> \<cx\> \<cy\> \<player\> \<x\> \<y\> \<xdir\> \<ydir\>
    #[cfg(feature = "ap")]
    TargetzonePlayerLeft(
        String,
        String,
        (f32, f32),
        Player,
        (f32, f32),
        (f32, f32),
        Time,
    ),
    /// LADDERLOG_WRITE_TARGETZONE_TIMEOUT Write to ladderlog: TARGETZONE_TIMEOUT \<object_id\> \<zone_name\> \<cx\> \<cy\>
    #[cfg(feature = "ap")]
    TargetzoneTimeout(String, String, (f32, f32)),
    /// LADDERLOG_WRITE_TEAM_COLORED_NAME Write to ladderlog: TEAM_COLORED_NAME \<team_name\> \<team_colored_name\>
    #[cfg(feature = "ap")]
    TeamColoredName(Team, String),
    /// TEAM_CREATED \<team name>
    TeamCreated(Team),
    /// TEAM_DESTROYED \<team name>
    TeamDestroyed(Team),
    /// TEAM_PLAYER_ADDED \<team name> \<player>
    TeamPlayerAdded(Team, Player),
    /// TEAM_PLAYER_REMOVED \<team name> \<player>
    TeamPlayerRemoved(Team, Player),
    /// TEAM_RENAMED \<old team name> \<new team name>
    TeamRenamed(String, Team),
    /// LADDERLOG_WRITE_VOTE_CREATED Write to ladderlog: VOTE_CREATED \<suggestor\> \<description\>
    #[cfg(feature = "ap")]
    VoteCreated(Player, String),
    /// LADDERLOG_WRITE_VOTER        Write to ladderlog: VOTER \<player_name\> [0-against|1-for] \<description\>
    #[cfg(feature = "ap")]
    Voter(Player, bool, String),
    /// WAIT_FOR_EXTERNAL_SCRIPT \(see also: WAIT_FOR_EXTERNAL_SCRIPT and WAIT_FOR_EXTERNAL_SCRIPT_TIMEOUT)
    WaitForExternalScript,
    /// LADDERLOG_WRITE_WINZONE_ACTIVATED Write to ladderlog: WINZONE_ACTIVATED \<id\> \<name\> \<xpos\> \<ypos\>
    #[cfg(feature = "ap")]
    WinzoneActivated(String, String, (f32, f32)),
    /// LADDERLOG_WRITE_WINZONE_PLAYER_ENTER Write to ladderlog: WINZONE_PLAYER_ENTER \<player\> \<x\> \<y\> \<xdir\> \<ydir\> \<time\>
    #[cfg(feature = "ap")]
    WinzonePlayerEnter(Player, (f32, f32), (f32, f32), Time),
    /// LADDERLOG_WRITE_ZONE_COLLAPSED Write to ladderlog: ZONE_COLLAPSED \<zone_id\> \<object_id\> \<cx\> \<cy\>
    #[cfg(feature = "ap")]
    ZoneCollapsed(String, String, (f32, f32)),
    /// LADDERLOG_WRITE_ZONE_CREATED Write to ladderlog: ZONE_CREATED \<effect\> \<id\> \<name\> \<xpos\> \<ypos\> \<xdir\> \<ydir\>
    #[cfg(feature = "ap")]
    ZoneCreated(String, String, String, (f32, f32), (f32, f32)),
    /// LADDERLOG_WRITE_ZONE_GRIDPOS Write to ladderlog: ZONE_GRIDPOS \<effect\> \<id\> \<name\> \<radius\> \<growth\> \<posx\> \<posy\> \<velx\> \<vely\> \<r\> \<g\> \<b\>
    #[cfg(feature = "ap")]
    ZoneGridpos(
        String,
        String,
        String,
        f32,
        f32,
        (f32, f32),
        (f32, f32),
        Color,
    ),
    /// LADDERLOG_WRITE_ZONE_ROUTE_STOPPED Write to ladderlog: ZONE_ROUTE_STOPPED \<effect\> \<id\> \<name\> \<posx\> \<posy\> \<velx\> \<vely\>
    #[cfg(feature = "ap")]
    ZoneRouteStopped(String, String, String, (f32, f32), (f32, f32)),
    /// LADDERLOG_WRITE_ZONE_SHOT_RELEASED Write to ladderlog: ZONE_SHOT_RELEASED \<0-shot|1-deathshot> \<id\> \<player_name\> \<zone_pos_x\> \<zone_pos_y\> \<zone_dir_x\> \<zone_dir_y\>
    #[cfg(feature = "ap")]
    ZoneShotReleased(bool, String, String, (f32, f32), (f32, f32)),
    /// LADDERLOG_WRITE_ZONE_SPAWNED Write to ladderlog: ZONE_SPAWNED \<zone_effect\> \<object id\> \<zone_name\> \<x\> \<y\> \<xdir\> \<ydir\>
    #[cfg(feature = "ap")]
    ZoneSpawned(String, String, String, (f32, f32), (f32, f32)),
}

impl LadderLogEntry {
    pub fn parse(raw: &str) -> Option<LadderLogEntry> {
        let split: Vec<_> = raw.split(' ').collect();
        match split[0] {
            "AUTHORITY_BLURB" => Some(LadderLogEntry::AuthorityBlurb(
                split[1].trim().to_string(),
                Player::parse_or_default(split[2]),
                split[3].trim().to_string(),
            )),
            "BASEZONE_CONQUERED" => Some(LadderLogEntry::BasezoneConquered(
                Team::parse_or_default(split[1]),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
            )),
            "BASEZONE_CONQUERER" => Some(LadderLogEntry::BasezoneConquerer(
                Player::parse_or_default(split[1]),
            )),
            "CHAT" => {
                if split[2] == "/me" {
                    Some(LadderLogEntry::Chat(
                        Player::parse_or_default(split[1]),
                        SlashMe(true),
                        split[3..].join(" "),
                    ))
                } else {
                    Some(LadderLogEntry::Chat(
                        Player::parse_or_default(split[1]),
                        SlashMe(false),
                        split[2..].join(" "),
                    ))
                }
            }
            #[cfg(feature = "styct")]
            "COMMAND" => Some(LadderLogEntry::Command(
                split[1].trim()[1..].to_string(),
                Player::parse_or_default(split[2]),
                split[3..].iter().map(|x| x.trim().to_string()).collect(),
            )),
            "DEATH_FRAG" => Some(LadderLogEntry::DeathFrag(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            "DEATH_SUICIDE" => Some(LadderLogEntry::DeathSuicide(Player::parse_or_default(
                split[1],
            ))),
            "DEATH_TEAMKILL" => Some(LadderLogEntry::DeathTeamkill(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            "ENCODING" => Some(LadderLogEntry::Encoding(split[1].trim().to_string())),
            "GAME_END" => Some(LadderLogEntry::GameEnd(Time::parse_or_default(
                split[1].trim(),
            ))),
            "GAME_TIME" => Some(LadderLogEntry::GameTime(
                i32::parse_or_default(split[1]),
                f32::parse_or_default(split[2]),
            )),
            "NEW_MATCH" => Some(LadderLogEntry::NewMatch(Time::parse_or_default(
                split[1].trim(),
            ))),
            "NEW_ROUND" => Some(LadderLogEntry::NewRound(Time::parse_or_default(
                split[1].trim(),
            ))),
            "NUM_HUMANS" => Some(LadderLogEntry::NumHumans(u8::parse_or_default(
                split[1].trim(),
            ))),
            "ONLINE_PLAYER" => Some(LadderLogEntry::OnlinePlayer(
                Player::parse_or_default(split[1]),
                if split.len() > 2 {
                    Some(Ping::parse_or_default(split[2]))
                } else {
                    None
                },
                if split.len() > 3 {
                    Some(Team::parse_or_default(split[3]))
                } else {
                    None
                },
            )),
            "PLAYER_ENTERED" => Some(LadderLogEntry::PlayerEntered(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
                split[3].trim().to_string(),
            )),
            "PLAYER_LEFT" => Some(LadderLogEntry::PlayerLeft(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
            )),
            "PLAYER_RENAMED" => Some(LadderLogEntry::PlayerRenamed(
                split[1].trim().to_string(),
                Player::parse_or_default(split[2]),
                IpAddr::parse_or_default(split[3]),
                ScreenName::parse_or_default(split[4]),
            )),
            "POSITIONS" => Some(LadderLogEntry::Positions(
                Team::parse_or_default(split[1]),
                TeamMembers::from_slice(&split[2..]),
            )),
            "ROUND_SCORE" => Some(LadderLogEntry::RoundScore(
                Score::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
                if split.len() > 3 {
                    Some(Team::parse_or_default(split[3]))
                } else {
                    None
                },
            )),
            "ROUND_SCORE_TEAM" => Some(LadderLogEntry::RoundScoreTeam(
                Score::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            "ROUND_WINNER" => Some(LadderLogEntry::RoundWinner(
                Team::parse_or_default(split[1]),
                TeamMembers::from_slice(&split[2..]),
            )),
            "SACRIFICE" => Some(LadderLogEntry::Sacrifice(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
                Player::parse_or_default(split[3]),
            )),
            "TEAM_CREATED" => Some(LadderLogEntry::TeamCreated(Team::parse_or_default(
                split[1],
            ))),
            "TEAM_DESTROYED" => Some(LadderLogEntry::TeamDestroyed(Team::parse_or_default(
                split[1],
            ))),
            "TEAM_PLAYER_ADDED" => Some(LadderLogEntry::TeamPlayerAdded(
                Team::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            "TEAM_PLAYER_REMOVED" => Some(LadderLogEntry::TeamPlayerRemoved(
                Team::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            "TEAM_RENAMED" => Some(LadderLogEntry::TeamRenamed(
                split[1].trim().to_string(),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "ADMIN_COMMAND" => Some(LadderLogEntry::AdminCommand(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "ADMIN_LOGIN" => Some(LadderLogEntry::AdminLogin(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "ADMIN_LOGOUT" => Some(LadderLogEntry::AdminLogout(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "BALL_VANISH" => Some(LadderLogEntry::BallVanish(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
            )),
            #[cfg(feature = "ap")]
            "BASEZONE_CONQUERER_TEAM" => Some(LadderLogEntry::BasezoneConquererTeam(
                Team::parse_or_default(split[1]),
                i32::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "BASE_ENEMY_RESPAWN" => Some(LadderLogEntry::BaseEnemyRespawn(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "BASE_RESPAWN" => Some(LadderLogEntry::BaseRespawn(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "BLASTZONE_PLAYER_ENTER" => Some(LadderLogEntry::BlastzonePlayerEnter(
                Player::parse_or_default(split[1]),
            )),
            #[cfg(feature = "ap")]
            "CURRENT_MAP" => Some(LadderLogEntry::CurrentMap(
                i32::parse_or_default(split[1]),
                f32::parse_or_default(split[2]),
                PathBuf::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "CUSTOM_INVALID_COMMAND" => Some(LadderLogEntry::CustomInvalidCommand(
                split[1].trim().to_string(),
                Player::parse_or_default(split[2]),
                IpAddr::parse_or_default(split[3]),
                AccessLevel::parse_or_default(split[4]),
                split[5..].iter().map(|x| x.trim().to_string()).collect(),
            )),
            #[cfg(feature = "ap")]
            "CYCLE_CREATED" => Some(LadderLogEntry::CycleCreated(
                split[1].trim().to_string(),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                Team::parse_or_default(split[6]),
                Time::parse_or_default(split[7]),
            )),
            #[cfg(feature = "ap")]
            "CYCLE_DEATH_TELEPORT" => Some(LadderLogEntry::CycleDeathTeleport(
                split[1].trim().to_string(),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                Team::parse_or_default(split[6]),
                Time::parse_or_default(split[7]),
                split[8].trim().to_string(),
                Player::parse_or_default(split[9]),
            )),
            #[cfg(feature = "ap")]
            "CYCLE_DESTROYED" => Some(LadderLogEntry::CycleDestroyed(
                split[1].trim().to_string(),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                Team::parse_or_default(split[6]),
                Time::parse_or_default(split[7]),
                split[8].trim().to_string(),
                Player::parse_or_default(split[9]),
            )),
            #[cfg(feature = "ap")]
            "DEATHZONE_ACTIVATED" => Some(LadderLogEntry::DeathzoneActivated(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
            )),
            #[cfg(feature = "ap")]
            "DEATH_BASEZONE_CONQUERED" => Some(LadderLogEntry::DeathBasezoneConquered(
                Player::parse_or_default(split[1]),
                if split.len() > 2 {
                    Some(bool::parse_or_default(split[2]))
                } else {
                    None
                },
            )),
            #[cfg(feature = "ap")]
            "DEATH_DEATHSHOT" => Some(LadderLogEntry::DeathDeathshot(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_DEATHZONE" => Some(LadderLogEntry::DeathDeathzone(Player::parse_or_default(
                split[1].trim(),
            ))),
            #[cfg(feature = "ap")]
            "DEATH_DEATHZONE_TEAM" => Some(LadderLogEntry::DeathDeathzoneTeam(
                Team::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_EXPLOSION" => Some(LadderLogEntry::DeathExplosion(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_RUBBERZONE" => Some(LadderLogEntry::DeathRubberzone(Player::parse_or_default(
                split[1].trim(),
            ))),
            #[cfg(feature = "ap")]
            "DEATH_SELF_DESTRUCT" => Some(LadderLogEntry::DeathSelfDestruct(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_SHOT_FRAG" => Some(LadderLogEntry::DeathShotFrag(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_SHOT_SUICIDE" => Some(LadderLogEntry::DeathShotSuicide(
                Player::parse_or_default(split[1]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_SHOT_TEAMKILL" => Some(LadderLogEntry::DeathShotTeamkill(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "DEATH_ZOMBIEZONE" => Some(LadderLogEntry::DeathZombiezone(
                Player::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "END_CHALLENGE" => Some(LadderLogEntry::EndChallenge(Time::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "FLAG_CONQUEST_ROUND_WIN" => Some(LadderLogEntry::FlagConquestRoundWin(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "FLAG_DROP" => Some(LadderLogEntry::FlagDrop(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "FLAG_HELD" => Some(LadderLogEntry::FlagHeld(Player::parse_or_default(
                split[1].trim(),
            ))),
            #[cfg(feature = "ap")]
            "FLAG_RETURN" => Some(LadderLogEntry::FlagReturn(Player::parse_or_default(
                split[1].trim(),
            ))),
            #[cfg(feature = "ap")]
            "FLAG_SCORE" => Some(LadderLogEntry::FlagScore(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "FLAG_TAKE" => Some(LadderLogEntry::FlagTake(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "FLAG_TIMEOUT" => Some(LadderLogEntry::FlagTimeout(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "INVALID_COMMAND" => Some(LadderLogEntry::InvalidCommand(
                split[1].trim()[1..].to_string(),
                Player::parse_or_default(split[2]),
                IpAddr::parse_or_default(split[3]),
                AccessLevel::parse_or_default(split[4]),
                split[5..].iter().map(|x| x.trim().to_string()).collect(),
            )),
            #[cfg(feature = "ap")]
            "MATCH_ENDED" => Some(LadderLogEntry::MatchEnded(Time::parse_or_default(
                split[1].trim(),
            ))),
            #[cfg(feature = "ap")]
            "MATCH_SCORE" => Some(LadderLogEntry::MatchScore(
                i32::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
                Team::parse_or_default(split[3]),
            )),
            #[cfg(feature = "ap")]
            "MATCH_SCORE_TEAM" => Some(LadderLogEntry::MatchScoreTeam(
                i32::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
                u32::parse_or_default(split[3]),
            )),
            #[cfg(feature = "ap")]
            "NEW_SET" => Some(LadderLogEntry::NewSet(
                split[1].trim().to_string(),
                Time::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "NEXT_ROUND" => Some(LadderLogEntry::NextRound(
                u32::parse_or_default(split[1]),
                u32::parse_or_default(split[2]),
                PathBuf::parse_or_default(split[3]),
                split[4].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "OBJECTZONE_PLAYER_ENTERED" => Some(LadderLogEntry::ObjectzonePlayerEntered(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                Player::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                (
                    f32::parse_or_default(split[8]),
                    f32::parse_or_default(split[9]),
                ),
                Duration::parse_or_default(split[10]),
            )),
            #[cfg(feature = "ap")]
            "OBJECTZONE_PLAYER_LEFT" => Some(LadderLogEntry::ObjectzonePlayerLeft(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                Player::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                (
                    f32::parse_or_default(split[8]),
                    f32::parse_or_default(split[9]),
                ),
                Duration::parse_or_default(split[10]),
            )),
            #[cfg(feature = "ap")]
            "OBJECTZONE_SPAWNED" => Some(LadderLogEntry::ObjectzoneSpawned(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                (
                    f32::parse_or_default(split[5]),
                    f32::parse_or_default(split[6]),
                ),
            )),
            #[cfg(feature = "ap")]
            "OBJECTZONE_ZONE_ENTERED" => Some(LadderLogEntry::ObjectzoneZoneEntered(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                Player::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                (
                    f32::parse_or_default(split[8]),
                    f32::parse_or_default(split[9]),
                ),
                Duration::parse_or_default(split[10]),
            )),
            #[cfg(feature = "ap")]
            "OBJECTZONE_ZONE_LEFT" => Some(LadderLogEntry::ObjectzoneZoneLeft(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                Player::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                (
                    f32::parse_or_default(split[8]),
                    f32::parse_or_default(split[9]),
                ),
                Duration::parse_or_default(split[10]),
            )),
            #[cfg(feature = "ap")]
            "ONLINE_AI" => Some(LadderLogEntry::OnlineAi(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
                i32::parse_or_default(split[3]),
            )),
            #[cfg(feature = "ap")]
            "ONLINE_PLAYERS_ALIVE" => Some(LadderLogEntry::OnlinePlayersAlive(
                split[1..]
                    .iter()
                    .map(|x| Player::parse_or_default(x))
                    .collect(),
            )),
            #[cfg(feature = "ap")]
            "ONLINE_PLAYERS_COUNT" => Some(LadderLogEntry::OnlinePlayersCount(
                u32::parse_or_default(split[1]),
                u32::parse_or_default(split[2]),
                u32::parse_or_default(split[3]),
                u32::parse_or_default(split[4]),
                u32::parse_or_default(split[5]),
                u32::parse_or_default(split[6]),
            )),
            #[cfg(feature = "ap")]
            "ONLINE_PLAYERS_DEAD" => Some(LadderLogEntry::OnlinePlayersDead(
                split[1..]
                    .iter()
                    .map(|x| Player::parse_or_default(x))
                    .collect(),
            )),
            #[cfg(feature = "ap")]
            "ONLINE_TEAM" => Some(LadderLogEntry::OnlineTeam(
                Player::parse_or_default(split[1]),
                ScreenName::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "ONLINE_ZONE" => Some(LadderLogEntry::OnlineZone),
            #[cfg(feature = "ap")]
            "PLAYER_AI_ENTERED" => Some(LadderLogEntry::PlayerAiEntered(
                Player::parse_or_default(split[1]),
                ScreenName::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "PLAYER_AI_LEFT" => Some(LadderLogEntry::PlayerAiLeft(Player::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "PLAYER_COLORED_NAME" => Some(LadderLogEntry::PlayerColoredName(
                Player::parse_or_default(split[1]),
                split[2].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "PLAYER_ENTERED_GRID" => Some(LadderLogEntry::PlayerEnteredGrid(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
                ScreenName::parse_or_default(split[3]),
            )),
            #[cfg(feature = "ap")]
            "PLAYER_ENTERED_SPECTATOR" => Some(LadderLogEntry::PlayerEnteredSpectator(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
                ScreenName::parse_or_default(split[3]),
            )),
            #[cfg(feature = "ap")]
            "PLAYER_GRIDPOS" => Some(LadderLogEntry::PlayerGridpos(
                Player::parse_or_default(split[1]),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                f32::parse_or_default(split[6]),
                f32::parse_or_default(split[7]),
                f32::parse_or_default(split[8]),
                Team::parse_or_default(split[9]),
                bool::parse_or_default(split[10]),
                f32::parse_or_default(split[11]),
            )),
            #[cfg(feature = "ap")]
            "PLAYER_JOINS_SPECTATORS" => Some(LadderLogEntry::PlayerJoinsSpectators),
            #[cfg(feature = "ap")]
            "PLAYER_KILLED" => Some(LadderLogEntry::PlayerKilled(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                (
                    f32::parse_or_default(split[5]),
                    f32::parse_or_default(split[6]),
                ),
            )),
            #[cfg(feature = "ap")]
            "PLAYER_LEAVES_SPECTATORS" => Some(LadderLogEntry::PlayerLeavesSpectators),
            #[cfg(feature = "ap")]
            "PLAYER_LOGIN" => Some(LadderLogEntry::PlayerLogin),
            #[cfg(feature = "ap")]
            "PLAYER_LOGOUT" => Some(LadderLogEntry::PlayerLogout),
            #[cfg(feature = "ap")]
            "QUEUE_FINISHED" => Some(LadderLogEntry::QueueFinished(Time::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "QUEUE_STARTED" => Some(LadderLogEntry::QueueStarted(Time::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "ROUND_COMMENCING" => Some(LadderLogEntry::RoundCommencing(
                u32::parse_or_default(split[1]),
                u32::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "ROUND_ENDED" => Some(LadderLogEntry::RoundEnded(Time::parse_or_default(split[1]))),
            #[cfg(feature = "ap")]
            "ROUND_FINISHED" => Some(LadderLogEntry::RoundFinished(Time::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "ROUND_STARTED" => Some(LadderLogEntry::RoundStarted(Time::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "SET_WINNER" => Some(LadderLogEntry::SetWinner(Team::parse_or_default(split[1]))),
            #[cfg(feature = "ap")]
            "SHUTDOWN" => Some(LadderLogEntry::Shutdown(Time::parse_or_default(split[1]))),
            #[cfg(feature = "ap")]
            "SOCCER_BALL_PLAYER_ENTERED" => Some(LadderLogEntry::SoccerBallPlayerEntered(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
            )),
            #[cfg(feature = "ap")]
            "SOCCER_GOAL_PLAYER_ENTERED" => Some(LadderLogEntry::SoccerGoalPlayerEntered(
                Player::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
                Team::parse_or_default(split[3]),
            )),
            #[cfg(feature = "ap")]
            "SOCCER_GOAL_SCORED" => Some(LadderLogEntry::SoccerGoalScored(
                Team::parse_or_default(split[1]),
                Team::parse_or_default(split[2]),
                Player::parse_or_default(split[3]),
                Time::parse_or_default(split[4]),
            )),
            #[cfg(feature = "ap")]
            "SPAWN_POSITION_TEAM" => Some(LadderLogEntry::SpawnPositionTeam(
                Team::parse_or_default(split[1]),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
            )),
            #[cfg(feature = "ap")]
            "START_CHALLENGE" => Some(LadderLogEntry::StartChallenge(Time::parse_or_default(
                split[1],
            ))),
            #[cfg(feature = "ap")]
            "SVG_CREATED" => Some(LadderLogEntry::SvgCreated),
            #[cfg(feature = "ap")]
            "TACTICAL_POSITION" => Some(LadderLogEntry::TacticalPosition(
                Time::parse_or_default(split[1]),
                Player::parse_or_default(split[2]),
                split[3].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "TACTICAL_STATISTICS" => Some(LadderLogEntry::TacticalStatistics(
                split[1].trim().to_string(),
                Player::parse_or_default(split[2]),
                Time::parse_or_default(split[3]),
                split[4].trim().to_string(),
                u32::parse_or_default(split[5]),
            )),
            #[cfg(feature = "ap")]
            "TARGETZONE_CONQUERED" => Some(LadderLogEntry::TargetzoneConquered(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                if split.len() > 5 {
                    Some(Player::parse_or_default(split[5]))
                } else {
                    None
                },
                if split.len() > 6 {
                    Some(Team::parse_or_default(split[6]))
                } else {
                    None
                },
            )),
            #[cfg(feature = "ap")]
            "TARGETZONE_PLAYER_ENTER" => Some(LadderLogEntry::TargetzonePlayerEnter(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                Player::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                (
                    f32::parse_or_default(split[8]),
                    f32::parse_or_default(split[9]),
                ),
                Time::parse_or_default(split[10]),
            )),
            #[cfg(feature = "ap")]
            "TARGETZONE_PLAYER_LEFT" => Some(LadderLogEntry::TargetzonePlayerLeft(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
                Player::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                (
                    f32::parse_or_default(split[8]),
                    f32::parse_or_default(split[9]),
                ),
                Time::parse_or_default(split[10]),
            )),
            #[cfg(feature = "ap")]
            "TARGETZONE_TIMEOUT" => Some(LadderLogEntry::TargetzoneTimeout(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
            )),
            #[cfg(feature = "ap")]
            "TEAM_COLORED_NAME" => Some(LadderLogEntry::TeamColoredName(
                Team::parse_or_default(split[1]),
                split[2].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "VOTER" => Some(LadderLogEntry::Voter(
                Player::parse_or_default(split[0]),
                bool::parse_or_default(split[1]),
                split[2].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "VOTE_CREATED" => Some(LadderLogEntry::VoteCreated(
                Player::parse_or_default(split[1]),
                split[2].trim().to_string(),
            )),
            #[cfg(feature = "ap")]
            "WINZONE_ACTIVATED" => Some(LadderLogEntry::WinzoneActivated(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
            )),
            #[cfg(feature = "ap")]
            "WINZONE_PLAYER_ENTER" => Some(LadderLogEntry::WinzonePlayerEnter(
                Player::parse_or_default(split[1]),
                (
                    f32::parse_or_default(split[2]),
                    f32::parse_or_default(split[3]),
                ),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                Time::parse_or_default(split[6]),
            )),
            #[cfg(feature = "ap")]
            "ZONE_COLLAPSED" => Some(LadderLogEntry::ZoneCollapsed(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                (
                    f32::parse_or_default(split[3]),
                    f32::parse_or_default(split[4]),
                ),
            )),
            #[cfg(feature = "ap")]
            "ZONE_CREATED" => Some(LadderLogEntry::ZoneCreated(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                split[3].trim().to_string(),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
            )),
            #[cfg(feature = "ap")]
            "ZONE_GRIDPOS" => Some(LadderLogEntry::ZoneGridpos(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                split[3].trim().to_string(),
                f32::parse_or_default(split[4]),
                f32::parse_or_default(split[5]),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
                Color::new(
                    u8::parse_or_default(split[8]),
                    u8::parse_or_default(split[9]),
                    u8::parse_or_default(split[10]),
                ),
            )),
            #[cfg(feature = "ap")]
            "ZONE_ROUTE_STOPPED" => Some(LadderLogEntry::ZoneRouteStopped(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                split[3].trim().to_string(),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
            )),
            #[cfg(feature = "ap")]
            "ZONE_SHOT_RELEASED" => Some(LadderLogEntry::ZoneShotReleased(
                bool::parse_or_default(split[1]),
                split[2].trim().to_string(),
                split[3].trim().to_string(),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
            )),
            #[cfg(feature = "ap")]
            "ZONE_SPAWNED" => Some(LadderLogEntry::ZoneSpawned(
                split[1].trim().to_string(),
                split[2].trim().to_string(),
                split[3].trim().to_string(),
                (
                    f32::parse_or_default(split[4]),
                    f32::parse_or_default(split[5]),
                ),
                (
                    f32::parse_or_default(split[6]),
                    f32::parse_or_default(split[7]),
                ),
            )),
            _ => None,
        }
    }
}
