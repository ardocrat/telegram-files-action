use action::Action;
mod action;

use crate::bot::TelegramBot;
mod bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action = Action::new()?;
    let bot = TelegramBot::new(action.token, action.api_url)?;
    bot.send(action.message, action.files, action.chat_ids, action.pin).await?;
    Ok(())
}