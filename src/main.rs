use pinger::Pinger;
mod pinger;

fn main() {
    let mut pinger = Pinger::new();

    pinger.add_host("google.com".to_string());
    pinger.add_host("huggler.me".to_string());
    pinger.add_host("10.0.0.2".to_string());


    pinger.execute_pinging();
}
