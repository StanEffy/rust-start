fn main() {
    let mut missiles: u8 = 8;
    let ready: u8 = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    
    println!("Firing {} of my {} missiles...", READY_AMOUNT, STARTING_MISSILES);
}    

    const STARTING_MISSILES: u128 = 8;
    const READY_AMOUNT: u32 = 2;
    