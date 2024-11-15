/*

 fn main(){
    let mut s1: String = String::from("Hello memory leak");
    println!("{}", s1 );

    s1 = pint_str(s1);

    println!("{}", s1 );


  
}

fn pint_str(s2: String)-> String{

    println!("{} pintString", s2);
    return s2;
} 

this code is ugly as we need to return the value and the ownership keeps jumping too much 

*/
// We should use references and borrowing instead



fn main(){
    let mut s1 = String::from("hey there!");
    // let s2 = &s1;

    println!("{}", s1);
    do_something( &mut s1);
    // println!("{}", s1);

}

fn do_something(s2: &mut String) {
    s2.push_str(" how are you?");
    println!("{}", s2); //s2 owns the value

}