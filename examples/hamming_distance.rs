/*

- Receives two `&str`s.
- Iterates over them _in parallel_.

With this alg one could do a search for (or count) words that
have distance `0` after comparing them with the reversed versions of them.

Run this example: `cargo`

*/
fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    // these two are iterators
    let ch1 = s1.chars();
    let ch2 = s2.chars();

    let mut distance: usize = 0;

    for (c1, c2) in ch1.zip(ch2) {
        println!("comparing {} / {}", c1, c2);
        if c1 != c2 {
            distance += 1
        }
    }

    Some(distance)
}

fn hamming_distance_alt(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(
        s1.chars()
            .zip(s2.chars())
            .map(|(c1, c2)| if c1 != c2 { 1 } else { 0 })
            .sum(),
    )
}

fn hamming_distance_ints(ints_1: Vec<i32>, ints_2: Vec<i32>) -> Option<usize> {
    let mut distance: usize = 0;

    for (i1, i2) in ints_1.iter().zip(ints_2.iter()) {
        println!("comparing {} / {}", i1, i2);
        // Example XOR bitwise operation
        //
        //     11001000
        //   ^ 10111000
        //     --------
        //   = 01110000
        //
        // https://en.wikipedia.org/wiki/Bitwise_operations_in_C
        if i1 ^ i2 == 0 {
            distance += 1
        }
    }

    Some(distance)
}

fn main() {
    let distance = hamming_distance("juan", "jaun");
    dbg!(distance);
    assert_eq!(distance, Some(2));

    let distance = hamming_distance_alt("juan", "jaun");
    dbg!(distance);
    assert_eq!(distance, Some(2));

    let distance = hamming_distance_ints(vec![1, 2, 3, 4], vec![1, 3, 2, 4]);
    dbg!(distance);
    assert_eq!(distance, Some(2));
}
