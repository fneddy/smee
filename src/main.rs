use std::io::{self, BufRead};

use smee::*;



fn main(){
    let mut ctx = Context::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Err(error) = ctx.eval(line.expect("stdin read error").as_bytes()) {
            /*currently only in debug mode */
            println!("Error: {:#?}",error)
        }
    }
    
}
