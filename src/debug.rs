// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

pub fn play() {
   // `Structure` is printable!
   println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "muhammed";
    let age = 27;
    let muhammed = Person{name, age};
    // Pretty print
    println!("{:#?}", muhammed);
}