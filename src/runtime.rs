use std::collections::HashSet;

use crate::{model::ladderlog::*, *};

pub struct RuntimeBundle {
    players: HashSet<Player>,
    num_humans: u8,
}

impl RuntimeBundle {
    fn new() -> RuntimeBundle {
        RuntimeBundle {
            players: HashSet::new(),
            num_humans: 0,
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
    pub fn num_humans(&self) -> &u8 {
        &self.num_humans
    }
    pub fn kill_all(&self) {
        for player in &self.players {
            crate::kill(player);
        }
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

/// Opinionated version of run that automatically tracks the active combatants and number of human players.
/// This function automatically disables allow imposters, as it uses a hashset to keep track of unique players.
pub fn run_suite(mut callback: impl FnMut(&LadderLogEntry, &RuntimeBundle)) {
    let mut data = RuntimeBundle::new();
    allow_imposters(false);
    run(|entry| {
        process_suite(&entry, &mut data);
        callback(&entry, &data);
    });
}

fn process_suite(entry: &LadderLogEntry, data: &mut RuntimeBundle) {
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
        _ => (),
    }
}
