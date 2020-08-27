struct A;
struct Single(A);
struct SingleGen<T>(T);

pub fn play() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}