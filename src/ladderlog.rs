use crate::{data::*, extension::DefaultParse};

/// An entry from the ladder log
pub enum LadderLogEntry {
    /// AUTHORITY_BLURB \<blurb> \<player> \<text>
    AuthorityBlurb((), Player, String),
    /// BASEZONE_CONQUERED \<team> \<cx> \<cy>
    BasezoneConquered(Team, (f32, f32)),
    /// BASEZONE_CONQUERER \<player>
    BasezoneConquerer(Player),
    /// CHAT \<chatter> [/me] \<chat string>
    Chat(Player, SlashMe, String),
    /// DEATH_FRAG \<prey> \<predator>
    DeathFrag(Player, Player),
    /// DEATH_SUICIDE \<player>
    DeathSuicide(Player),
    /// DEATH_TEAMKILL \<prey> \<predator>
    DeathTeamkill(Player, Player),
    /// ENCODING \<charset>. Specifies the encoding for data in ladderlog.txt.
    Encoding(String),
    /// GAME_END \<date and time>
    GameEnd(Time),
    /// GAME_TIME \<time> (see also: GAME_TIME_INTERVAL)
    GameTime(Duration),
    /// MATCH_WINNER \<team> \<players>
    MatchWinner(Team, TeamMembers),
    /// NEW_MATCH \<date and time>
    NewMatch(Time),
    /// NEW_ROUND \<date and time>
    NewRound(Time),
    /// NUM_HUMANS \<number of humans>
    NumHumans(u64),
    /// ONLINE_PLAYER \<name> [\<ping> [\<team>]]
    OnlinePlayer(Player, Option<Ping>, Option<Team>),
    /// PLAYER_ENTERED \<name> \<IP> \<screen name>
    PlayerEntered(Player, IpAddr, String),
    /// PLAYER_LEFT \<name> \<IP>
    PlayerLeft(Player, IpAddr),
    /// PLAYER_RENAMED \<old name> \<new name> \<ip> \<screen name>
    PlayerRenamed(String, Player, IpAddr, ScreenName),
    /// POSITIONS \<team> \<player1 player2 ...>
    Positions(Team, TeamMembers),
    /// ROUND_SCORE \<score difference> \<player> [\<team>]
    RoundScore(Score, Player, Option<Team>),
    /// ROUND_SCORE_TEAM \<score difference> \<team>
    RoundScoreTeam(Score, Team),
    /// ROUND_WINNER \<team> \<players>
    RoundWinner(Team, TeamMembers),
    /// SACRIFICE \<player who used the hole> <player who created the hole> <player owning the wall the hole was made into>
    Sacrifice(Player, Player, Player),
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
    /// WAIT_FOR_EXTERNAL_SCRIPT \(see also: WAIT_FOR_EXTERNAL_SCRIPT and WAIT_FOR_EXTERNAL_SCRIPT_TIMEOUT)
    WaitForExternalScript,
}

impl LadderLogEntry {
    pub fn parse(raw: &str) -> Option<LadderLogEntry> {
        let split: Vec<_> = raw.split(' ').collect();
        match split[0] {
            "AUTHORITY_BLURB" => Some(LadderLogEntry::AuthorityBlurb(
                (),
                Player::parse_or_default(split[2]),
                split[3].to_string(),
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
            "CHAT" => Some(LadderLogEntry::Chat(
                Player::parse_or_default(split[1]),
                SlashMe::parse_or_default(split[2]),
                split[3].to_string(),
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
            "ENCODING" => Some(LadderLogEntry::Encoding(split[1].to_string())),
            "GAME_END" => Some(LadderLogEntry::GameEnd(Time::parse_or_default(split[1]))),
            "GAME_TIME" => Some(LadderLogEntry::GameTime(Duration::parse_or_default(
                split[1],
            ))),
            "MATCH_WINNER" => Some(LadderLogEntry::MatchWinner(
                Team::parse_or_default(split[1]),
                TeamMembers::from_slice(&split[2..]),
            )),
            "NEW_MATCH" => Some(LadderLogEntry::NewMatch(Time::parse_or_default(split[1]))),
            "NEW_ROUND" => Some(LadderLogEntry::NewRound(Time::parse_or_default(split[1]))),
            "NUM_HUMANS" => Some(LadderLogEntry::NumHumans(u64::parse_or_default(split[1]))),
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
                split[3].to_string(),
            )),
            "PLAYER_LEFT" => Some(LadderLogEntry::PlayerLeft(
                Player::parse_or_default(split[1]),
                IpAddr::parse_or_default(split[2]),
            )),
            "PLAYER_RENAMED" => Some(LadderLogEntry::PlayerRenamed(
                split[1].to_string(),
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
                split[1].to_string(),
                Team::parse_or_default(split[2]),
            )),
            _ => None,
        }
    }
}
