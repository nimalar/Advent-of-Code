use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

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

fn paint(mut commands:Vec<i64>, starting_panel:i32) -> Vec<Vec<i64>>
{
	let max = 140;
	let mut grid:Vec<Vec<i64>> = vec![vec![-1; max]; max];
	let mut position = (max/2, max/2);
	let mut direction = (0i32, -1i32);
	let mut input = Vec::new();
	let mut output = Vec::new();
	let mut i = 0;
	let mut base = 0;
	
	if starting_panel == 1
	{
		grid[position.1][position.0] = 1;
	}
	
	while i < commands.len()	
	{
		input.push(cmp::max(0, grid[position.1][position.0]));
		i = program(&mut commands, &mut input, &mut output, i, &mut base);
		
		grid[position.1][position.0] = output.remove(0);
		let movement = output.remove(0);
		if movement == 0
		{
			direction = (direction.1, -1 * direction.0);
		}
		else
		{
			direction = (-1 * direction.1, direction.0);
		}
		position = ((position.0 as i32 + direction.0) as usize, (position.1 as i32 + direction.1) as usize);
	}
	return grid;
}

fn main()
{	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut positions:Vec<i64> = Vec::new();
	
	for value in contents.split(',')
	{
		positions.push(value.parse().unwrap());
	}
	
	let mut result = 0;
	let grid = paint(positions.to_vec(), 0);
	
	for i in 0..grid.len()
	{
		for j in 0..grid[i].len()
		{
			if grid[i][j] >= 0
			{
				result += 1;
			}
		}
	}
	println!("First star: {}", result);
	println!("Second star:");
	
	let second_grid = paint(positions.to_vec(), 1);
	for k in 0..second_grid.len()
	{
		let mut second_result = String::from("");
		for l in 0..second_grid[k].len()
		{
			if second_grid[k][l] == 1
			{
				second_result.push_str("#");
			}
			else
			{
				second_result.push_str(" ");
			}
		}
		if second_result.contains("#")
		{
			println!("{}", second_result);
		}
	}
}
