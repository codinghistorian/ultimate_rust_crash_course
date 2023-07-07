// const missiles:i32 = 9;
// const ready:i32 = 2;

// let (x, y, z) = (10, 20, 30);

fn main() {
    // let mut missiles = 8;
    // let ready = 2;
    let (mut missiles, mut _ready, mut _no_use): (i32, i32, i32)  = (9, 2, 8);
    missiles = 11;
    println!("Firing {} of my {} missiles...", _ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", (missiles - _ready));
    const STARTING_MISSILES:i32 = 8;
    const READY_AMOUNT:i32 = 2;
}
