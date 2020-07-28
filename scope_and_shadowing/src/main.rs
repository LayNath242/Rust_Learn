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

fn main() {
    scope_and_shadowing();
}