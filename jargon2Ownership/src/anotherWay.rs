/**
 * @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 * @Brief -> Demonstration of how memory is deallocated even if we dont
 *           assign the data to another data
 * @Date -> 8/7/24
 */

fn main() {
     /* main function */
     let mut my_string: String = String::from("Hello");
     my_string = taking_ownership(my_string);
     // Here the my_string assigned data is passed to the parameter of taking_ownership
     // so the data is trasferred and the my_string (prev Owner) can no more acess
     // the data
     println!("{my_string}");
}

fn taking_ownership(new_string: String) -> String {
     println!("{}", new_string); 
     return new_string;      
}
