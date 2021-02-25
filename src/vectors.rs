use std::mem;


pub fn run(){

    let mut num: Vec<i32>=vec![1,2,3,4,5];

    num[3] =20;
    num.push(6);

    println!("{:?}",num);
    num.pop();
    num.pop();
    println!("{:?}",num);
    
    println!(" single var, {}",num[3]);

    println!(" memory occupation, {}",mem::size_of_val(&num));


    let slice: &[i32]=&num[0..2];
    println!("slice: {:?}",slice);
    
    for x in num.iter(){
        println!("number: {}",x);
    }


    for x in num.iter_mut(){
        *x *=2;
    }

    println!("numbers vec: {:?}",num);

}