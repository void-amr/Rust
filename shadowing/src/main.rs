/** 
 *  @Author -> AryanRanjane(ranjanearyan14@gmail.com) 
 *  @Brief -> Demonstration of type inference in rust 
 *  @Date -> 30/6/24
 */ 

/*
    Shadowing is the way to change the data from one data type to another or changing the same data 
    or changing the data by overshadowing the same data into the compiler 
    the compiler overshadows the previous the data from one variale into another data-type

    There is the difference between mut and shadowing* 
    
    mut makes the variable changed for the variable instance 
    ie let's say  let mut x = 5; 
    here x is mutable which means that x can be changed ie it's value can be changed  

    where as shadowing means that this x variable can be used to change the data ie 
    can be changed the instane of varible or infer the type or data or overshadow the data for a 
    scope.  
*/ 
    fn main() {
         
        /* Function scope */
        let x = 5; 

        let x = x+1; 

        {
           let x = x * 2; 
            println!("The value of x in the inner scope is : {x}"); 

        } 

        println!("The value of x in functionn scope is : {x}"); 
    } 
