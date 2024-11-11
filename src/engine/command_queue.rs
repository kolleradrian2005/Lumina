use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use super::gl_command::GlCommand;

#[derive(Debug)]
pub struct CommandQueue {
    sender: Sender<GlCommand>,
    receiver: Arc<Mutex<Receiver<GlCommand>>>,
}

impl CommandQueue {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        CommandQueue {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn get_sender(&self) -> Sender<GlCommand> {
        self.sender.clone()
    }

    pub fn process_commands(&self) {
        let receiver = self.receiver.lock().unwrap();
        while let Ok(command) = receiver.try_recv() {
            command.execute();
        }
    }
}
