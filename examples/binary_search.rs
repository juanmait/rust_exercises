/// Would this only work against an array of unique ordered values?
/// In a scenario where the array is ordered but items are not unique
/// when a key is found we can't guarantee that is the first, last, or
/// mid occurrence of it.
fn find(arr: &[i32], key: i32) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }

    let mut l = 0;
    let mut r: usize = arr.len();

    while l < r {
        let mid_idx = (l + r) / 2;

        let mid_val = arr[mid_idx];

        if mid_val == key {
            return Some(mid_idx);
        }

        if mid_val < key {
            l = (mid_idx + 1) as usize;
        } else {
            r = (mid_idx) as usize;
        }
    }

    None
}

/// implementation using `match` statement
fn find_using_match(arr: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r: usize = arr.len();

    while l < r {
        let mid_idx = (l + r) / 2;

        match arr[mid_idx].cmp(&key) {
            std::cmp::Ordering::Less => l = (mid_idx + 1) as usize,
            std::cmp::Ordering::Equal => return Some(mid_idx),
            std::cmp::Ordering::Greater => r = (mid_idx) as usize,
        }
    }

    None
}

/// -> https://youtu.be/j0HSGETK0sI?t=364
fn find_generic<T: Ord, V: AsRef<[T]>>(arr: V, key: T) -> Option<usize> {
    let arr = arr.as_ref();
    let mut l = 0;
    let mut r: usize = arr.len();

    while l < r {
        let mid_idx = (l + r) / 2;

        match arr[mid_idx].cmp(&key) {
            std::cmp::Ordering::Less => l = (mid_idx + 1) as usize,
            std::cmp::Ordering::Equal => return Some(mid_idx),
            std::cmp::Ordering::Greater => r = (mid_idx) as usize,
        }
    }

    None
}

fn test_regular_find() {
    println!("Testing `find()`");

    let arr = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    let arr2 = &[1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(find(arr, 3), Some(3));
    assert_eq!(arr[3], 3);

    assert_eq!(find(arr, 8), Some(8));
    assert_eq!(arr[8], 8);

    assert_eq!(find(arr2, 3), Some(2));
    assert_eq!(arr2[2], 3);

    assert_eq!(find(arr2, 8), Some(7));
    assert_eq!(arr2[7], 8);

    // NOT FOUND
    assert_eq!(find(arr, 9), None);
    assert_eq!(find(arr, -1), None);
}

fn test_using_match() {
    println!("Testing `find_using_match()`");

    let arr = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    let arr2 = &[1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(find_using_match(arr, 3), Some(3));
    assert_eq!(arr[3], 3);

    assert_eq!(find_using_match(arr, 8), Some(8));
    assert_eq!(arr[8], 8);

    assert_eq!(find_using_match(arr2, 3), Some(2));
    assert_eq!(arr2[2], 3);

    assert_eq!(find_using_match(arr2, 8), Some(7));
    assert_eq!(arr2[7], 8);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vehicle {
    id: u32,
    make: String,
    model: String,
    price: u32,
}

fn test_with_generic() {
    println!("Testing 'test_with_generic'...");
    let mut vehicles: Vec<Vehicle> = vec![
        Vehicle {
            id: 1,
            make: "Chevrolet".to_string(),
            model: "Malibu".to_string(),
            price: 123,
        },
        Vehicle {
            id: 2,
            make: "ford".to_string(),
            model: "ranger".to_string(),
            price: 1234,
        },
        // this is our vehicle
        Vehicle {
            id: 3,
            make: "ford".to_string(),
            model: "ranger".to_string(),
            price: 12345,
        },
        Vehicle {
            id: 4,
            make: "ford".to_string(),
            model: "T".to_string(),
            price: 12345,
        },
    ];

    // vehicles can be sorted now
    vehicles.sort();

    let result1 = find_generic(
        &vehicles,
        Vehicle {
            id: 3,
            make: "ford".to_string(),
            model: "ranger".to_string(),
            price: 12345,
        },
    );

    let result2 = find_generic(
        &vehicles,
        Vehicle {
            id: 3,
            make: "ford".to_string(),
            model: "none".to_string(),
            price: 12345,
        },
    );

    assert_eq!(result1, Some(2));
    assert_eq!(result2, None);
}

// Run the tests:
//
// ```sh
// # Run it
// cargo run --example binary_search
//
// # Watch it
// cargo watch -q -c -w examples/ -x 'run --example binary_search'
// ```
fn main() {
    test_regular_find();
    test_using_match();
    test_with_generic();
}
