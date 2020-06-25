// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.

enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click {x: i64, y: i64}
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page PageUnloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("Pasted {}", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click {x, y} => {
            println!("Clicked at x={}, y={}", x, y)
        }
    }
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x+y,
            Self::Subtract => x-y
        }
    }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

pub fn play() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let add: Operations = Operations::Add;
    let subtract: Operations = Operations::Subtract;
    
    println!("{:?}", add.run(70, 4));
    println!("{:?}", subtract.run(78, 4));
}