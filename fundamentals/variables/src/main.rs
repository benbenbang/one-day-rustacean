const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    part1();
    part2();
    extra();

    // error[E0070]: invalid left-hand side of assignment
    // READY_AMOUNT = 1;
}

fn part1() {
    let missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);
}

fn part2() {
    let mut missiles = 8;
    let ready = 2;

    missiles -= ready;

    println!("{} missiles left", missiles);

    let missiles = STARTING_MISSLES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);
}

fn extra() {
    // no need for mut if we want to use one liner
    // let (mut mussiles, ready)...
    let (missiles, ready): (i32, i32) = (8, 2);

    println!("{}", missiles - ready);
}
