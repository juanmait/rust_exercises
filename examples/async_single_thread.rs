use std::{cell::Cell, rc::Rc};

#[derive(Debug, Clone)]
struct State {
    count: Cell<usize>,
}

type SharedState = Rc<State>;

fn main() {
    let state: SharedState = Rc::new(State {
        count: Cell::new(0),
    });

    // Build an async tokio runtime that runs everything on the current thread.
    let runtime = tokio::runtime::Builder::new_current_thread()
        // Enables both I/O and time drivers.
        // .enable_all()
        .build()
        .expect("failed building single thread runtime");

    // combine it with a `LocalSet, which means it can spawn `!Send` futures.
    let local_set = tokio::task::LocalSet::new();

    local_set.block_on(&runtime, async_main(state));
}

async fn async_main(state: SharedState) {
    let st1 = state.clone();
    let st2 = state.clone();

    let result = tokio::task::spawn_local(async move {
        let value = st1.count.get();
        println!("task #1 before: {}", value);
        st1.count.set(value + 1);
        println!("task #1 after: {}", st1.count.get());
    })
    .await;

    let result2 = tokio::task::spawn_local(async move {
        let value = st2.count.get();
        println!("task #2 before {}", value);
        st2.count.set(value + 1);
        println!("task #2 after {}", st2.count.get());
    })
    .await;

    result.unwrap();
    result2.unwrap();

    println!("count after all tasks: {}", state.count.get());
}
