use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut pots:Vec<char> = Vec::new();
	let mut rules:HashMap<&str, char> = HashMap::new();
	let mut zero_index:i32 = 0;
	
	for line in contents.lines()
	{
		if line.contains("initial state:")
		{
			let words:Vec<&str> = line.split(' ').collect();
			pots = words[2].chars().collect();
		}
		else if line.contains("=>")
		{
			let words:Vec<&str> = line.split(" => ").collect();
			rules.insert(words[0], words[1].chars().collect::<Vec<char>>()[0]);
		}
	}
	
	for _i in 0..20
	{
		let mut new_pots:Vec<char> = vec!['.'; pots.len()];
		let mut x:i32 = -4;
		let mut offset = 0;
		while x < pots.len() as i32
		{
			let mut y = x;
			let mut rule_string = String::new();
			while y < x + 5
			{
				if y < 0 || y >= pots.len() as i32
				{
					rule_string.push_str(".");
				}
				else
				{
					rule_string.push_str(&pots[y as usize].to_string());
				}
				y += 1;
			}
			let value = rules.get(&*rule_string).unwrap_or(&'.');
			if value == &'#'
			{
				let mut z = x + offset;
				while z + 2 < 0
				{
					new_pots.insert(0, '.');
					zero_index += 1;
					offset += 1;
					z += 1;
				}
				while z + 2 >= new_pots.len() as i32
				{
					let length = new_pots.to_vec().len();
					new_pots.insert(length, '.');
				}
				new_pots[(z + 2) as usize] = '#';
			}
			x += 1;
		}
		pots = new_pots;
	}
	let mut sum:i32 = 0;
	for i in 0..pots.len()
	{
		if pots[i] == '#'
		{
			sum += i as i32 - zero_index;
		}
	}
	println!("{:?}", sum);
}
