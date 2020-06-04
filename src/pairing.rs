use crossbeam_channel::{select, Receiver, Sender};
use crate::Choice;
use crate::Answer;

pub fn pairing_loop(event_reciever: Receiver<Choice>, pairing_sender: Sender<Answer>) {
    std::thread::spawn(move || {
        let mut pair = 0;
        loop {
            select! {
                recv(event_reciever) -> peer => {
                    pair += 1;
                    if pair % 2 == 0  {
                        println!("{:?} received", peer.unwrap());
                    } else {
                        println!("Send pair: {:?}", pairing_sender.send(Answer::Ok));
                    }
                },
            }
        }
    });
}
