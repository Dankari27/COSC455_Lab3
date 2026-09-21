fn main() {
    println!("Hello, World!\n");

    // Some practice code for the Linkdin learn stuff. Totally not going to forget this in like 3 hours. Will try and practice more on this later. 
    // Part 2: Declaring Variables & Types 
    let count = 42;
    let price = 9.99;
    let is_active = true;
    let letter = 'R';
    println!("Count: {}, Price: {}, Active: {}, Letter: {}", count, price, is_active, letter);

    // Part 3: Compound Types 
    let numbers = [10, 20, 30, 40];
    println!("First number: {}, Total elements: {}", numbers[0], numbers.len());

    let student = ("Richard", 2026);
    println!("Name: {}, Year: {}", student.0, student.1);

    // Part 4: Functions
    let total = add(15, 25);
    println!("Function result: {}", total);

    // Part 5: Program Flow Control
    let score = 88;
    if score >= 70 {
        println!("Status: Passed");
    } else {
        println!("Status: Failed");
    }

    print!("Counting: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
}

// Helper function
fn add(a: i32, b: i32) -> i32 {
    a + b
}