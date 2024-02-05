use _rust_concurrency::{passing_message, shared_state, simple_example};

fn main() {
    simple_example::run();

    passing_message::run();

    shared_state::run();
}
