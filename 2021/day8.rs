use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut result_1 = 0;
	let mut result_2 = 0;
	
	for row in contents.lines()
	{
		let parts:Vec<&str> = row.split(" | ").collect();
		let output:Vec<String> = parts[1].trim().split(' ').map(|value| { let mut x = value.chars().collect::<Vec<char>>(); x.sort(); x.into_iter().collect::<String>()}).collect();
		let mut all:Vec<String> = parts[0].trim().split(' ').map(|value| { let mut x = value.chars().collect::<Vec<char>>(); x.sort(); x.into_iter().collect::<String>()}).collect();
		all.append(&mut output.to_vec());
		let mut numbers:HashMap<usize, String> = HashMap::new();
		loop
		{
			for value in all.to_vec()
			{
				if value.len() == 2
				{
					numbers.insert(1, value);
				}
				else if value.len() == 4
				{
					numbers.insert(4, value);
				}
				else if value.len() == 7
				{
					numbers.insert(8, value);
				}
				else if value.len() == 3
				{
					numbers.insert(7, value);
				}
				else if value.len() == 5
				{
					if numbers.contains_key(&7)
					{
						let compare_seven = numbers.get(&7).unwrap();
						if compare_seven.chars().all(|x| value.contains(x))
						{
							numbers.insert(3, value);
						}
						else
						{
							if numbers.contains_key(&6)
							{
								let compare_six = numbers.get(&6).unwrap();
								if value.chars().all(|x| compare_six.contains(x))
								{
									numbers.insert(5, value);
								}
								else
								{
									numbers.insert(2, value);
								}
							}
						}
					}
				}
				else if value.len() == 6
				{
					if numbers.contains_key(&3)
					{
						let compare_three = numbers.get(&3).unwrap();
						if compare_three.chars().all(|x| value.contains(x))
						{
							numbers.insert(9, value);
						}
						else
						{
							if numbers.contains_key(&1)
							{
								let compare_one = numbers.get(&1).unwrap();
								if !compare_one.chars().all(|x| value.contains(x))
								{
									numbers.insert(6, value);
								}
								else
								{
									numbers.insert(0, value);
								}
							}
						}
					}
				}
			}
			if numbers.len() == 10
			{
				break;
			}
		}
		
		let mut multiplier = 1000;
		for value in output
		{
			let x = value.len();
			if x == 2 || x == 3 || x == 4 || x == 7
			{
				result_1 += 1;
			}
			for (key, val) in numbers.iter()
			{
				if *val == value
				{
					result_2 += multiplier * key;
					multiplier /= 10;
					break;
				}
			}
		}
	}
	
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
