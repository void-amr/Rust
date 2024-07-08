/*
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> Demonstration of ownership works in Rust 
 *  @Date -> 8/7/24 
 */


 fn main() {
     /* main function */
     let s1: String = String::from("Hello");
     let s2: String = s1; 
     // println!("{}",s1);
     println!("{}",s2); 
 }
