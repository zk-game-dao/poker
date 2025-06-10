use candid::{CandidType, Principal};
use errors::chat_error::ChatError;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize as SerdeDeserialize, Serialize};
use std::borrow::Cow;
use std::collections::VecDeque;

#[derive(Clone, Debug, CandidType, Serialize, SerdeDeserialize, PartialEq, Eq)]
pub enum ChatMessageType {
    TableMessage,
    PrivateMessage,
}

#[derive(Clone, Debug, CandidType, Serialize, SerdeDeserialize)]
pub struct ChatMessage {
    pub id: u64,
    pub sender: Principal,
    pub sender_name: String,
    pub content: String,
    pub timestamp: u64,
    pub message_type: ChatMessageType,
    pub recipient: Option<Principal>,
    pub edited: bool,
    pub edit_timestamp: Option<u64>,
}

// Implement Storable for ChatMessage
impl Storable for ChatMessage {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8192, // Maximum size in bytes
        is_fixed_size: false,
    };
}

#[derive(Clone, Debug, CandidType, Serialize, SerdeDeserialize, Default)]
pub struct ChatHistory {
    messages: VecDeque<ChatMessage>,
    next_id: u64,
    capacity: usize,
}

// Implement Storable for ChatHistory
impl Storable for ChatHistory {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8_600_000,
        is_fixed_size: false,
    };
}

impl ChatHistory {
    pub fn new(capacity: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(capacity),
            next_id: 0,
            capacity,
        }
    }

    pub fn add_message(
        &mut self,
        sender: Principal,
        sender_name: String,
        content: String,
        message_type: ChatMessageType,
        recipient: Option<Principal>,
    ) -> u64 {
        // Truncate content if it exceeds the maximum allowed length
        let content = if content.len() > 2000 {
            content.chars().take(2000).collect()
        } else {
            content
        };

        let message = ChatMessage {
            id: self.next_id,
            sender,
            sender_name,
            content,
            timestamp: ic_cdk::api::time(),
            message_type,
            recipient,
            edited: false,
            edit_timestamp: None,
        };

        // Remove oldest message if we're at capacity
        if self.messages.len() >= self.capacity {
            self.messages.pop_front();
        }

        self.messages.push_back(message);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn edit_message(
        &mut self,
        message_id: u64,
        new_content: String,
        editor: Principal,
    ) -> Result<(), ChatError> {
        let message = self.messages.iter_mut().find(|msg| msg.id == message_id);

        match message {
            Some(msg) => {
                // Check if the editor is the original sender
                if msg.sender != editor {
                    return Err(ChatError::Unauthorized {
                        reason: "Only the original sender can edit a message".to_string(),
                    });
                }

                // Truncate content if it exceeds the maximum allowed length
                let new_content = if new_content.len() > 2000 {
                    new_content.chars().take(2000).collect()
                } else {
                    new_content
                };

                // Update the message
                msg.content = new_content;
                msg.edited = true;
                msg.edit_timestamp = Some(ic_cdk::api::time());
                Ok(())
            }
            None => Err(ChatError::MessageNotFound(message_id)),
        }
    }

    pub fn get_messages(&self, from_message_id: Option<u64>, page_size: usize) -> Vec<ChatMessage> {
        match from_message_id {
            Some(id) => {
                // Find index of message with the ID
                if let Some(index) = self.messages.iter().position(|msg| msg.id == id) {
                    // Return messages after this one
                    self.messages
                        .iter()
                        .skip(index + 1)
                        .take(page_size)
                        .cloned()
                        .collect()
                } else {
                    // ID not found, return most recent messages
                    self.messages
                        .iter()
                        .rev()
                        .take(page_size)
                        .cloned()
                        .collect()
                }
            }
            None => {
                // No ID provided, return most recent messages
                self.messages
                    .iter()
                    .rev()
                    .take(page_size)
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn get_messages_for_user(&self, user: Principal) -> Vec<ChatMessage> {
        self.messages
            .iter()
            .filter(|msg| {
                msg.message_type == ChatMessageType::TableMessage
                    || msg.sender == user
                    || msg.recipient == Some(user)
            })
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
