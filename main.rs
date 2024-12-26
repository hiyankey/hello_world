

fn main(){
    println!("Hello world!");
    test_func();
}

fn test_func(){

    let x:f32 = 225.0;
    let y:u8 = x as u8 -5;
    println!("{:?}", y);

    let mut is_ready: bool = true;
    println!("{}",is_ready);
    is_ready = false;

    println!("{}",is_ready);
let char: char = '🔥';
    let str: &str = "Hello dev!";
    println!("{}", str);
    println!("{}", char);


    let point: (i32, u32) = (-1,4);
println!("{:?}", point);

let arr: [i16; 3] = [12, 45, 34];

println!("{:?}", arr)
}