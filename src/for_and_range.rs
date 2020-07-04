pub fn play() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // for n in 1..=100 {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Tom", "Jhon"];
    for name in names.iter() {
        match name {
            &"Bob" => println!("There is a rustacean among us!"),
            &"Jhon" => println!("There is another rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }
}