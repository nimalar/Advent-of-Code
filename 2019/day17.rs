use std::env;
use std::fs::File;
use std::io::prelude::*;

fn make_room(ref mut p:&mut Vec<i64>, index:usize) -> i64
{
	if index >= p.len()
	{
		p.resize(index + 1, 0);
	}
	return p[index];
}

fn program(ref mut p:&mut Vec<i64>, ref mut input_signals:&mut Vec<i64>, ref mut output_signals:&mut Vec<i64>, mut i:usize, relative_base:&mut i64) -> usize
{
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
			i = p.len();
			break;
		}
		else
		{
			// always at least one parameter
			let mut first = p[i+1];
			if first_parameter == 0
			{
				first = make_room(p, first as usize);
			}
			else if first_parameter == 2
			{
				first = make_room(p, (first + *relative_base) as usize);
			}
			if opcode == 3
			{
				// input
				let mut index = p[i+1];
				if first_parameter == 0
				{
					make_room(p, index as usize);
				}
				else if first_parameter == 2
				{
					make_room(p, (index + *relative_base) as usize);
					index = index + *relative_base;
				}
				if input_signals.len() > 0
				{
					p[index as usize] = input_signals.remove(0);
					i += 2;
				}
				else
				{
					// continue later from this same index
					return i;
				}
			}
			else if opcode == 4
			{
				// output
				output_signals.push(first);
				i += 2;
			}			
			else if opcode == 9
			{
				*relative_base += first;
				i += 2;
			}
			else
			{
				let mut second = p[i+2];
				if second_parameter == 0
				{
					second = make_room(p, second as usize);
				}
				else if second_parameter == 2
				{
					second = make_room(p, (second + *relative_base) as usize);
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
					let mut index = p[i+3];
					if third_parameter == 0
					{
						make_room(p, index as usize);
					}
					else if third_parameter == 2
					{
						make_room(p, (index + *relative_base) as usize);
						index = p[i+3] + *relative_base;
					}
					if opcode == 1
					{
						// add
						p[index as usize] = first + second;
					}
					else if opcode == 2
					{
						// multiply
						p[index as usize] = first * second;
					}
					else if (opcode == 7 && first < second) || (opcode == 8 && first == second)
					{
						p[index as usize] = 1;
					}
					else if opcode == 7 || opcode == 8
					{
						p[index as usize] = 0;
					}	
					i += 4;
				}				
			}
		}
	}
	return i;
}

fn main()
{	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut positions:Vec<i64> = Vec::new();
	let mut scaffolds:Vec<Vec<char>> = Vec::new();
	
	for value in contents.split(',')
	{
		positions.push(value.parse().unwrap());
	}
	
	let mut input = Vec::new();
	let mut output = Vec::new();
	let mut base = 0;
	
	// scan first
	program(&mut positions.to_vec(), &mut input, &mut output, 0, &mut base);
	let mut row:Vec<char> = Vec::new();
	while output.len() > 0
	{
		let result = output.remove(0) as u8 as char;
		//print!("{}", result);
		if result == '\n' && row.len() > 0
		{
			scaffolds.push(row.to_vec());
			row.clear();
		}
		else
		{
			row.push(result);
		}
	}
	
	let mut result = 0;
	for i in 1..scaffolds.len()-1
	{
		for j in 1..scaffolds[i].len()-1
		{
			if scaffolds[i][j] == '#' && scaffolds[i-1][j] == '#' && scaffolds[i+1][j] == '#' && scaffolds[i][j-1] == '#' && scaffolds[i][j+1] == '#'
			{
				result += i * j;
			}
		}
	}
	println!("First star: {}", result);
	
	// movement
	let mut commands = positions.to_vec();
	commands[0] = 2;
	let mut base = 0;
	
	// inputs calculated with advanced image recognition algorithm
	// R6 R6 R8 L10 L4 R6 L10 R8 R6 L10 R8 R6 R6 R8 L10 L4 L4 L12 R6 L10 R6 R6 R8 L10 L4 L4 L12 R6 L10 R6 R6 R8 L10 L4 L4 L12 R6 L10 R6 L10 R8
	// A: R6 R6 R8 L10 L4
	// B: R6 L10 R8
	// C: L4 L12 R6 L10
	let routine = "A,B,B,A,C,A,C,A,C,B\n";
	let a = "R,6,R,6,R,8,L,10,L,4\n";
	let b = "R,6,L,10,R,8\n";
	let c = "L,4,L,12,R,6,L,10\n";
	let video = "n\n";
	let char_vec = vec![routine, a, b, c, video];
	
	for row in 0..char_vec.len()
	{
		for c in char_vec[row].chars()
		{
			input.push(c as u8 as i64);
		}
	}
	program(&mut commands, &mut input, &mut output, 0, &mut base);
	
	println!("Second star: {:?}", output.pop().unwrap());
}
