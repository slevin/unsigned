/*
nicer than c or d in that it wouldn't straight assign
a uint to an f64 so I got to see a warning with the right type
but still can cast and thusly get the wrong value
its dynamic typing with ints and Unitas but not floats
*/
fn main() {
    let count = 1u;
    let secondsAgo: f64 = (60 * 60 * 24 * count * -1) as f64;
    println!("seconds ago:{:?}", secondsAgo);
}
