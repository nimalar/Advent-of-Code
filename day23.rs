use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn main()
{
	part1();
	part2();
}

fn part1() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);
	let mut instructions:Vec<Vec<String>> = Vec::new();
	let mut register = HashMap::new();
	
	// read lines to vector
	for line in reader.lines()
	{
		let l = line.unwrap();
		let vec:Vec<&str> = l.split_whitespace().collect();
		let mut strings:Vec<String> = Vec::new();
		for i in 0..vec.len()
		{
			strings.push(vec[i].to_string());
		}
		instructions.push(strings);
	}
	
	let mut index:i64 = 0;
	let mut summa = 0;
	
	// start jumping
	while index >= 0 && index < instructions.len() as i64
	{
		if instructions[index as usize][0] == "jnz"
		{
			// determine if jump
			let value = &instructions[index as usize][1];
			let mut result = 0;
			match value.parse::<i64>()
			{
				Ok(n) => result = n,
				Err(e) => result = *register.entry(value).or_insert(0 as i64),
			}			
			if result != 0
			{
				let jump = &instructions[index as usize][2];
				match jump.parse::<i64>()
				{
					Ok(n) => index += n,
					Err(e) => index += *register.entry(value).or_insert(0 as i64),
				}			
			}
			else
			{
				index += 1;
			}
		}
		else 
		{
			let mut inserted = &instructions[index as usize][2];
			let mut new_value = 0;
			match inserted.parse::<i64>()
			{
				Ok(n) => new_value = n,
				Err(e) => new_value = *register.entry(inserted).or_insert(0 as i64),
			}
			let value = register.entry(&instructions[index as usize][1]).or_insert(0 as i64);
			
			if instructions[index as usize][0] == "set"
			{	
				*value = new_value;
			}
			else if instructions[index as usize][0] == "sub"
			{	
				*value -= new_value;
			}
			else if instructions[index as usize][0] == "mul"
			{	
				*value *= new_value;
				summa += 1;
			}
			else if instructions[index as usize][0] == "mod"
			{	
				*value = *value % new_value;
			}
			index += 1;
		} 
	}
	println!("{}", summa);
}

fn part2() {	
	let mut b = 57;
	b *= 100;
	b += 100000;
	let mut c = b + 17000;
	let mut d = 0;
	let mut f = 0;
	let mut h = 0;

	loop
	{
		f = 1;
		d = 2;
		loop
		{
			if b % d == 0
			{
				f = 0;
				break;
			}
			d += 1;
			if d == b || f == 0
			{
				break;
			}
		}
		if f == 0
		{
			h = h + 1;
		}
		if b - c == 0
		{
			break;
		}
		else
		{
			b = b + 17;
		}	
	}
	println!("{}", h);
}
