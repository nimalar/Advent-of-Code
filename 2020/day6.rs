use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	contents += "\n";
	let mut result_1 = 0;
	let mut result_2 = 0;
	let mut anyone:Vec<char> = Vec::new();
	let mut everyone:Vec<char> = Vec::new();
	let mut first = true;
	
	for row in contents.lines()
	{
		if row.trim() == ""
		{
			// empty line, change of group
			result_2 += everyone.len();
			anyone = Vec::new();
			first = true;
		}
		else
		{
			for c in row.trim().chars()
			{
				if !anyone.contains(&c)
				{
					anyone.push(c);
					result_1 += 1;
				}
			}
			if first
			{
				everyone = row.trim().chars().collect();
			}
			else
			{
				let mut i = 0;
				while i < everyone.len()
				{
					if !row.contains(everyone[i])
					{
						everyone.remove(i);
						continue;
					}
					i += 1;
				}
			}
			first = false;
		}
	}
	
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
