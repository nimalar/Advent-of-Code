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
		
	let mut vec:Vec<i32> = Vec::new();
	for number in contents.split_whitespace()
	{
		vec.push(number.parse().unwrap());
	}
	
	let mut freq = 0;
	let mut double_freq = 0;
	let mut found = false;
	let mut freqs:Vec<i32> = Vec::new();
	while !found
	{
		for i in 0..vec.len()
		{
			freq += vec[i];
			if freqs.contains(&freq)
			{
				double_freq = freq;
				found = true;
				break;
			}
			else
			{
				freqs.push(freq);
			}
		}
	}
	
	println!("double frequency: {}", double_freq);
}
