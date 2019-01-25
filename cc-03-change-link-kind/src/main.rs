extern "C" {
    fn small_function() -> i32;
}

fn main() {
    unsafe {
        println!("small {}", small_function());
    }
}
