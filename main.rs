// pub mod lib;
fn main() {
    println!("Hello world!");
    // test_func();
    //    let result =  lib::utils::get_full_name("Emmanuel","Yankey");
    //    println!("{}", result);
    //    println!("{}", lib::db::query("GET"))

    // test_if();

    test_while() 
}

fn test_while() {
let drivers_age = 18u8;
let mut current_age = 0u8; 
    while current_age < drivers_age {
        println!("Waiting...");
        current_age += 1;
    }
}


#[allow(dead_code)]
fn test_if() {
    let std_user_age = 18u8;

    println!("Enter user age:");
    let user_input: &mut String = &mut String::new();
    std::io::stdin().read_line(user_input).unwrap();
    let user_age = user_input.trim().parse::<u8>().unwrap();

    if user_age >= std_user_age {
        println!("Yay! you're old enough!");
    }
}
#[allow(dead_code)]
fn test_func() {
    let x: f32 = 225.0;
    let y: u8 = x as u8 - 5;
    println!("{:?}", y);

    let mut is_ready: bool = true;
    println!("{}", is_ready);
    is_ready = false;

    println!("{}", is_ready);
    let char: char = '🔥';
    let str: &str = "Hello dev!";
    println!("{}", str);
    println!("{}", char);

    let point: (i32, u32) = (-1, 4);
    println!("{:?}", point);

    let arr: [i16; 6] = [12, 45, 34, 23, 12, 12];

    println!("{:?}", arr);
    let new_arr: &[i16] = &arr[0..=5];

    println!("{:?}", new_arr);
}
