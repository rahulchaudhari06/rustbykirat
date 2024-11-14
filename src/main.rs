
 fn main(){
    let s1: String = String::from("Hello memory leak");
    println!("{}", s1 );

    pint_str(s1);

    println!("{}", s1 );


  
}

fn pint_str(s2: String){

    println!("{}", s2);
} 