fn main() {
    // let mut vec = Vec::new();
    let mut vec = vec![1,2,3,4,5];
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // vec.push(4);
    // vec.push(5);
    vec.remove(3); // removes index 3 i.e. value = 4 from the vector


    println!("{:?}", vec);
    println!("{:?}", even_filter(vec))
}

fn even_filter(vec: Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    return new_vec;

}