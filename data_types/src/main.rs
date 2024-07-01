/*
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com)
 *  @Brief -> Demonstration of scalar data_types and consts in rust
 *  @Date -> 2/7/24 
 */

fn main() {
    /*  usize here is arch type which means that the data-type is set
        according to architecture of the machine. 
      */
    let x:usize = 4783989;
    let y:f32 = 55.355;
    let result = float_data(y);
    let boolean:bool = true; 
    let result_1 = boolean_data(boolean); 
    let charachter = 'z'; 
    let result_2 = char_data(charachter);

    println!(" Charachter data-type {result_2}");
    println!(" Boolean data-type {result_1}"); 
    println!(" floating data-type {result}");

    int_data(x);
}

fn int_data(num:usize) ->() {
    println!(" arch data-type from integer {num}");
}

fn float_data(num:f32) -> f32 {
    num
}

fn boolean_data(_valid:bool) -> bool {
    true
}

fn char_data(_char:char) -> char {
    return _char;
}