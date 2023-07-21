const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // test mutating constant
    // READY_AMOUNT = 1;
    
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    // unused test
    // let taco = "taco";
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    println!("{} missiles left", missiles - ready)
}
