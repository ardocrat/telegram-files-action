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
        let mut message_ids: Vec<MessageId> = vec![];
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
                if pin && !res.is_empty() {
                    let id = res[res.len() - 1].id;
                    self.pin_message(chat_id, id).await?;
                }
                message_ids = res.iter().map(|m| m.id).collect::<Vec<MessageId>>();
            } else {
                // Repost and pin message from 1st chat.
                let res: Vec<MessageId> = self.bot
                    .copy_messages(ChatId(*chat_id), ChatId(chat_ids[0]), message_ids.clone())
                    .send()
                    .await?;
                if pin && !res.is_empty() {
                    let id = res[res.len() - 1];
                    self.pin_message(chat_id, id).await?;
                }
            }
        }
        Ok(())
    }

    async fn pin_message(&self, chat_id: &i64, msg_id: MessageId) -> Result<(), Box<dyn Error>> {
        self.bot
            .pin_chat_message(ChatId(*chat_id), msg_id)
            .send()
            .await?;
        Ok(())
    }
}