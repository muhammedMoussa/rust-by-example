use std::convert::From;

#[derive(Debug)]
struct MyNumber {
    value: i32
}

impl From<i32> for MyNumber {
    fn from(item: i32) -> Self {
        MyNumber {value: item}
        // MyNumber {value: item*item}
    }
}

pub fn play() {
    let num = MyNumber::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type declaration
    let num: MyNumber = int.into();
    println!("My number is {:?}", num);
}