const LIFE:u8 = 12;
static S:u32 = 1000;
static mut Z:u8 = 22;

fn scope_and_shadowing() {
    let a = 129;
    println!("original outside a = {}", a);
    let a = 70;
    println!("new assign outside a = {}", a);

    {
        let a = 234;
        println!("assign inside scope a = {}", a);

        let b = 40;
        println!("b = {}", b);
    }

    println!("last outside a = {}", a);
    // println!("b = {}", b); // can not get b value

}

pub fn scope_gobal() {
    scope_and_shadowing();
    println!("{}", LIFE);
    println!("{}", S);

    unsafe{
        Z = 25;
        println!("{}", Z);
    }

}
