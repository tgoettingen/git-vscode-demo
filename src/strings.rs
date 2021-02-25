pub fn run(){

    let hello="hello";

    let mut h = String::from("haha ");
    println!("{},{},len {}, {}",hello,h,h.len(),hello.len());
    h.push('W');
    h.push_str("orld");
    println!("{},capacity {},contain world {}",h,h.capacity(),h.contains("World"));

    println!("replace {}",h.replace("World", "new time"));


    for word in h.split_whitespace(){
        println!("{}",word);
    }


    // creat string with capacity
    let mut s=String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2,s.len());

    
    println!("{}",s);




}