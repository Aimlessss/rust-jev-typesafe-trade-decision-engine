use crate::Order;
use std::sync::mpsc::sync_channel;
use std::thread;

pub(crate) enum Command {
    AddOrder(Order),
    CancelOrder(u64),
}

fn main(){
    let (sender, receiver) = sync_channel::<Command>(100);
    let handle = thread::spawn(move || {
        let command = receiver.recv().unwrap();

        match command {
            Command::AddOrder(order) => {
                println!("added order {}", order.id);
            }
            Command::CancelOrder(id) => {
                println!("canc order {}", id);
            }
        }
    });
}