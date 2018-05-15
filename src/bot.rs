// =======================================================================
//  Copyleft 총통각하 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use futures::Future;
use tokio_core::reactor::{Handle};
use telegram_bot::{Api, Message, ParseMode, MessageKind};
use telegram_bot::prelude::*;

//* Use from local library *//
use conf::{Config, Commands, Command, TelegramConfig};

pub struct TGBot;

impl TGBot {
    fn start(api: Api, message: Message, handle: &Handle) {
        let conf: Commands = unsafe { ::COMMANDS.clone() };
        let commands: Vec<Command> = conf.commands.unwrap();
        let command: Command = commands.into_iter().nth(0).unwrap();

        let html = api.send(message.text_reply(command.reply.unwrap())
            .parse_mode(ParseMode::Html)
        );

        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }
}

pub fn bot(api: Api, message: Message, handle: &Handle) {
    let conf: Config = unsafe { ::CONF.clone() };
    let conf_telegram: TelegramConfig = conf.telegram.unwrap();
    let username = conf_telegram.username.unwrap();

    let function: fn(Api, Message, &Handle) = match message.kind {
        MessageKind::Text {ref data, ..} => {
            let matches: Vec<&str> = data.as_str().matches("/").collect();
            if matches.is_empty() {
                return
            } else {
                let matches: Vec<&str> = data.as_str().matches("@").collect();
                let msg: Vec<&str> = data.split(|c| c == '/' || c == '@' || c == ' ').collect();
                
                if !matches.is_empty() {
                    if msg[2] != username.as_str() {
                        return
                    }
                }

                match msg[1] {
                    "start" => TGBot::start,
                    _ => return,
                }
            }
        }
        _ => return
    };

    function(api, message, handle)
}
