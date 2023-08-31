/// cargo watch -c -x 'run --example deref_mut'
///     `-x` to specify the cargo command.
///     `-c` to clear the screen before each run.
fn main() {
    let slice = [&mut b'h', &mut b'e', &mut b'l', &mut b'l', &mut b'o'];
    let first = slice.first().unwrap();

    dbg!(first);

    *slice[0] += 1;
    *slice[0] = *slice[0] + 1;

    let first = slice.first().unwrap();

    dbg!(first);
}
