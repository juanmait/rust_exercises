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
use std::{collections, thread, time};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 1 }
    }
}

impl<Iter: Iterator> Iterator for Progress<Iter> {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

fn expensive_calculation(_n: &i32) {
    thread::sleep(time::Duration::from_secs(1));
}

fn main() {
    println!("{CLEAR}");

    let v = vec![1, 2, 3];

    for n in Progress::new(v.iter()) {
        expensive_calculation(n);
    }

    let mut h = collections::HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(3);

    for n in Progress::new(h.iter()) {
        expensive_calculation(n);
    }

    println!("All done.");
}
