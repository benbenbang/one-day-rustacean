// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.
//
trait Bite {
    fn bite(self: &mut Self) {}
}

// Create a struct named Grapes with a field that tracks how many grapes are left.
//
#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

// Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
//
impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1
    }
}

fn bunny_nibbles<T: Bite>(food: &mut T) {
    food.bite()
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // generic
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);

    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
