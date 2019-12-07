use std::env;
use std::fs::File;
use std::io::prelude::*;

fn its_a_match(i:usize, registers:&[usize], params:&[usize], result:&[usize]) -> i32
{
	let mut temp = registers.to_vec();	
	
	match i
	{
		// addr
		0 => temp[params[3]] = registers[params[1]] + registers[params[2]],
		// addi
		1 => temp[params[3]] = registers[params[1]] + params[2],
		// mulr
		2 => temp[params[3]] = registers[params[1]] * registers[params[2]],
		// muli
		3 => temp[params[3]] = registers[params[1]] * params[2],
		// banr
		4 => temp[params[3]] = registers[params[1]] & registers[params[2]],
		// bani
		5 => temp[params[3]] = registers[params[1]] & params[2],
		// borr
		6 => temp[params[3]] = registers[params[1]] | registers[params[2]],
		// bori
		7 => temp[params[3]] = registers[params[1]] | params[2],
		// setr
		8 => temp[params[3]] = registers[params[1]],
		// seti
		9 => temp[params[3]] = params[1],
		// gtir
		10 => temp[params[3]] = (params[1] > registers[params[2]]) as usize,
		// gtri
		11 => temp[params[3]] = (registers[params[1]] > params[2]) as usize,
		// gtrr
		12 => temp[params[3]] = (registers[params[1]] > registers[params[2]]) as usize,
		// eqir
		13 => temp[params[3]] = (params[1] == registers[params[2]]) as usize,
		// eqri
		14 => temp[params[3]] = (registers[params[1]] == params[2]) as usize,
		// eqrr
		15 => temp[params[3]] = (registers[params[1]] == registers[params[2]]) as usize,
		_ => (),
	}
	
	if temp == result
	{
		return 1;
	}
	else
	{
		return 0;
	}
	
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut registers:Vec<usize> = vec![0; 4];
	let mut result:Vec<usize> = vec![0; 4];
	let mut empty_lines = 0;
	let mut params:Vec<usize> = vec![0; 4]; // opcode, A, B, C
	let mut answer = 0;
	
	for line in contents.lines()
	{
		if !line.is_empty()
		{
			empty_lines = 0;
			let parts:Vec<&str> = line.split_whitespace().collect();
			if parts[0] == "Before:"
			{
				registers[0] = parts[1][1..2].parse().unwrap();
				registers[1] = parts[2][0..1].parse().unwrap();
				registers[2] = parts[3][0..1].parse().unwrap();
				registers[3] = parts[4][0..1].parse().unwrap();
			}
			else if parts[0] == "After:"
			{
				result[0] = parts[1][1..2].parse().unwrap();
				result[1] = parts[2][0..1].parse().unwrap();
				result[2] = parts[3][0..1].parse().unwrap();
				result[3] = parts[4][0..1].parse().unwrap();
					
				let mut matches = 0;
				
				// calculate matches
				
				for i in 0..16
				{
					matches += its_a_match(i, &registers, &params, &result);
					if matches == 3
					{
						answer += 1;
						break;
					}
				}
			}
			else
			{
				for i in 0..parts.len()
				{
					params[i] = parts[i].parse().unwrap();
				}
			}
		}
		else
		{
			empty_lines += 1;
			if empty_lines > 2
			{
				// start of test program
				break;
			}
		}
	}
	println!("{}", answer)
}
