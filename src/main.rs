//options and results

enum CustomOption{
    Some(i32),
    None,
}

fn find_first_a(s: String)-> CustomOption {
    for ( index,char) in s.chars().enumerate() {
    if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
     return CustomOption::None;
}

fn main() {

    let index = find_first_a(String::from("rahul"));

    match index {
          CustomOption::Some(value) => println!("index is {}", value ),
          CustomOption::None => println!("a was not found ")  
    }

}