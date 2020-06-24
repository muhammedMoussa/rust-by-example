// The 'a defines a lifetime
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
} 

// A unit struct
#[derive(Debug)]
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

pub fn play() {
    // Create struct with field init shorthand
    let name = "Muhammed";
    let age = 27;
    let muhammed = Person { name, age };

    // Print debug struct
    println!("{:?}", muhammed);

    // Instantiate a `Point`
    let point: Point = Point {x: 10.2, y: 10.4};

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;
    println!("{:?}", _unit);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // @TODO: add rect_area and square 
}