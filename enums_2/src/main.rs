/**
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> Demonstration of enums can be used
 *  @Date -> 8/7/24
 */

/*
   We are creating a compound data where one of the muliple values can
   be true.
   when we say we have a enum of IpAddrKind
                           we have two variants V4 and V6.
                           one of the variants can be choosen from the both the type
                           of Ip's.
                           Even tho a single variant is choosen both the variants are
                           of same type that is of enum
*/

enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    // variant of enums are namespaced
    let v4 = IpAddrKind::V4(String::from("3522346"));
    let v6 = IpAddrKind::V6(String::from("sfdlkms0"));
    route(v4,v6);
}

fn route (_ipAddr1: String, _ipAddr2: String) -> () {
    println!("This works");
}
