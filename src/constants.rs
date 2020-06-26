// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

pub fn play() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);

    println!("{} is {}", n, if is_big(n) {"Big"} else {"Small"})

    // Error! Cannot modify a `const`.
    //  THRESHOLD = 5;
    // FIXME ^ Comment out this line
}