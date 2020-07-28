use std::mem::size_of_val;

pub fn core_data_type() {
    let a:u8 = 100; //8 bit unsigned integer
    println!("a = {}", a);

    // a = 102; // immutable value so can not change

    let mut b:i8 = 100; // 8 bit unsigned integer
    println!("b = {}", b);

    b = -23; // mutable value so can change
    println!("b = {}", b);

    let c = 1234567890;

    println!("c = {} size of c = {} byte", c, size_of_val(&c));

    let d:isize = 129;
    let size_of_d = size_of_val(&d);
    println!("d = {} size of d = {} you use {} bit os"
    , d, size_of_d, size_of_d*8);

    let e:f32 =2.5;
    println!("e = {} size of e = {} byte", e, size_of_val(&e));

    let f = "you are cool";
    println!("f = {} size of f = {} byte", f, size_of_val(&f));

    let g:bool = true;
    println!("g = {} size of g = {} byte", g, size_of_val(&g));
}
