extern "C" {
    fn small_function() -> i32;
    fn big_function() -> i32;
}

fn main() {
    unsafe {
        println!("small {}", small_function());
        println!("big   {}", big_function());
    }
}
