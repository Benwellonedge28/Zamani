#![allow(unused_imports, dead_code, unused_variables)]

//! Zamani — agent_communication module
//! Facilitates message passing and knowledge sharing between AGI agents.

use std::collections::{HashMap, VecDeque};

/// Initialize agent_communication
pub fn init_agent_communication() {
    println!("[StdLib::Agents] Initializing Agent Communication Fabric...");
}

/// Shutdown agent_communication
pub fn shutdown_agent_communication() {
    println!("[StdLib::Agents] Shutting down Agent Communication Fabric...");
}

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub payload: String,
    pub priority: u8,
}

pub struct CommunicationFabric {
    pub mailboxes: HashMap<String, VecDeque<Message>>,
}

impl CommunicationFabric {
    pub fn new() -> Self {
        CommunicationFabric {
            mailboxes: HashMap::new(),
        }
    }

    pub fn send_message(&mut self, msg: Message) {
        self.mailboxes
            .entry(msg.receiver.clone())
            .or_insert_with(VecDeque::new)
            .push_back(msg);
    }

    pub fn receive_message(&mut self, agent_id: &str) -> Option<Message> {
        self.mailboxes.get_mut(agent_id)?.pop_front()
    }
}
