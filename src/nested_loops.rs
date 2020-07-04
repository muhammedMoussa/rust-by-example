#![allow(unreachable_code)]

pub fn play() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            //break;
            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    
    println!("Exited the outer loop");
}