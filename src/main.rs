// structs

struct Rect {
    width: u32,
    height: u32,

}

impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
       2 * ( self.width + self.height)
    }
}

fn main(){
  let rect1 = Rect{
    width: 30,
    height:50,
    // you cannot define the function inside the stuct we need to just implement it and we can use it later however
  };

  println!("The area is: {}", rect1.area());
  println!("The perimeter is: {}", rect1.perimeter())
}


