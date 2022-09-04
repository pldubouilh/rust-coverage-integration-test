fn do_something() {
    eprint!("hello");
}

fn do_something_else() {
    eprint!(" ");
    eprint!("world");
}

fn do_finally() {
    eprint!(" ");
    eprint!("joe");
}

fn main() {
    do_something();
    do_something_else();
    // do_finally();
    eprintln!("");
}
