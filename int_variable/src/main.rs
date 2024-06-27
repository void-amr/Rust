/**
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> Demonstration of data-type in rust
 *  @Date -> 27/6/24
 */

fn main() {
    integer();
}

/**
  There are three main types in rust

  Number => integer and floating
  Booleans => true or false 
  String => collection of charchters

  String being the most complex primitive type in rust because string acts as a collection of characters.
  which grows dynamicaly as the program runs ie something like vector
*/

fn integer() {
  /*
  signed integer
  */
  let x:i8 = -12;
  let y:i32 = -124;
  let z:i64 = -1234543354;
  let w:i128 = -123454335436327472477643637643;
  /*
  unsigned integer 
  */
  let a:u8 = 12;
  let b:u16 = 124;
  let c:u32 = 1234543354;
  let d:u64 = 1234543354;
  let e:u128 = 123454335445378648856437838534578;

  print!("value of a {}\n",a);
  print!("value of b {}\n",b);
  print!("value of c {}\n",c);
  print!("value of d {}\n",d);
  print!("value of e {}\n",e);
  
  print!("value of x {}\n",x);
  print!("value of y {}\n",y);
  print!("value of z {}\n",z); 
  print!("value of w {}\n",w);  
}

