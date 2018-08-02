// global constant
const MAX_POINTS: u32 = 100_100;

//1. Mutablility Example
fn mutablity_example() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}

//2. Global Constant Example
fn global_const_example() {
    println!("Max Points: {}", MAX_POINTS);
}

//3. Shadowing Examples
fn shadowing_examples() {
    // example 1
    let y = 5;

    // effectively creating new variable
    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // example 2
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // example 3 : not allowed to mutate a variable type;
    // snake case
    // let mut spaces_1 = "    ";
    // spaces_1 = spaces_1.len();

    // gives unused variable error
    //let guess : u32 = "42".parse().expect("Not a number!");

    // no unused variable error
    let _guess: u32 = "42".parse().expect("Not a number!");

    // not adding annotation type will give error during conversion
    //  let guess = "42".parse().expect("Not a number!");
}

//4. Floating Point Type Examples
fn floating_point_examples() {
    let f = 2.0; // f64
    println!("The value of f is: {}", f);

    let g: f32 = 3.0;// f32
    println!("The value of g is: {}", g);
}

//5.  Boolean Type Examples
fn boolean_examples() {
    let t = true;
    println!("The value of t is: {}", t);

    // with explicit annotation type
    let f: bool = false;
    println!("The value of f is: {}", f);
}

//6. Character Type - Unicode Scalar Value
fn char_examples() {
    let c = 'z';
    println!("The value of c is: {}", c);

    let z = 'Ƶ';
    println!("The value of z is: {}", z);

    let hear_eyed_cat = '😻';
    println!("The value of hear_eyed_cat is: {}", hear_eyed_cat);
}

//7. Tuple Type (Compound Type) Examples
fn tuple_examples() {
    // allows for different types unlike array type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // printing tuple doesn't work
    // println!("The value of tup is: {}", tup);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // dot notation examples
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of x.0 is: {}", five_hundred);

    let six_point_four = x.1;
    println!("The value of x.1 is: {}", six_point_four);

    let one = x.2;
    println!("The value of x.2 is: {}", one);
}

//8. Array Type (Compound Type) Examples
fn array_examples() {
    // element of array same type unlike tuple
    // array is flexible as vector
    // array size cannotbe changed once allocated
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    println!("The value of a[0] is: {}", first);

    let second = a[1];
    println!("The value of a[1] is: {}", second);

    let months = ["January", "February", "March", "April", "May",
        "June", "July", "August", "September", "October", "November", "December"];

    // accessing invalid index
    let index = 10;

    // no compilation error but run error
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the
    // index is 10', src/main.rs:126:1
    // let element = a[index];
}

fn main() {
    mutablity_example();
    global_const_example();
    shadowing_examples();
    floating_point_examples();
    boolean_examples();
    char_examples();
    tuple_examples();
    array_examples();
}
