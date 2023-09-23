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

struct Progress<I> {
    iter: I,
    i: usize,
}

impl<I> Progress<I> {
    pub fn new(iter: I) -> Self {
        Progress { iter, i: 1 }
    }
}

impl<I: Iterator> Iterator for Progress<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt
where
    Self: Sized,
{
    fn progress(self) -> Progress<Self>;
}

impl<I: Iterator> ProgressIteratorExt for I  {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    thread::sleep(time::Duration::from_secs(1));
}

fn main() {
    println!("{CLEAR}");

    let v = vec![1, 2, 3];

    for n in v.iter().progress() {
        expensive_calculation(n);
    }

    let mut h = collections::HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(3);

    for n in h.iter().progress() {
        expensive_calculation(n);
    }

    println!("All done.");
}
