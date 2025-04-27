use std::{
    io::{self, BufRead},
    sync::mpsc::{Receiver, Sender, TryRecvError, channel},
    thread,
};

pub struct NonblockingReader {
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl Default for NonblockingReader {
    fn default() -> Self {
        Self::new()
    }
}

impl NonblockingReader {
    pub fn new() -> Self {
        let (tx, rx) = {
            let (tx, ext_rx) = channel::<String>();
            let (ext_tx, rx) = channel::<String>();
            thread::spawn(move || {
                let mut buf = String::new();
                loop {
                    // let mut buf = String::new();
                    // io::stdin().read_line(&mut buf).unwrap();
                    // tx.send(buf).unwrap();
                    if let Ok(s) = rx.try_recv() {
                        buf = s;
                    }

                    let mut stdin = io::stdin().lock();
                    let buffer = stdin.fill_buf().unwrap();
                    let len = buffer.len();
                    buf += String::from_utf8(Vec::from(buffer)).unwrap().as_str();
                    stdin.consume(len);

                    if len > 0 {
                        println!("Updated: {buf:?}");
                    }

                    if let Some(nl) = buf.find("\n") {
                        tx.send(buf[..nl].to_string()).unwrap();
                        if buf[nl..].len() > 1 {
                            buf = buf[nl..].to_string();
                        } else {
                            buf = String::new();
                        }
                    }
                }
            });
            (ext_tx, ext_rx)
        };
        Self { tx, rx }
    }

    /// Attempt to read a line from stdin.
    /// Returns None if a line cannot be read.
    /// Panics if the channel is closed.
    pub fn readline(&self) -> Option<String> {
        match self.rx.try_recv() {
            Ok(s) => Some(s.replace("[A", "").replace("[B", "")),
            Err(TryRecvError::Empty) => None,
            _ => panic!("channel disconnected"),
        }
    }

    pub fn set_contents(&self, contents: String) {
        self.tx.send(contents).unwrap();
    }
}
