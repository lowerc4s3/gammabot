use std::collections::HashSet;
use std::env;

use anyhow::Context;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree::deps;
use teloxide::prelude::*;

use bot::{admin, user};
use teloxide::requests::HasPayload;
use teloxide::types::BotCommandScope;
use teloxide::utils::command::BotCommands;

mod bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()
        .inspect_err(|e| eprintln!("Failed to load .env file: {e}"))
        .ok();
    let token = env::var("BOT_TOKEN").context("no bot token provided")?;
    let admin_id: UserId = UserId(
        env::var("ADMIN_ID")
            .context("no admin id provided")?
            .parse()
            .context("malformed admin id")?,
    );

    let bot = Bot::new(token);
    bot.set_my_commands(admin::Cmd::bot_commands())
        .with_payload_mut(|p| {
            p.scope = Some(BotCommandScope::Chat {
                chat_id: admin_id.into(),
            })
        })
        .await
        .context("failed to set admin cmds")?;
    bot.set_my_commands(user::Cmd::bot_commands())
        .await
        .context("failed to set user cmds")?;

    eprintln!("Bot has started");
    Dispatcher::builder(bot, bot::schema())
        .dependencies(deps![
            InMemStorage::<admin::State>::new(),
            InMemStorage::<user::State>::new(),
            HashSet::<UserId>::from([admin_id])
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
