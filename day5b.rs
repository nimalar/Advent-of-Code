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
		
	let chars:Vec<char> = contents.chars().collect();
	let mut min_len = 100000000;
	
	for j in 97u8..123
	{
		let k = j as char;
		let mut stripped_chars = chars.to_vec();
		stripped_chars.retain(|c| c != &k && c != &k.to_ascii_uppercase());
		let mut i = 0;
		let mut previous_len = stripped_chars.len();
		
		loop
		{
			if i >= stripped_chars.len() - 1
			{
				i = 0;
				if stripped_chars.len() == previous_len
				{
					break;
				}
				previous_len = stripped_chars.len();
			}
			// different cases
			if ((stripped_chars[i] < 'a' && stripped_chars[i+1] >= 'a') || (stripped_chars[i] >= 'a' && stripped_chars[i+1] < 'a')) && stripped_chars[i].to_string().to_uppercase() == stripped_chars[i+1].to_string().to_uppercase()
			{
				stripped_chars.remove(i+1);
				stripped_chars.remove(i);
			}
			else
			{
				i += 1;
			}
		}
		if stripped_chars.len() < min_len
		{
			min_len = stripped_chars.len();
		}
	}
	println!("{}", min_len);
}

