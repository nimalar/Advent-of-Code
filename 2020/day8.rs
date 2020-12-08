use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Instruction
{
	operation : String,
	argument : i32,
}

fn execute(ref instructions:&Vec<Instruction>, mut index:i32, acc:&mut i32) -> bool
{
	let mut already_executed:Vec<usize> = Vec::new();
	while index < instructions.len() as i32
	{
		if already_executed.contains(&(index as usize))
		{
			return false;
		}
		already_executed.push(index as usize);
		if instructions[index as usize].operation == "jmp"
		{
			index += instructions[index as usize].argument;
			continue;
		}
		else if instructions[index as usize].operation == "acc"
		{
			*acc += instructions[index as usize].argument;
		}
		index += 1;
	}
	return true;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut instructions:Vec<Instruction> = Vec::new();
	
	for row in contents.lines()
	{
		
		let parts:Vec<&str> = row.trim().split_whitespace().collect();
		instructions.push(Instruction{operation : parts[0].to_string(), argument : parts[1].parse().unwrap() });
	}
	let mut acc = 0;	
	let mut result = execute(&instructions, 0, &mut acc);
	println!("First star: {}", acc);
	
	let mut index = 0;
	while !result
	{
		let mut instructions_copy = instructions.to_vec();
		while index < instructions_copy.len()
		{
			if instructions_copy[index].operation == "jmp"
			{
				instructions_copy[index].operation = "nop".to_string();
			}
			else if instructions_copy[index].operation == "nop"
			{
				instructions_copy[index].operation = "jmp".to_string();
			}
			else
			{
				index += 1;
				continue;
			}
			acc = 0;
			result = execute(&instructions_copy, 0, &mut acc);
			index += 1;
			break;
		}
	}
	println!("Second star: {}", acc);
}
