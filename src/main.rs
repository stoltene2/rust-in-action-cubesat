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

    fn receive(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
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

impl Mailbox {
    /// Adds messages to the outgoing msg queue for sending to `CubeSat`s
    fn post(&mut self, msg: Message) -> () {
        self.messages.push(msg)
    }

    /// Deliver messages to the `CubeSat`s
    fn deliver(&mut self, cube_sat: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if cube_sat.id == self.messages[i].to {
                return Some(self.messages.remove(i));
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    conent: String,
}

struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg)
    }
}

fn check_status(cube_sat: CubeSat) -> CubeSat {
    println!("{:?}", StatusMessage::Ok);
    cube_sat
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let gs = GroundStation {};

    let mailbox = &mut Mailbox {
        messages: Vec::new(),
    };

    let sats: Vec<CubeSat> = fetch_sat_ids()
        .iter()
        .map(|sat_id| -> CubeSat { gs.connect(*sat_id) })
        .collect();

    for sat in sats {
        let msg = Message {
            to: sat.id,
            conent: String::from("Hello sat"),
        };
        gs.send(mailbox, msg);
        let msg = sat.receive(mailbox).unwrap();
        println!("msg: {:?}", msg);
    }
}
