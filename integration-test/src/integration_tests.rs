use crate::utils::run_ensure_outputs;

#[test]
fn test_exec() {
    {
        println!("~~ test output");
        run_ensure_outputs("hello world")
    }
}