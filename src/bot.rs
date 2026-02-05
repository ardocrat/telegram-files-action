use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use teloxide::{net, Bot};
use teloxide::prelude::{Request, Requester};
use teloxide::types::{ChatId, InputFile, InputMedia, InputMediaDocument, Message, ParseMode};

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
        let mut msg: Option<Message> = None;
        for (num, chat_id) in chat_ids.iter().enumerate() {
            // Repost and pin message from 1st chat.
            if let Some(m) = &msg {
                self.bot
                    .copy_message(ChatId(*chat_id), ChatId(chat_ids[0]), m.id)
                    .send()
                    .await?;
                if pin {
                    self.bot
                        .pin_chat_message(ChatId(*chat_id), m.id)
                        .send()
                        .await?;
                }
            }

            // Upload files to 1st chat.
            if num == 0 {
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
                let res: Vec<Message> = self.bot
                    .send_media_group(ChatId(chat_id.clone()), input_media)
                    .send()
                    .await?;
                if !res.is_empty() {
                    let m = res[0].clone();
                    let id = m.id;
                    msg = Some(m);
                    if pin {
                        self.bot
                            .pin_chat_message(ChatId(*chat_id), id)
                            .send()
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}