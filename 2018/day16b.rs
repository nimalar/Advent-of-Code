use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_result(i:usize, registers:&[usize], params:&[usize]) -> Vec<usize>
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
	return temp;
}

fn its_a_match(registers:&[usize], params:&[usize], result:&[usize], opcodes:&mut Vec<Vec<(usize)>>)
{
	let mut matches:Vec<usize> = Vec::new();
	
	for i in 0..16
	{
		let temp = get_result(i, &registers, &params);
	
		if temp == result
		{
			matches.push(i);
		}
	}
	
	if opcodes[params[0]].is_empty()
	{
		// first time, all are possible
		opcodes[params[0]] = matches;
	}
	else
	{	
		// check which are the same and ditch the others
		opcodes[params[0]].retain(|&x| matches.contains(&x));
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
	let mut opcodes:Vec<Vec<usize>> = vec![Vec::new(); 16];
	let mut rules = true;
	
	for line in contents.lines()
	{
		if !line.is_empty() && rules
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

				// calculate matches
				
				its_a_match(&registers, &params, &result, &mut opcodes);
			}
			else
			{
				for i in 0..parts.len()
				{
					params[i] = parts[i].parse().unwrap();
				}
			}
		}
		else if rules
		{
			empty_lines += 1;
			if empty_lines > 2
			{
				// start of test program, time to allocate commands to opcodes
				// loop until all opcodes are allocated to one command
				let mut used:Vec<usize> = Vec::new();
				while used.len() < 16
				{
					for i in 0..opcodes.len()
					{
						if opcodes[i].len() > 1
						{
							// ditch already used
							opcodes[i].retain(|&x| !used.contains(&x));
						}
						// if only one, it is the one
						else if !used.contains(&opcodes[i][0])
						{
							used.push(opcodes[i][0]);
						}
					}
				}
				rules = false;
				registers = vec![0; 4];
			}
		}
		else
		{
			// test program
			let parts:Vec<&str> = line.split_whitespace().collect();
			for i in 0..parts.len()
			{
				params[i] = parts[i].parse().unwrap();
			}
			registers = get_result(opcodes[params[0]][0], &registers, &params);
			
		}
	}
	println!("{:?}", registers[0]);
}
