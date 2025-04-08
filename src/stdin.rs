use std::{
    io,
    sync::mpsc::{Receiver, TryRecvError, channel},
    thread,
};

pub struct NonblockingReader {
    channel: Receiver<String>,
}

impl Default for NonblockingReader {
    fn default() -> Self {
        Self::new()
    }
}

impl NonblockingReader {
    pub fn new() -> Self {
        let stdin_channel = {
            let (tx, rx) = channel::<String>();
            thread::spawn(move || {
                loop {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).unwrap();
                    tx.send(buf).unwrap();
                }
            });
            rx
        };
        Self {
            channel: stdin_channel,
        }
    }

    /// Attempt to read a line from stdin.
    /// Returns None if a line cannot be read.
    /// Panics if the channel is closed.
    pub fn readline(&self) -> Option<String> {
        match self.channel.try_recv() {
            Ok(s) => Some(s),
            Err(TryRecvError::Empty) => None,
            _ => panic!("channel disconnected"),
        }
    }
}
