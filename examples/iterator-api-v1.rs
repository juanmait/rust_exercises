// "Type-Driven API Design in Rust" by Will Crichton
//
// - https://youtu.be/bnnacleqg6k?si=vJ7iCxAhMotaCvBZ
//
// ```sh
// # Run it
// cargo run --example iterators
//
// # Watch it
// cargo watch -q -c -w examples/ -x 'run --example iterator-api-v1'
// ```
use std::{thread, time, collections};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_calculation(_n: &i32) {
    thread::sleep(time::Duration::from_secs(1));
}

// > "f is a function from pointers to `T` to nothing"
fn progress<Iter: Iterator>(iter: Iter, f: fn(Iter::Item) -> ()) {
    let mut i = 1;

    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n);
    }
}

fn main() {
    println!("{CLEAR}");

    let v = vec![1, 2, 3];
    progress(v.iter(), expensive_calculation);

    let mut h = collections::HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(3);

    progress(h.iter(), expensive_calculation);

    println!("All done.");
}
