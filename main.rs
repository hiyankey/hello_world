// pub mod lib;
// pub mod closures;
// pub mod matchtest;
pub mod optiontest;
fn main() {
    println!("Hello world!");
    // test_func();
    //    let result =  lib::utils::get_full_name("Emmanuel","Yankey");
    //    println!("{}", result);
    //    println!("{}", lib::db::query("GET"))

    // test_if();

    // test_while()

    // test_loop(); 
    // test_for();
    // closures::test_closures();
    // matchtest::test_match();
    let result = optiontest::test_option_arch();
    if result.is_some() {
        println!("User has selected an option");
        println!("{}", result.unwrap().to_string());
    } else {
        println!("User has selected None")
    }

}
#[allow(dead_code)]
fn test_for(){
let arr = [12,19, 17, 29,36];
let age_to_drive = 18i32;

for age in arr {
    println!("Your current age is {0}", age);
    if age > age_to_drive {
        println!("You are eligible to drive")
    } else {
        println!("You are not eligible yo drive")
    }
}
}


#[allow(dead_code)]
fn test_loop(){
let mut x =  0u8;
loop {
    println!("Hello from 🦀🦀🦀");
    if x > 10 {
        break;
    };
    x += 1
};

}

#[allow(dead_code)]
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
