fn main() {

    // expected f64, found integral variable
    // mismatched types
    //fahrenheit_celsius(5);
    let celsius = fahrenheit_celsius(5.0);
    println!("Celsius values is {}", celsius);

    let n = 5;
    println!("fibonacci of {}th: {}", n, fibonacci(n));

    println!("{}", "-----------------");
    print_song();
}

/**
 * Convert temperatures between Fahrenheit and Celsius.
*/
fn fahrenheit_celsius(fahrenheit: f64) -> f64 {
    println!("Fahrenheit to Celsius conversion!");
    5.0 / 9.0 * (fahrenheit - 32.0)
}

/**
 * Generate the nth Fibonacci number.
*/
//fn fibonacci(const n: usize) -> u32 {
fn fibonacci(n: usize) -> u32 {
    println!("{}th Fibonacci", n);

    const N: usize = 100;
    //doesn't work
    //const N: usize = n;

    let mut fib = [0; N];
    fib[1] = 1;
    let mut index = 0;

    if n <= 1 {
        return fib[n];
    }

    while index < n {
        if index > 1 {
            fib[index] = fib[index - 1] + fib[index - 2];
        }

        index = index + 1;
    }

    fib[n - 1]
}

/**
 * Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage
 * of the repetition in the song.
*/
fn print_song() {
    let presents = ["a Partridge in a Pear Tree", "2 Turtle Doves", "3 French Hens",
        "4 Calling Birds", "5 Gold Rings", "6 Geese a-Laying",
        "7 Swans a-Swimming", "8 Maids a-Milking", "9 Ladies Dancing",
        "10 Lords a-Leaping", "11 Pipers Piping", "12 Drummers Drumming"];

    for x in 1..13 {
        println!("On the {} day of Christmas my true love sent to me", x);
        for y in 0..x {
            if y > 0 {
                print!("and {}", presents[y]);
            } else {
                print!("{}", presents[y]);
            }

            if y < x - 1 {
                println!("{}", ",")
            }
        }
        println!();
        println!();
    }
}
