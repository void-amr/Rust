/**
 * @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 * @Brief -> Demonstration of impl and struct
*/

/* Struct with multiple values of shape
    of different data-types
*/
struct Rect {
    shape_type: String,
    width: i64,
    height: i64,
    is_shape: bool,
}

impl Rect {
    /* Here we are attaching function to instances of struct Shape */
    fn area(&self) -> i64 {
        self.width * self.height
    }
}

fn main() {
    let shape_type: String = String::from("Rectangle");
    let is_shape: bool = true;
    let rect = Rect {
        shape_type,
        width: 30,
        height: 30,
        is_shape,
    };
    println!("The area of Rectangle is {}", rect.area());
    println!("The Type of Rectangle is {}", rect.shape_type);
    println!("Is this even a shaped {:?} ", rect.is_shape);
}
