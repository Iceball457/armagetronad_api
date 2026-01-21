use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub mod chat_command;

use crate::{
    TeamMembers,
    model::{AccessLevel, Player, Team, ladderlog::*},
    runtime::chat_command::ChatCommand,
};

pub struct RuntimeBundle<T> {
    players: HashSet<Player>,
    teams: HashMap<Team, TeamMembers>,
    num_humans: u8,
    script_data: T,
}

impl<T> RuntimeBundle<T> {
    fn new(script_data: T) -> RuntimeBundle<T> {
        RuntimeBundle {
            players: HashSet::new(),
            teams: HashMap::new(),
            num_humans: 0,
            script_data,
        }
    }
    fn add_player(&mut self, player: &Player) {
        self.players.insert(player.clone());
    }
    fn remove_player(&mut self, player: &Player) {
        self.players.remove(player);
    }
    pub fn players(&self) -> &HashSet<Player> {
        &self.players
    }
    fn add_team(&mut self, team: &Team, players: &TeamMembers) {
        self.teams.insert(team.clone(), players.clone());
    }
    fn clear_teams(&mut self) {
        self.teams.clear();
    }
    pub fn teams(&self) -> Vec<(Team, TeamMembers)> {
        self.teams.clone().into_iter().collect()
    }
    pub fn team_members(&self, team: &Team) -> Option<&TeamMembers> {
        self.teams.get(team)
    }
    pub fn num_humans(&self) -> &u8 {
        &self.num_humans
    }
    pub fn kill_all_players(&self) {
        for player in &self.players {
            crate::kill(player);
        }
    }
    pub fn script_data(&mut self) -> &mut T {
        &mut self.script_data
    }
}

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
            crate::wait_for_external_script(true);
            if let LadderLogEntry::Encoding(ref new_encoding) = entry {
                encoding.clear();
                encoding.push_str(new_encoding);
            }
            callback(entry);
            crate::wait_for_external_script(false);
        }
    }
}

/// Opinionated version of run that automatically tracks the active combatants and number of human players.
/// This function automatically disables allow imposters, as it uses a hashset to keep track of unique players.
pub fn run_suite<T>(
    script_data: T,
    mut callback: impl FnMut(&LadderLogEntry, &mut RuntimeBundle<T>),
    mut command_tree: ChatCommand<T>,
) {
    #[cfg(feature = "ap")]
    crate::intercept_unknown_commands(true);
    let mut data = RuntimeBundle::new(script_data);
    let user_levels = crate::extra::user_levels();
    crate::allow_imposters(false);
    run(|entry| {
        process_suite(&entry, &mut data, &user_levels, &mut command_tree);
        callback(&entry, &mut data);
    });
}

fn process_suite<T>(
    entry: &LadderLogEntry,
    data: &mut RuntimeBundle<T>,
    user_levels: &HashMap<Player, AccessLevel>,
    command_tree: &mut ChatCommand<T>,
) {
    match entry {
        LadderLogEntry::TeamPlayerAdded(team, player) => {
            if *team != Team(String::from("ai_team")) {
                data.add_player(player);
            }
        }
        LadderLogEntry::TeamPlayerRemoved(_, player) => {
            data.remove_player(player);
        }
        LadderLogEntry::PlayerRenamed(old_name, player, _, _) => {
            data.remove_player(&Player(String::from(old_name)));
            data.add_player(player);
        }
        LadderLogEntry::NumHumans(num_humans) => {
            data.num_humans = *num_humans;
        }
        #[cfg(feature = "styct")]
        LadderLogEntry::Command(command, player, args) => {
            let mut args = args.clone();
            args.insert(0, String::from(command));
            command_tree.execute(data, player, user_levels.get(player).copied(), &args[..]);
        }
        #[cfg(feature = "ap")]
        LadderLogEntry::InvalidCommand(command, player, _, access_level, args) => {
            let mut args = args.clone();
            args.insert(0, command.to_string());
            command_tree.execute(data, player, Some(*access_level), &args[..]);
        }
        LadderLogEntry::Positions(team, players) => {
            data.add_team(team, players);
        }
        LadderLogEntry::NewRound(_) => {
            data.clear_teams();
        }
        _ => (),
    }
}
