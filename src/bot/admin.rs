use anyhow::Result;
use rust_decimal::Decimal;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{UpdateFilterExt, UpdateHandler};
use teloxide::macros::BotCommands;
use teloxide::prelude::*;

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Cmd {
    #[command(description = "создать новый счет")]
    Invoice,
    #[command(description = "добавить нового пользователя")]
    AddUser,
}

#[derive(Debug, Default, Clone)]
pub enum State {
    #[default]
    Start,
    CreateInvoice {
        price: Decimal,
    },
    NewUser {
        id: UserId,
    },
    AcceptInvoice, // TODO: Accept image as param or path (and mb sender id)
}

type ChatState = Dialogue<State, InMemStorage<State>>;

pub fn schema() -> UpdateHandler<anyhow::Error> {
    use dptree::case;
    Update::filter_message()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(
            case![State::Start]
                .filter_command::<Cmd>()
                .branch(case![Cmd::Invoice].endpoint(get_invoice))
                .branch(case![Cmd::AddUser].endpoint(get_new_user)),
        )
        .branch(case![State::CreateInvoice { price }].endpoint(create_invoice))
        .branch(case![State::NewUser { id }].endpoint(add_new_user))
}

async fn get_invoice(bot: Bot, state: ChatState, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "get_invoice").await?;
    Ok(())
}

async fn get_new_user(bot: Bot, state: ChatState, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "get_new_user").await?;
    Ok(())
}

async fn create_invoice(bot: Bot, state: ChatState, msg: Message, price: Decimal) -> Result<()> {
    bot.send_message(msg.chat.id, "create_invoice").await?;
    Ok(())
}

async fn add_new_user(bot: Bot, state: ChatState, msg: Message, id: UserId) -> Result<()> {
    bot.send_message(msg.chat.id, "add_new_user").await?;
    Ok(())
}
