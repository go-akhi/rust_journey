fn main_aside() {
    loop {
        println!("Again!");
    }
}

fn main_aside_1() {
    let mut counter: i32 = 0;

    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

fn main_aside_2() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn main_aside_3() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Done");
}

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
