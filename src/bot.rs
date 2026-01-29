use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use teloxide::{net, Bot};
use teloxide::prelude::{Request, Requester};
use teloxide::types::{ChatId, InputFile, InputMedia, InputMediaDocument, ParseMode};

pub struct TelegramBot {
    pub bot: Bot,
}

impl TelegramBot {
    pub fn new(token: String, url: String) -> Result<TelegramBot, Box<dyn Error>> {
        let client = net::default_reqwest_settings()
            .timeout(Duration::from_mins(60)).build()?;
        Ok(TelegramBot {
            bot: Bot::with_client(token, client).set_api_url(reqwest::Url::parse(url.as_str())?),
        })
    }

    pub async fn send(
        &self,
        message: String,
        files: Vec<PathBuf>,
        chat_ids: Vec<i64>,
        pin: bool,
    ) -> Result<(), Box<dyn Error>> {
        for chat_id in &chat_ids {
            let input_media: Vec<InputMedia> = files.iter().enumerate().map(|(index, path)| {
                if index == files.len() - 1 {
                    InputMedia::Document(
                        InputMediaDocument::new(InputFile::file(path))
                            .caption(message.as_str())
                            .parse_mode(ParseMode::Html)
                    )
                } else {
                    InputMedia::Document(
                        InputMediaDocument::new(InputFile::file(path))
                    )
                }
            }).collect();
            let res = self.bot.send_media_group(ChatId(chat_id.clone()), input_media).send().await?;
            // Pin message if non-empty.
            if !res.is_empty() {
                let message_id = res.get(res.len() - 1).unwrap().id;
                if pin {
                    self.bot.pin_chat_message(ChatId(*chat_id), message_id).send().await?;
                }
            }
        }

        Ok(())
    }
}