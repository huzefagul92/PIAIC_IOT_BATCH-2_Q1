
        
    /* This program is for printing series of first 'N' (user given limit) Fibonacci Numbers on the console */

use std::io;

pub fn fibonacci() {

    println!("\n    Please enter the quantity of Fibonacci number series\n    ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("no data is given");
    let num : u32 = num.trim().parse().unwrap();
    
    let mut first   : usize = 0;
    let mut second  : usize = 1;
    let mut initial : usize = 0;
    let mut next    : usize; 
    println!("\n     The following is the Fibonacci series\n");
    println!("    **************************************\n");
                // for first N Fibonacci Series, we used 0..num Range pattern with num excluding
    for _x in 0..num {            
        if initial <= 1 {
            next = initial;
            initial= 1+initial;
        }

        else{
            next = first + second;
            first = second;
            second = next;
        }      
        println!("      Number-{} : {},   \n",_x+1, next); 
    }
}
