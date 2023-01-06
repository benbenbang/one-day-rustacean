#![allow(dead_code, unused_variables)]

use ding_machine::{ding, on_off, print_array, print_difference, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr: [f32; 2] = [6.3, 15.0];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding(series[series.len() - 1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // produce a output "Lights are on!" or "Lights are off!"
    on_off(mess.2[0].0);
    on_off(mess.2[1].0);

    print_distance(coords);
}
