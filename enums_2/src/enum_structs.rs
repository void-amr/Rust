/**
 *  @Author -> AryanRanjane(ranjanearayan14@gmail.com)
 *  @Brief -> Demonstraiton of enum_with structs
 *  @Date -> 8/7/24
 */

#[derive(Debug)]

enum IpAddrKind {
     V4,
     V6,
} 
struct IpAddr {
     kind: IpAddrKind,
     address: String,
} 

fn main() {
    /* main function */
     let home = IpAddr {
         kind: IpAddrKind::V4,
         address: String::from("126.456.353"),
    };
     println!("Ipaddress is of kind {:?}", IpAddrKind::V4);
     println!("Ipaddres is {}", home.address);
}
