pub use crate::raw::types::Message;
use crate::types::MessageEntityType;

impl Message {
    #[inline]
    pub fn bot_command<'a>(&'a self, command: &str) -> Option<&'a str> {
        self.bot_commands().and_then(|mut bot_commands| {
            bot_commands.find(|bot_command| {
                *bot_command == command || bot_command.starts_with(&format!("{}@", command))
            })
        })
    }

    #[inline]
    pub fn bot_commands(&self) -> Option<impl Iterator<Item = &str>> {
        self.entities.as_ref().map(|entities| {
            entities
                .iter()
                .filter(|entity| MessageEntityType::BotCommand == entity.type_)
                .map(move |entity| unsafe { entity.unsafe_text(self.text.as_ref().unwrap()) })
        })
    }
}
