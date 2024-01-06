use effbool::EffBool;
use std::sync::mpsc::{channel, Receiver, Sender};

enum Message {
    ExpensiveActionA,
    ExpensiveActionB,
}
fn main() {
    let (sender, receiver) = channel::<Message>();

    //spawn receiver thread.
    std::thread::spawn(move || loop {
        if let Ok(msg) = receiver.recv() {
            match msg {
                Message::ExpensiveActionA => {
                    println!("Do A");
                }
                Message::ExpensiveActionB => {}
            }
        }
    });

    let mut flag = EffBool::default();
    let mut x = 0;
    // sender loop
    loop {
        if x == 4 {
            flag.set(true); //set true
        }
        //send message ONLY when the flag is changed from false to true.
        if flag.is_changed() && flag.get() {
            println!("send");
            sender.send(Message::ExpensiveActionA).unwrap();
        } else {
            println!("do not send");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        x += 1;
    }
}
