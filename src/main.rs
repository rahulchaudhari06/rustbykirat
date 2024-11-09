enum Shape{
    Rectanle(f64,f64),
    Circle(f64)
}

fn calculate_area(shape: Shape) -> f64{
    let area = match shape  {
        Shape::Circle(r)=> 3.14 * r * r,
        Shape::Rectanle(a,b )=> a*b,
    };
    return area;
}
fn main(){

    let rect = Shape::Rectanle(2.0, 4.0);
    let circle = Shape::Circle(6.0);

    println!("Area of Rectanle is: {} and area of circle is: {}",calculate_area(rect),calculate_area(circle) )
}

