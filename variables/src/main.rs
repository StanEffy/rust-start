fn main() {
    let mut missiles: u8 = 8;
    let ready: u8 = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
