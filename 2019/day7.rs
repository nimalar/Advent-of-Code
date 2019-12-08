use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn program(ref mut p:&mut Vec<i32>, ref mut input_signals:&mut Vec<i32>, ref mut output_signals:&mut Vec<i32>, mut i:usize) -> usize
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
				first = p[p[i+1] as usize];
			}
			if opcode == 3
			{
				// input
				let index = p[i+1] as usize;
				if input_signals.len() > 0
				{
					p[index] = input_signals.remove(0);
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
	return i;
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
	
	let mut result = 0;
	let mut second_result = 0;
	for i in 0..2
	{
		for a in 0..5
		{
			for b in (0..5).filter(|x| x != &a)
			{
				for c in (0..5).filter(|x| x != &a && x != &b)
				{			
					for d in (0..5).filter(|x| x != &a && x != &b && x != &c)
					{
						for e in (0..5).filter(|x| x != &a && x != &b && x != &c && x != &d)
						{
							let mut a_out = vec![b + i*5];
							let mut b_out = vec![c + i*5];
							let mut c_out = vec![d + i*5];
							let mut d_out = vec![e + i*5];
							let mut e_out = vec![a + i*5, 0];
							let mut a_vec = positions.to_vec();
							let mut b_vec = positions.to_vec();
							let mut c_vec = positions.to_vec();
							let mut d_vec = positions.to_vec();
							let mut e_vec = positions.to_vec();
							let mut a_i = 0;
							let mut b_i = 0;
							let mut c_i = 0;
							let mut d_i = 0;
							let mut e_i = 0;
							
							while e_i < positions.len()	
							{
								a_i = program(&mut a_vec, &mut e_out, &mut a_out, a_i);
								b_i = program(&mut b_vec, &mut a_out, &mut b_out, b_i);
								c_i = program(&mut c_vec, &mut b_out, &mut c_out, c_i);
								d_i = program(&mut d_vec, &mut c_out, &mut d_out, d_i);
								e_i = program(&mut e_vec, &mut d_out, &mut e_out, e_i);
							}
							if i == 0
							{
								result = cmp::max(result, e_out.pop().unwrap());
							}
							else
							{
								second_result = cmp::max(second_result, e_out.pop().unwrap());
							}
						}
					}
				}
			}
		}
	}
	
	println!("First star: {}", result);
	println!("Second star: {}", second_result);
}
