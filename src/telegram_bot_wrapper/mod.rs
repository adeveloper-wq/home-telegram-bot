pub mod commands;
pub mod answers;

use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::dispatching::DispatcherBuilder;
use teloxide::{
    types::{
        ParseMode, 
        InputFile, 
        InputMedia, 
        InputMediaVideo, 
        InputMediaPhoto,
        InlineQueryResult,
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText, 
        InlineQueryResultVideo, 
        InlineQueryResultPhoto,
        InlineKeyboardButton,
        InlineKeyboardMarkup, InlineQueryResultGif
    }, 
    payloads::SendMessageSetters,     
};
use crate::fritzbox_communication_wrapper::FritzboxCommunicationWrapper;
use self::commands::Command;

/// Telegram Bot object.
pub struct TelegramBotWrapper{
    pub fritzbox_communication_wrapper: FritzboxCommunicationWrapper
}

/// Implementation for the Telegram Bot object.
impl TelegramBotWrapper {
    
    /// Get new FritboxCommunicationWrapper object.
    pub async fn new() -> TelegramBotWrapper  {
        // Initiate the fritzbox communication wrapper object.
        let fritzbox_communication_wrapper = FritzboxCommunicationWrapper::new().await;
      
        // Start bot by getting the telegram token from an environment variable.
        let bot = Bot::from_env();

        // Initiate the commands with the corresponding answers.
        // teloxide::commands_repl(bot, self::answer, Command::ty()).await;

        let handler = dptree::entry()
            .branch(Update::filter_message().endpoint(message_handler))
            .branch(Update::filter_inline_query().endpoint(inline_queries_handler))
            .branch(Update::filter_callback_query().endpoint(callback_queries_handler));

        Dispatcher::builder(bot, handler)
        .default_handler(|_| async {})
        .enable_ctrlc_handler()
        .build()
        .dispatch().await;

        // TODO https://github.com/0xNima/Twideo/blob/master/src/main.rs

        // Return the FritzboxCommunicationWrapper object.
        return TelegramBotWrapper{fritzbox_communication_wrapper: fritzbox_communication_wrapper};    
    }

    
}

/* async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::SpeedTest => bot.send_message(msg.chat.id, self.fritzbox_communication_wrapper.get_speed().await).await?,
    };

    Ok(())
} */