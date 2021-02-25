pub fn run(){
    // variables hold primitive data or refrences to data
    // variables are immutable by default
    // rust is a block-scoped language


    let name = "haisen";
    let mut age = 43;
    

    println!("My name is {}, {}",name,age);
    age =42;
    println!("age: {}",age);
    const ID:i32=1;
    println!("ID: {}",ID)
}