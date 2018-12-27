use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_result(i:&str, registers:&[usize], params:&[usize]) -> Vec<usize>
{
	let mut temp = registers.to_vec();
	match i
	{
		// addr
		"addr" => temp[params[3]] = registers[params[1]] + registers[params[2]],
		// addi
		"addi" => temp[params[3]] = registers[params[1]] + params[2],
		// mulr
		"mulr" => temp[params[3]] = registers[params[1]] * registers[params[2]],
		// muli
		"muli" => temp[params[3]] = registers[params[1]] * params[2],
		// banr
		"banr" => temp[params[3]] = registers[params[1]] & registers[params[2]],
		// bani
		"bani" => temp[params[3]] = registers[params[1]] & params[2],
		// borr
		"borr" => temp[params[3]] = registers[params[1]] | registers[params[2]],
		// bori
		"bori" => temp[params[3]] = registers[params[1]] | params[2],
		// setr
		"setr" => temp[params[3]] = registers[params[1]],
		// seti
		"seti" => temp[params[3]] = params[1],
		// gtir
		"gtir" => temp[params[3]] = (params[1] > registers[params[2]]) as usize,
		// gtri
		"gtri" => temp[params[3]] = (registers[params[1]] > params[2]) as usize,
		// gtrr
		"gtrr" => temp[params[3]] = (registers[params[1]] > registers[params[2]]) as usize,
		// eqir
		"eqir" => temp[params[3]] = (params[1] == registers[params[2]]) as usize,
		// eqri
		"eqri" => temp[params[3]] = (registers[params[1]] == params[2]) as usize,
		// eqrr
		"eqrr" => temp[params[3]] = (registers[params[1]] == registers[params[2]]) as usize,
		// #ip
		"#ip" => (),
		_ => (),
	}
	//*pointer += 1;
	return temp;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut registers:Vec<usize> = vec![0; 6];
	let mut opcode:&str;
	let mut params:Vec<usize> = vec![0; 4]; // opcode, A, B, C
	let mut instruction_pointer = 0;
	let mut pointer_register = 0;
	let instructions:Vec<&str> = contents.lines().collect();
	let mut line = 0;
	
	while line < instructions.len()
	{
		let parts:Vec<&str> = instructions[line].split_whitespace().collect();
		
		opcode = parts[0];
		for i in 1..parts.len()
		{
			params[i] = parts[i].parse().unwrap();
		}
		if opcode == "#ip"
		{
			pointer_register = params[1];
			instruction_pointer = 0;
			line = instruction_pointer + 1;
			continue;
		}
		registers[pointer_register] = instruction_pointer;
		registers = get_result(opcode, &registers, &params);
		instruction_pointer = registers[pointer_register];
		instruction_pointer += 1;
		line = instruction_pointer + 1;
	}
	println!("{:?}", registers[0]);
}
