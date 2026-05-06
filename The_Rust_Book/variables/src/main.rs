fn main_aside() {
    let mut x = 4;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}

fn main() {
    let x = 4;
    let x = x + 1;
    println!("The value of x is {x}");
    {
        let x = x*2;
        println!("Value of x in inner scope is {x}");
    }
    println!{"Value of x after the inner scope is {x}"};
}