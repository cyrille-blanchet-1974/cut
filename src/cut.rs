use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_cut_char(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    start: usize,
    end: usize,
) -> JoinHandle<()> {
    spawn(move || {
        for l in from_read {
            if l.len() < start {
                continue;
            }
            let (a, _b) = l.split_at(end);
            let (_c, d) = a.split_at(start);
            if to_write.send(d.to_string()).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}

pub fn start_thread_cut_field(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    field: usize,
    delimit: char,
) -> JoinHandle<()> {
    spawn(move || {
        for l in from_read {
            let v: Vec<&str> = l.split(delimit).collect();
            if to_write.send(v.get(field).unwrap().to_string()).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}
