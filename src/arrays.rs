use std::mem;


pub fn run(){

    let mut num: [i32;5]=[1,2,3,4,5];

    num[3] =20;
    println!("{:?}",num);

    println!(" single var, {}",num[3]);

    println!(" memory occupation, {}",mem::size_of_val(&num));


    let slice: &[i32]=&num[0..2];
    println!("slice: {:?}",slice);

}