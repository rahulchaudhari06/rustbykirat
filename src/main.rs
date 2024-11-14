 fn main(){

    create_string();
  
}

fn create_string(){
    let s1: String = String::from("Hello memory leak");
    let _s2: String = s1.clone();



    println!("{}", s1 );

}