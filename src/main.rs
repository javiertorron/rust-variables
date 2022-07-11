fn main() {
    // Constants always have to be declared as a type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // You cannot modify immutable values
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;   ----> This would throw an exception
    // println!("The value of x is: {x}");
    let mut y = 9;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    // Shadowing variables
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}