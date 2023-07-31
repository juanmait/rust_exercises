const BRACES: [u8; 3] = [b'{', b'(', b'['];

// We're gonna be using string slices (&str)
// All slices happen to be offsets into a pointer in memory.
fn brackets_are_balanced(string: &str) -> bool {
    // We will have a stack in which we're going to store each occurrence of
    // the different types of opening braces in the order as they appear in the given
    // `str`.
    let mut stack: Vec<u8> = vec![];

    string.as_bytes().iter().all(|b| match b {
        // if the char is any of the closing braces..
        b'}' => stack.pop() == Some(b'{'),
        b')' => stack.pop() == Some(b'('),
        b']' => stack.pop() == Some(b'['),
        _ => {
            // otherwise if the char is an opening brace
            // we store is in our stack
            if BRACES.contains(b) {
                stack.push(*b);
            }
            true
        }
    }) && stack.len() == 0 // at the end of the iteration the stack has to be empty
}

fn main() {
    assert!(brackets_are_balanced(""));
    assert!(brackets_are_balanced("[[]]"));
    assert!(brackets_are_balanced("()"));

    assert!(!brackets_are_balanced("[[]"));
    assert!(!brackets_are_balanced("{[])"));
}
