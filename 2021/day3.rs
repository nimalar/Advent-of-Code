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
	
	let mut numbers:Vec<Vec<char>> = Vec::new();
	let mut gamma = String::new();
	let mut epsilon = String::new();
	let mut oxygen = String::new();
	let mut co2 = String::new();
	let mut oxy_skip:Vec<usize> = Vec::new();
	let mut co2_skip:Vec<usize> = Vec::new();
	let mut oxy_prev = '0';
	let mut co2_prev = '0';
	
	for row in contents.lines()
	{
		numbers.push(row.chars().collect());
	}

	for i in 0..numbers[0].len()
	{
		let mut ones = 0;
		let mut oxy_ones = 0;
		let mut co2_ones = 0;
		for j in 0..numbers.len()
		{
			if !oxy_skip.contains(&j) 
			{
				if i > 0 && numbers[j][i-1] != oxy_prev
				{
					oxy_skip.push(j);
				}
				else
				{
					if numbers[j][i] == '1'
					{
						oxy_ones += 1;
					}
					oxygen = numbers[j].iter().collect();
				}
			}
			if !co2_skip.contains(&j) 
			{
				if i > 0 && numbers[j][i-1] != co2_prev
				{
					co2_skip.push(j);
				}
				else
				{					
					if numbers[j][i] == '1'
					{
						co2_ones += 1;
					}
					co2 = numbers[j].iter().collect();
				}
			}
			if numbers[j][i] == '1'
			{
				ones += 1;
			}
		}
		if ones > numbers.len() / 2
		{
			gamma.push('1');
			epsilon.push('0');
		}
		else
		{
			gamma.push('0');
			epsilon.push('1');
		}
		oxy_prev = '0';
		if oxy_ones * 2 >= (numbers.len() - oxy_skip.len())
		{
			oxy_prev = '1';
		}
		co2_prev = '0';
		if co2_ones * 2 < (numbers.len() - co2_skip.len())
		{
			co2_prev = '1';
		}
	}
	println!("First star: {}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap());
	println!("Second star: {}", isize::from_str_radix(&oxygen, 2).unwrap() * isize::from_str_radix(&co2, 2).unwrap());
}
