// Based on this video: https://www.youtube.com/watch?v=vw1PSv7cmHI
//
// Given a string of digits, calculate the largest product for a contiguous
// substring of digits of length n. For example, for the input '1027839564',
// the largest product for a series of 3 digits is 270 (9 * 5 * 6),
// and the largest product for a series of 5 digits is 7560 (7 * 8 * 3 * 9 * 5).
//
// `cargo run --example largest-series-product`

use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

/// largest series product
fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    let results = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        // check https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E
        // for more info about how collect can return a Result!
        .collect::<Result<Vec<u32>, Error>>()?;

    results
        .windows(span)
        .map(|w| w.iter().map(|d| *d as u64).product())
        .max()
        .ok_or(Error::SpanTooLong)
}

/// Largest series product (alternative implementation)
/// 
/// About the alternative implementation: https://youtu.be/vw1PSv7cmHI?t=918s
fn lsp_alt(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    } else if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    // the window should store a max of `span` digits
    let mut window = VecDeque::new();

    let mut product: u64 = 1;
    let mut max: u64 = 1;
    let mut zeroes: usize = 0;

    // iterate over every char computing the product registering
    // the max product value so far
    for c in string_digits.chars() {
        let val = c.to_digit(10);

        if val.is_none() {
            return Err(Error::InvalidDigit(c));
        }

        let digit = val.unwrap() as u64;

        window.push_front(digit);

        if digit == 0 {
            zeroes += 1;
        } else {
            // compute the product
            product = product * digit;
        }

        // ensure that `window` never grow greater than `span`
        // by removing the last (oldest) element from it
        //
        // Since the max was already computed for this value
        // and the max is all we care about (we don't care what the items
        // were in the window when max was computed), we discard
        // this digit and "undo" the product that this item caused..
        if window.len() > span {
            let oldest = window.pop_back().unwrap();

            if oldest == 0 {
                zeroes -= 1;
            } else {
                // this is kind of an "undo" of the oldest
                // product computation.
                product = product / oldest;
            }
        }

        if window.len() == span && product > max && zeroes == 0 {
            max = product
        }
    }

    Ok(max)
}

fn main() {
    assert_eq!(lsp("1027839564", 3), Ok(270));
    assert_eq!(lsp_alt("1027839564", 3), Ok(270));
}
