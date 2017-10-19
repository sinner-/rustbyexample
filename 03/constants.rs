//Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    //Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;
    let m = 9;

    //Access constant in main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    println!("{} is {}", m, if is_big(m) { "big" } else { "small" });

    //Cant modify a `const`
    //THRESHOLD = 5;
}
