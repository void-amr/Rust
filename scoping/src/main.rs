/**
 * @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 * @Brief -> Demonstration of scoping in rust
 * @Date -> 5/7/24
*/

fn main() {
    /* main_function */
    let long_lived_binding = 1;

    {
        /* inside the block */
        let short_lived_binding = 2;
        println!("{short_lived_binding}");
        println!("{long_lived_binding}");
    }
    // Error as the variable is declared in the inside block of main block.
    println!("{long_lived_binding}");
    println("{short_lived_long}");
}
