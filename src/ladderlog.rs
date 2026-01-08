use std::{
    net::IpAddr,
    time::{Duration, Instant},
};

use crate::data::*;

/// An entry from the ladder log
pub enum LadderLogEntry {
    /// AUTHORITY_BLURB <blurb> <player> <text>
    AuthorityBlurb((), Player, String),
    /// BASEZONE_CONQUERED <team> <cx> <cy>
    BasezoneConquered(Team, (f32, f32)),
    /// BASEZONE_CONQUERER <player>
    BasezoneConquerer(Player),
    /// CHAT <chatter> [/me] <chat string>
    Chat(Player, SlashMe, String),
    /// DEATH_FRAG <prey> <predator>
    DeathFrag(Player, Player),
    /// DEATH_SUICIDE <player>
    DeathSuicide(Player),
    /// DEATH_TEAMKILL <prey> <predator>
    DeathTeamkill(Player, Player),
    /// ENCODING <charset>. Specifies the encoding for data in ladderlog.txt.
    Encoding(String),
    /// GAME_END <date and time>
    GameEnd(Instant),
    /// GAME_TIME <time> (see also: GAME_TIME_INTERVAL)
    GameTime(Duration),
    /// MATCH_WINNER <team> <players>
    MatchWinner(Team, Vec<Player>),
    /// NEW_MATCH <date and time>
    NewMatch(Instant),
    /// NEW_ROUND <date and time>
    NewRound(Instant),
    /// NUM_HUMANS <number of humans>
    NumHumans(u64),
    /// ONLINE_PLAYER <name> [<ping> [<team>]]
    OnlinePlayer(Player, Ping, Option<Team>),
    /// PLAYER_ENTERED <name> <IP> <screen name>
    PlayerEntered(Player, IpAddr, String),
    /// PLAYER_LEFT <name> <IP>
    PlayerLeft(Player, IpAddr),
    /// PLAYER_RENAMED <old name> <new name> <ip> <screen name>
    PlayerRenamed(String, Player, IpAddr, ScreenName),
    /// POSITIONS <team> <player1 player2 ...>
    Positions(Team, Vec<Player>),
    /// ROUND_SCORE <score difference> <player> [<team>]
    RoundScore(Score, Player, Option<Team>),
    /// ROUND_SCORE_TEAM <score difference> <team>
    RoundScoreTeam(Score, Team),
    /// ROUND_WINNER <team> <players>
    RoundWinner(Team, Vec<Player>),
    /// SACRIFICE <player who used the hole> <player who created the hole> <player owning the wall the hole was made into>
    Sacrifice(Player, Player, Player),
    /// TEAM_CREATED <team name>
    TeamCreated(Team),
    /// TEAM_DESTROYED <team name>
    TeamDestroyed(Team),
    /// TEAM_PLAYER_ADDED <team name> <player>
    TeamPlayerAdded(Team, Player),
    /// TEAM_PLAYER_REMOVED <team name> <player>
    TeamPlayerRemoved(Team, Player),
    /// TEAM_RENAMED <old team name> <new team name>
    TeamRenamed(String, Team),
    /// WAIT_FOR_EXTERNAL_SCRIPT (see also: WAIT_FOR_EXTERNAL_SCRIPT and WAIT_FOR_EXTERNAL_SCRIPT_TIMEOUT)
    WaitForExternalScript,
}

impl LadderLogEntry {
    pub fn parse(raw: &str) -> Option<LadderLogEntry> {
        todo!();
    }
}
