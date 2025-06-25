fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        return 1;
    } else if color == "green" {
        return 2;
    } else if color == "blue" {
        return 3;
    } else {
        return 0
    }
}

fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_loop(mut number: i32) {
    let mut i = number - 1;
    
    while i > 1 {
        number = number * i;
        i -= 1;
    };
    println!("{number}");
}

fn factorial_recursive(number:  i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial_recursive(number - 1)
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number_match("blue"));
    factorial_loop(4);
    println!("{}",factorial_recursive(5));
}