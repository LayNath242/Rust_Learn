use std::f32::consts::PI;

fn operator() {
    let mut a = 2+8*4/3;
    println!("value of a = {}", a);

    a += 1;
    println!("value of a = {}", a);

    println!("Remainder of {}/{} = {}",
    a, 5, a%5);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed = {}", a, a_cubed);

    let b = 3.8;
    let b_cubed = f32::powi(b, 3);
    println!("{} cubed = {}", b, b_cubed);

    let b_to_pi = f32::powf(b, PI);
    println!("{}^pi = {}", b, b_to_pi);

    //Bitwise
    let c = 1|2;  // | OR & AND ^ XOR ! NOR
    println!("c = {}", c);

    let two_to_10 = 2 << 3;

    println!("two_to_10 = {}", two_to_10);
}


fn main(){
    operator()
}
