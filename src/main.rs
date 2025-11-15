fn main() {
    println!(
        "{}",
        std::env::var("that_var_the_build_script_sets").unwrap()
    );
}
