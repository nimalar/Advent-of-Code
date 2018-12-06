use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut vec:Vec<String> = Vec::new();
	for line in contents.split_whitespace()
	{
		vec.push(line.to_string());
	}
	
	let mut doubles = 0;
	let mut triples = 0;
	
	for i in 0..vec.len()
	{
		let mut doubled = false;
		let mut tripled = false;
		
		for c in vec[i].chars()
		{
			let matches:Vec<&str> = vec[i].matches(c).collect();
			if !doubled && matches.len() == 2
			{
				doubles += 1;
				doubled = true;
			}
			else if !tripled && matches.len() == 3
			{
				triples += 1;
				tripled = true;
			}
			if tripled && doubled
			{
				break;
			}
		}
		
	}
	
	println!("checksum: {}", doubles * triples);
}

