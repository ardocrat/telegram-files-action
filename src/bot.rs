use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use teloxide::{net, Bot};
use teloxide::prelude::{Request, Requester};
use teloxide::types::{ChatId, InputFile, InputMedia, InputMediaDocument, Message, MessageId, ParseMode};

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
        let mut msg: Vec<Message> = vec![];
        for (num, chat_id) in chat_ids.iter().enumerate() {
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
                    let id = res[0].id;
                    msg = res;
                    if pin {
                        self.bot
                            .pin_chat_message(ChatId(*chat_id), id)
                            .send()
                            .await?;
                    }
                }
            } else {
                // Repost and pin message from 1st chat.
                if !msg.is_empty() {
                    let ids: Vec<MessageId> = msg.iter().map(|m| m.id).collect::<Vec<MessageId>>();
                    if ids.is_empty() {
                        continue;
                    }
                    let last_id = ids[ids.len() - 1];
                    self.bot
                        .copy_messages(ChatId(*chat_id), ChatId(chat_ids[0]), ids)
                        .send()
                        .await?;
                    if pin {
                        self.bot
                            .pin_chat_message(ChatId(*chat_id), last_id)
                            .send()
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}