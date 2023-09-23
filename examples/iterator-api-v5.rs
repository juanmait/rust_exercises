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
// ```¿
use std::{collections, thread, time};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delimiters: (char, char),
}

struct Progress<I, B> {
    iter: I,
    i: usize,
    bound: B,
}

trait ProgressDisplay
where
    Self: Sized,
{
    fn display<I>(&self, progress: &Progress<I, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<I>(&self, progress: &Progress<I, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<I>(&self, progress: &Progress<I, Self>) {
        println!(
            "{}{}{}{}",
            self.delimiters.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delimiters.1
        );
    }
}

/// initially Progress will be "Unbounded"
impl<I> Progress<I, Unbounded> {
    pub fn new(iter: I) -> Self {
        Progress {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<I: ExactSizeIterator> Progress<I, Unbounded> {
    pub fn with_bound(self) -> Progress<I, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delimiters: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<I: ExactSizeIterator> Progress<I, Bounded> {
    pub fn with_delimiters(mut self, delimiters: (char, char)) -> Self {
        self.bound.delimiters = delimiters;
        self
    }
}

impl<I: Iterator, B: ProgressDisplay> Iterator for Progress<I, B> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);

        let next = self.iter.next();

        if next.is_some() {
            self.i += 1;
        }

        self.bound.display(self);
        next
    }
}

trait ProgressIteratorExt
where
    Self: Sized,
{
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<I: Iterator> ProgressIteratorExt for I {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    thread::sleep(time::Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];

    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }

    let mut h = collections::HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(2);

    for n in h.iter().progress().with_bound().with_delimiters(('<', '>')) {
        expensive_calculation(n);
    }

    let unbounded = (0..).into_iter();

    for n in unbounded.progress() {
        expensive_calculation(&n);
    }

    println!("All done.");
}
