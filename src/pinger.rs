pub struct Pinger {
    hosts_alive: u8   
}

impl Pinger {
    pub fn new() -> Pinger {
        Pinger  {
            hosts_alive: 0
        }
    }

    pub fn execute_pinging(&self) {
        println!("It Works");
    }
}