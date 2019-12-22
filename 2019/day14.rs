use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Reaction<'a>
{
	input: Vec<(&'a str, i64)>,
	output: i64,
}

fn main()
{	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut reactions:HashMap<&str, Reaction> = HashMap::new();
	let mut needed:Vec<(&str, i64)> = Vec::new();
	
	for row in contents.lines()
	{
		let mut parts:Vec<&str> = row.split(" => ").collect();
		let components:Vec<&str> = parts[0].split(", ").collect();
		let output:Vec<&str> = parts[1].split(" ").collect();
		let mut inputs:Vec<(&str, i64)> = Vec::new();
		for i in 0..components.len()
		{
			let input:Vec<&str> = components[i].split(" ").collect();
			inputs.push((input[1], input[0].parse().unwrap()));
		}
		reactions.insert(output[1], Reaction{input:inputs, output:output[0].parse().unwrap()});		
	}
	needed.push(("FUEL", 1));
	let mut ore:i64 = 0;
	let mut second_result = 1;
	'outer: loop
	{
		let mut zeroes = 0;
		for i in 0..needed.len()
		{
			if needed[i].0 != "ORE"
			{
				if needed[i].1 <= 0
				{
					zeroes += 1;
				}
				else
				{
					let reaction = reactions.get(needed[i].0).unwrap();
					let amount = (needed[i].1 as f64 / reaction.output as f64).ceil() as i64;
					for j in 0..reaction.input.len()
					{
						let mut present = false;
						for k in 0..needed.len()
						{
							if needed[k].0 == reaction.input[j].0
							{
								needed[k].1 += amount * reaction.input[j].1;
								present = true;
							}
						}
						if !present
						{
							needed.push((reaction.input[j].0, amount * reaction.input[j].1));
						}
					}
					needed[i].1 -= amount * reaction.output;
				}
			}
			else
			{
				ore = needed[i].1 as i64;
			}
			
		}
		if needed.len() - zeroes == 1
		{
			if second_result == 1
			{
				println!("First star: {}", ore);
			}
			if ore < 1000000000000
			{
				needed[0].1 += 1;
				second_result += 1;
			}
			else
			{
				println!("Second star: {}", second_result - 1);
				break 'outer;
			}
		}
	}
}
