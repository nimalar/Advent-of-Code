use std::env;
use std::fs::File;
use std::io::prelude::*;

fn program(mut p:Vec<i32>, input:i32) -> i32
{
	let mut i:usize = 0;
	let mut output = 0;
	while i < p.len()
	{
		// parameter mode
		let third_parameter = p[i] / 10000;
		let second_parameter = (p[i] - third_parameter * 10000) / 1000;
		let first_parameter = (p[i] - third_parameter * 10000 - second_parameter * 1000) / 100;
		let opcode = p[i] - third_parameter * 10000 - second_parameter * 1000 - first_parameter * 100;
		if opcode == 99
		{
			// halt
			break;
		}
		else
		{
			// always at least one parameter
			let mut first = p[i+1];
			if first_parameter == 0
			{
				first = p[p[i+1] as usize];
			}
			if opcode == 3
			{
				// input
				let index = p[i+1] as usize;
				p[index] = input;
				i += 2;
			}
			else if opcode == 4
			{
				// output
				output = first;
				i += 2;
			}			
			else
			{
				let mut second = p[i+2];
				if second_parameter == 0
				{
					second = p[p[i+2] as usize];
				}
				if (opcode == 5 && first != 0) || (opcode == 6 && first == 0)
				{
					i = second as usize;
				}
				else if opcode == 5 || opcode == 6
				{
					i += 3;
				}
				else
				{
					let index = p[i+3] as usize;
					if opcode == 1
					{
						// add
						p[index] = first + second;
					}
					else if opcode == 2
					{
						// multiply
						p[index] = first * second;
					}
					else if (opcode == 7 && first < second) || (opcode == 8 && first == second)
					{
						p[index] = 1;
					}
					else if opcode == 7 || opcode == 8
					{
						p[index] = 0;
					}	
					i += 4;
				}				
			}
		}
	}
	return output;
}


fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut positions:Vec<i32> = Vec::new();
	
	for value in contents.split(',')
	{
		positions.push(value.parse().unwrap());
	}
	
	let result = program(positions.to_vec(), 1);
	println!("First star: {}", result);
	
	let second_result = program(positions.to_vec(), 5);
	println!("Second star: {}", second_result);
}
