// // result enum
// use std::fs;

// fn main (){
//     let greeting_file_result = fs::read_to_string("src/hello.txt");

//     match greeting_file_result{
//     Ok(file_content) => {
//         println!("File read successfully! It says: {}",file_content )
//              },
//     Err(error)=>{
//         println!("Failed to read file! Error: {}", error)
//             }
//     }
// }

 
use std::fs::read_to_string;

fn main(){
    let ans = read_from_file_rahul(String::from("src/hello.txt"));

    match ans {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("{}", e),
    }
}

fn read_from_file_rahul (file_path: String) -> Result<String,String> {
    let result = read_to_string(file_path);

    match result {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("file not read"))
    }
    
}