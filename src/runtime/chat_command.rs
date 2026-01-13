use std::{collections::HashMap, fmt::Debug};

use crate::{
    model::{AccessLevel, Player},
    runtime::RuntimeBundle,
};

type Callback<'a, T> =
    dyn FnMut(&mut RuntimeBundle<T>, &Player, Option<AccessLevel>, &[String]) + 'a;

/// Raw command can be consumed by ChatCommand::from_raw to produce a tree of chat commands automatically
pub struct RawCommand<'a, T> {
    path: String,
    access_level: Option<AccessLevel>,
    closure: Option<Box<Callback<'a, T>>>,
    help_message: String,
}

impl<'a, T> RawCommand<'a, T> {
    pub fn new(
        path: &str,
        access_level: Option<AccessLevel>,
        closure: impl FnMut(&mut RuntimeBundle<T>, &Player, Option<AccessLevel>, &[String]) + 'a,
        help_message: &str,
    ) -> RawCommand<'a, T> {
        RawCommand {
            path: String::from(path),
            access_level,
            closure: Some(Box::new(closure)),
            help_message: String::from(help_message),
        }
    }
    /// Category commands don't do anything when called, but they do populate the help command!
    pub fn category(path: &str, help_message: &str) -> RawCommand<'a, T> {
        RawCommand {
            path: String::from(path),
            access_level: None,
            closure: None,
            help_message: String::from(help_message),
        }
    }
}

pub struct ChatCommand<'a, T> {
    subcommands: HashMap<String, ChatCommand<'a, T>>,
    access_level: Option<AccessLevel>,
    closure: Option<Box<Callback<'a, T>>>,
}

impl<'a, T> Default for ChatCommand<'a, T> {
    fn default() -> Self {
        Self {
            subcommands: Default::default(),
            access_level: Default::default(),
            closure: Default::default(),
        }
    }
}

impl<'a, T> Debug for ChatCommand<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChatCommand")
            .field("subcommands", &self.subcommands)
            .field("access_level", &self.access_level)
            .finish_non_exhaustive()
    }
}

impl<'a, T> ChatCommand<'a, T> {
    pub fn execute(
        &mut self,
        runtime_bundle: &mut RuntimeBundle<T>,
        player: &Player,
        access_level: Option<AccessLevel>,
        arguments: &[String],
    ) {
        if access_level.unwrap_or(AccessLevel::Program)
            <= *self.access_level.as_ref().unwrap_or(&AccessLevel::Program)
        {
            if let Some(subcommand_name) = arguments.get(0)
                && let Some(subcommand) = self.subcommands.get_mut(subcommand_name)
            {
                subcommand.execute(
                    runtime_bundle,
                    player,
                    access_level,
                    if arguments.len() > 0 {
                        &arguments[1..]
                    } else {
                        &[]
                    },
                );
            } else {
                if let Some(ref mut closure) = self.closure {
                    closure(runtime_bundle, player, access_level, arguments);
                }
            }
        } else {
            crate::player_message(player, "You are not authorized for that command.");
        }
    }
    pub fn from_raw(
        root_path: &'a str,
        raw_commands: Vec<RawCommand<'a, T>>,
    ) -> ChatCommand<'a, T> {
        let mut root_command = ChatCommand::default();
        let mut root_help = ChatCommand::default();
        let mut help_message = vec![];
        for raw_command in raw_commands {
            let mut command = &mut root_command;
            let mut help = &mut root_help;
            for segment in format!("{} {}", root_path, raw_command.path).split_whitespace() {
                command = command
                    .subcommands
                    .entry(segment.to_string())
                    .or_insert_with(|| ChatCommand::default());
            }
            for segment in raw_command.path.split_whitespace() {
                help = help
                    .subcommands
                    .entry(segment.to_string())
                    .or_insert_with(|| ChatCommand::default())
            }
            command.access_level = raw_command.access_level;
            command.closure = raw_command.closure;
            help_message.push(format!("/{} {}", root_path, raw_command.path,));
            help.closure = Some(Box::new(move |_, player, _, _| {
                crate::player_message(
                    player,
                    &format!(
                        "/{} {} | {}",
                        root_path, raw_command.path, raw_command.help_message
                    ),
                )
            }));
        }
        help_message.sort();
        help_message.push(format!("For details, use /{} help <item>", root_path));
        let help_message = help_message.join("\\n");
        root_help.closure = Some(Box::new(move |_, player, _, _| {
            crate::player_message(player, &help_message);
        }));
        root_command
            .subcommands
            .entry(String::from(root_path))
            .or_insert(ChatCommand::default())
            .subcommands
            .insert(String::from("help"), root_help);
        root_command
    }
}

#[test]
fn chat_command_tree_simple() {
    let mut x = ChatCommand::from_raw(
        "mode_select",
        vec![
            RawCommand::category("mode_unlock", "Usage: \"/mode_unlock auto|vote\""),
            RawCommand::new(
                "mode_unlock auto",
                Some(AccessLevel::Owner),
                |_, _, _, _| {},
                "Automatically change modes based on player count",
            ),
            RawCommand::new(
                "mode_unlock vote",
                Some(AccessLevel::Administrator),
                |_, _, _, _| {},
                "Automatically change modes based on player preference",
            ),
            RawCommand::new(
                "mode_vote",
                None,
                |_, _, _, _| {},
                "Preference is remembered until a server restart. Usage: \"/mode_vote dm|tdm|sumobar|fort\"",
            ),
        ],
    );
    println!("{:#?}", x);
    x.execute(
        &mut RuntimeBundle::new(()),
        &Player(String::from("Anyone")),
        None,
        &["mode_select".to_string(), "help".to_string()],
    );
    x.execute(
        &mut RuntimeBundle::new(()),
        &Player(String::from("Anyone")),
        None,
        &[
            "mode_select".to_string(),
            "help".to_string(),
            "mode_vote".to_string(),
        ],
    );
    assert!(false);
}
