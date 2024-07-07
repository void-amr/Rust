/**
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> Demonstration of structs in rust
 *  @Date -> 7/7/24
 */

struct User {
    /* struct is similar to that of c/c++ it is basically
    a contiguous memory with paddding */
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    /* Main function */
    let name: String = String::from("Aryan");
    let email: String = String::from("itzAryan@35gmail.com");
    let user = User {
        active: true,
        username: name,
        email, // can add , as the key pair value start with same name's
        sign_in_count: 1,
    };
    println!("User details username {:?}", user.username);
}
