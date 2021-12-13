use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;

fn main() 
{	
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
	let mut result_1 = 0;
	let mut result_2 = 0;
	
	for row in contents.lines()
	{
		let parts:Vec<&str> = row.split(" | ").collect();
		let output:Vec<String> = parts[1].trim().split(' ').map(|value| { let mut x = value.chars().collect::<Vec<char>>(); x.sort(); x.into_iter().collect::<String>()}).collect();
		let mut all:Vec<String> = parts[0].trim().split(' ').map(|value| { let mut x = value.chars().collect::<Vec<char>>(); x.sort(); x.into_iter().collect::<String>()}).collect();
		all.append(&mut output.to_vec());
		let mut numbers:HashMap<usize, String> = HashMap::new();
		
		while numbers.len() < 10
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
				else if value.len() == 5 && numbers.contains_key(&7)
				{
					if numbers.get(&7).unwrap().chars().all(|x| value.contains(x))
					{
						numbers.insert(3, value);
					}
					else if numbers.contains_key(&6)
					{
						if value.chars().all(|x| numbers.get(&6).unwrap().contains(x))
						{
							numbers.insert(5, value);
						}
						else
						{
							numbers.insert(2, value);
						}
					}
				}
				else if value.len() == 6 && numbers.contains_key(&4)
				{
					if numbers.get(&4).unwrap().chars().all(|x| value.contains(x))
					{
						numbers.insert(9, value);
					}
					else if numbers.contains_key(&1)
					{
						if numbers.get(&1).unwrap().chars().all(|x| value.contains(x))
						{
							numbers.insert(0, value);
						}
						else
						{
							numbers.insert(6, value);
						}
					}
				}
			}
		}
		
		let mut multiplier = 10_usize.pow(output.len() as u32);
		for value in output
		{
			for (key, val) in numbers.iter()
			{
				if *val == value
				{
					if key == &1 || key == &4 || key == &7 || key == &8
					{
						result_1 += 1;
					}
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
