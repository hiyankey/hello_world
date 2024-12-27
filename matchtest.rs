

pub fn test_match(){

    let age = 25;

    match age  {
        42 => {println!("{}",age)},
        0..=24 => {println!("{}",age)},
        _ => {println!("Default {}", age)}
    }
}