
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


