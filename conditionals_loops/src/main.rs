/**
 *  @Author -> AryanRanjane(ranjane14@gmail.com)
 *  @Brief -> Rust by example 
 *  @Date -> 2/7/24
 */

use std::io;

pub fn main() {
    /* Main function */
    let mut n:String = String::new();
    println!("Enter a string "); 
    io::stdin().read_line(&mut n).expect("No charchters allowed");
    let ans = get_first_name(n); 
    println!("{}",ans);
}

pub fn get_first_name(string:String) -> String { 
    let mut first_name = String::new(); 
    for c in string.chars() { 
        if c == ' ' { 
            break;
        }
        first_name.push(c);         
    }
    return first_name;
}