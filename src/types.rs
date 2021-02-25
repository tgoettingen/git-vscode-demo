pub fn run(){

    let x=1; // default i32
    let y=2.5; // default f64

    let z:i64 =3456456435636;

    let is_active=true;

    let a1="c"; // or a1='c';
    let face='\u{1f600}';


    println!("{},{}",x,y);
    println!("{}",std::i32::MAX);
    println!("{:?}",(x,y,z, is_active,a1,face));

}