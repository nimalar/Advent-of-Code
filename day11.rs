use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	// the seq of lengths
	let lengths:Vec<&str> = contents.split(',').collect();
	
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut z:i32 = 0;
    let mut max = 0;
    
    for length in lengths
    {
    
    	if length == "n"
    	{
    		z -= 1;
    		y += 1;
    	}
    	if length == "ne"
    	{
    		z -= 1;
    		x += 1;
    	}
    	if length == "nw"
    	{
    		x -= 1;
    		y += 1;
    	}
    	if length == "s"
    	{
    		z += 1;
    		y -= 1;
    	}
    	if length == "se"
    	{
    		x += 1;
    		y -= 1;
    	}
    	if length == "sw"
    	{
    		z += 1;
    		x -= 1;
    	}
    
        if ((x.abs() + y.abs() + z.abs()) / 2) > max
        {
            max = (x.abs() + y.abs() + z.abs()) / 2;
        }
    }
    	
    println!("{}", ((x.abs() + y.abs() + z.abs()) / 2));
    println!("{}", (max));
}
