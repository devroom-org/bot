// =======================================================================
//  Copyleft 총통각하 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

#![feature(const_fn)]

#[macro_use]
extern crate serde_derive;

extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;
extern crate toml;

mod conf;
mod bot;

//* Use from external library *//
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::{Api, UpdateKind};

//* Use from local library *//
use conf::{read_conf, read_commands};
use conf::{Config, Commands, TelegramConfig};
use bot::bot;

pub static mut CONF: Config = Config::default();
pub static mut COMMANDS: Commands = Commands::default();

fn main() {
    unsafe { CONF = read_conf("bot.toml").unwrap() };
    unsafe { COMMANDS = read_commands("commands.toml").unwrap() };
    let conf: Config = unsafe { CONF.clone() };
    let conf_telegram: TelegramConfig = conf.telegram.unwrap();
    let token = conf_telegram.bot_api.unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            bot(api.clone(), message, &handle)
        }
        Ok(())
    });

    core.run(future).unwrap();
}
