use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct Pinger {
    hosts_alive: u8,
    hosts: Vec<String>,
}

impl Pinger {
    pub fn new() -> Pinger {
        Pinger {
            hosts_alive: 0,
            hosts: Vec::new(),
        }
    }

    pub fn add_host(&mut self, host_address: String) {
        self.hosts.push(host_address);
    }

    pub fn execute_pinging(&self) {
        // let hosts = self.hosts;

        for h in hosts {
            
            thread::spawn(move || {
            // let host = hosts.get(i).unwrap();
            println!("tji");
            let mut ping_command = Command::new("sh");
            ping_command
                .arg("-c")
                .arg("ping ".to_owned() + h + " -t 1");
            let ping = ping_command.output().expect("failed to execute process");

            let mut raw_ping_output = String::from_utf8(ping.stdout).unwrap();

            let split_index = raw_ping_output.find("packets received").unwrap() - 2;

            // Get amount of received packets, to check if host is alive.
            let received_packets = raw_ping_output.split_off(split_index).remove(0).to_string();
            let received_packets: u32 = received_packets.parse().unwrap();

            if received_packets >= 1 {
                println!("HOST is up");
            } else {
                println!("HOST is down");
            }
                thread::sleep(Duration::from_millis(1))
            });
        }
    }
}
