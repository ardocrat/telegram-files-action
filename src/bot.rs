use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use teloxide::{net, Bot};
use teloxide::prelude::{Request, Requester, ResponseResult};
use teloxide::types::{ChatId, InputFile, InputMedia, InputMediaDocument, Message, MessageId, ParseMode};
use tokio::time::sleep;
use log::info;

pub struct TelegramBot {
    pub bot: Bot,
    delay: Duration,
}

impl TelegramBot {
    pub fn new(token: String, url: String, delay: Duration) -> Result<TelegramBot, Box<dyn Error>> {
        let client = net::default_reqwest_settings()
            .timeout(Duration::from_mins(60)).build()?;
        Ok(TelegramBot {
            bot: Bot::with_client(token, client).set_api_url(reqwest::Url::parse(url.as_str())?),
            delay,
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
            let mut sent_msg_id: Option<MessageId> = None;
            // Upload files to 1st chat.
            if num == 0 {
                while sent_msg_id.is_none() {
                    let media: Vec<InputMedia> = files.iter().enumerate().map(|(index, path)| {
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
                    if let Ok(res) = self.bot
                        .send_media_group(ChatId(chat_id.clone()), media)
                        .send()
                        .await {
                        if !res.is_empty() {
                            let id = res[res.len() - 1].id;
                            sent_msg_id = Some(id);
                            message_ids = res.iter().map(|m| m.id).collect::<Vec<MessageId>>();
                            info!("Sent message {:?} to {:?}", id, chat_id);
                        }
                        break;
                    } else {
                        info!("Error sending message to {:?}, trying again...", chat_id);
                    }
                }
                if !pin {
                    continue;
                }
            } else {
                // Repost message from 1st chat.
                while sent_msg_id.is_none() {
                    sleep(self.delay).await;
                    if let Ok(res) = self.bot
                        .copy_messages(ChatId(*chat_id), ChatId(chat_ids[0]), message_ids.clone())
                        .send()
                        .await {
                        if !res.is_empty() {
                            let id = res[res.len() - 1];
                            sent_msg_id = Some(id);
                            info!("Sent message {:?} to {:?}", id, chat_id);
                        }
                        break;
                    } else {
                        info!("Error sending message to {:?}, trying again...", chat_id);
                    }
                }
            }
            // Pin message.
            if let Some(id) = sent_msg_id {
                loop {
                    sleep(self.delay).await;
                    if self.bot.pin_chat_message(ChatId(*chat_id), id).send().await.is_ok() {
                        info!("Pinned message {:?} to {:?}", id, chat_id);
                        break;
                    } else {
                        info!("Error pin message {:?} to {:?}, trying again...", id, chat_id);
                    }
                }
            }
        }
        Ok(())
    }
}