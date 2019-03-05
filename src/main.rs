use pinger::Pinger;
mod pinger;

fn main() {
    let mut pinger = Pinger::new();

    pinger.add_host("192.168.1.10".to_string());
    pinger.add_host("192.168.1.4".to_string());
    pinger.add_host("192.168.1.20".to_string());


    pinger.execute_pinging();
}
