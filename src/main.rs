use std::process::Command;

use pinger::Pinger;
mod pinger;

fn main() {

    let pinger = Pinger::new();

    pinger.execute_pinging();

    // let host = "172.17.80.159";

    // let mut ping_command = Command::new("sh");
    // ping_command.arg("-c").arg("ping ".to_owned() + host + " -t 1");
    // let ping = ping_command.output().expect("failed to execute process");

    // let mut raw_ping_output = String::from_utf8(ping.stdout).unwrap();

    // let split_index = raw_ping_output.find("packets received").unwrap() - 2;

    // // Get amount of received packets, to check if host is alive.
    // let received_packets = raw_ping_output.split_off(split_index).remove(0).to_string();
    // let received_packets: u32 = received_packets.parse().unwrap();

    // if received_packets >= 1 {
    //     println!("fantastisch");
    // } else {
    //     println!("ne");
    // }
}
