use anyhow::Result;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{HandlerExt, UpdateFilterExt, UpdateHandler};
use teloxide::macros::BotCommands;
use teloxide::prelude::*;

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Cmd {
    #[command(description = "получить ссылку на подписку")]
    Sub,
}

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Start,
    PendingInvoice,
}

type ChatState = Dialogue<State, InMemStorage<State>>;

pub fn schema() -> UpdateHandler<anyhow::Error> {
    use dptree::case;
    Update::filter_message()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(
            teloxide::filter_command::<Cmd, _>()
                .chain(case![State::Start])
                .branch(case![Cmd::Sub].endpoint(get_subscription)),
        )
        .branch(case![State::PendingInvoice].endpoint(get_payment))
}

async fn get_subscription(bot: Bot, state: ChatState, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "get_subscription").await?;
    Ok(())
}

async fn get_payment(bot: Bot, state: ChatState, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "get_payment").await?;
    Ok(())
}
