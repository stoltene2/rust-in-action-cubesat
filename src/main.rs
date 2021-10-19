#![allow(unused_variables)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(cube_sat: CubeSat) -> CubeSat {
    println!("{:?}", StatusMessage::Ok);
    cube_sat
}

fn main() {
    let sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", sat_a, sat_b, sat_c);
}
