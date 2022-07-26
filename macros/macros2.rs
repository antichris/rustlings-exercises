// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    do_macro();
}

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn do_macro() {
    my_macro!();
}
