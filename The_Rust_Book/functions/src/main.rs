fn main() {
    println!("Hello World!");

    another_function(34, 'm');
}

fn another_function(x:i32, unit: char) {
    println!("Another function. The value of x is {x}, the unit is {unit}. The measurement is therefore {x}{unit}.");
}