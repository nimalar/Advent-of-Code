use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	
	let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

	let chars:Vec<char> = contents.chars().collect();
	let mut x = 0;
	let mut level = 0;
	let mut score = 0;
	let mut garbage = false;
	let mut garbage_count = 0;
	while x < chars.len()
	{
		if !garbage
		{
			if chars[x] == '{'
			{
				level += 1;
			}
			if chars[x] == '}'
			{
				score += level;
				level -= 1;
			}
			if chars[x] == '<'
			{
				garbage = true;
			}
		}
		// wait for garbage to stop
		else
		{
			if chars[x] == '>'
			{
				garbage = false;
			}
			else if chars[x] == '!'
			{
				x += 1;
			}
			else
			{
				garbage_count += 1;
			}
		}
		x += 1;
	}
	
	println!("osa1: {}", score);
	println!("osa2: {}", garbage_count);
}
