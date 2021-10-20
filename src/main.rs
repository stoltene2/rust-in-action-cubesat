#![allow(unused_variables)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id,
            mailbox: Mailbox { messages: vec![] },
        }
    }

    fn receive(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn check_status(cube_sat: CubeSat) -> CubeSat {
    println!("{:?}", StatusMessage::Ok);
    cube_sat
}

fn main() {
    let gs = GroundStation {};

    let mut sat_a = CubeSat::new(0);

    println!("t0: {:?}", sat_a);
    gs.send(&mut sat_a, Message::from("Hello from Earth"));

    println!("t1: {:?}", sat_a);
    let msg = sat_a.receive();

    println!("t2: {:?}", sat_a);
    println!("msg: {:?}", msg);
}
