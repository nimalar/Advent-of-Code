use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	dance(&contents, 1);
	dance(&contents, 1000000000);
}

fn dance(contents:&String, mut count:usize)
{
	let mut dancers:Vec<char> = "abcdefghijklmnop".to_string().chars().collect();
	let mut dancers_prev:Vec<Vec<char>> = Vec::new();
	let mut x = 0;
	let mut used = false;
	
	while x < count
	{
		if dancers_prev.contains(&dancers.to_vec()) && !used
		{
			// remove all useless loops
			count = x + count % dancers_prev.len();
			used = true;
		}
		else if !used
		{
			dancers_prev.push(dancers.to_vec());
		}
		// get instructions
		for instruction in contents.split(',')
		{
			let chars:Vec<char> = instruction.chars().collect();
			
			// spin
			if chars[0] == 's'
			{
				let size:usize = (&chars[1..chars.len()]).into_iter().collect::<String>().parse().unwrap();
				let mut dancer_copy = dancers.to_vec();
				let mut new_vec = dancer_copy.split_off(dancers.len()-size);
				new_vec.append(&mut dancer_copy);
				dancers = new_vec;
			}
			
			//exchange
			if chars[0] == 'x'
			{
				//find the position of '/'
				let pos_div:usize = instruction.find('/').unwrap();
				// first pos
				let pos_a:usize = (&chars[1..pos_div]).into_iter().collect::<String>().parse().unwrap();
				// second pos
				let pos_b:usize = (&chars[pos_div+1..chars.len()]).into_iter().collect::<String>().parse().unwrap();		
				dancers.swap(pos_a, pos_b);
			}
			
			//partner
			if chars[0] == 'p'
			{
				//find the positions to be swapped
				let pos_a:usize = dancers.iter().position(|&s| s == chars[1]).unwrap();
				let pos_b:usize = dancers.iter().position(|&s| s == chars[3]).unwrap();
				dancers.swap(pos_a, pos_b);
			}
			
		}
		x += 1;
	}
	println!("{}", dancers.into_iter().collect::<String>());
}
