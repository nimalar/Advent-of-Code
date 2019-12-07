use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents:String = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut chars:Vec<char> = contents.chars().collect();
	
	let mut i = 0;
	let mut previous_len = chars.len();
	
	loop
	{
		if i >= chars.len()-1
		{
			i = 0;
			if chars.len() == previous_len
			{
				break;
			}
			previous_len = chars.len();
		}
		// different cases
		if ((chars[i] < 'a' && chars[i+1] >= 'a') || (chars[i] >= 'a' && chars[i+1] < 'a')) && chars[i].to_string().to_uppercase() == chars[i+1].to_string().to_uppercase()
		{
			chars.remove(i+1);
			chars.remove(i);
		}
		else
		{
			i += 1;
		}
	}
	println!("{}", chars.len());
}

