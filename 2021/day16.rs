use std::env;
use std::fs::read_to_string;

fn packet(binary:&[char], i:&mut usize, count_packets:bool, mut subpackets:u32, values:&mut Vec<u64>) -> u32
{
	let mut result = 0;
	while (count_packets && subpackets > 0) || (!count_packets && *i < binary.len())
	{
		let version = u32::from_str_radix(&binary[*i..*i+3].into_iter().collect::<String>(), 2).unwrap();
		let id = u32::from_str_radix(&binary[*i+3..*i+6].into_iter().collect::<String>(), 2).unwrap();
		result += version;
		*i += 6;
		// literal
		if id == 4
		{
			let mut literal:Vec<char> = Vec::new();
			loop
			{
				let starter = binary[*i];
				literal.extend_from_slice(&binary[*i+1..*i+5]);
				*i += 5;
				if starter == '0'
				{
					values.push(u64::from_str_radix(&literal.into_iter().collect::<String>(), 2).unwrap());
					break;
				}
			}
		}
		// operator
		else
		{
			let length = binary[*i];
			let mut new_values:Vec<u64> = Vec::new();
			let mut index = 0;
			if length == '0'
			{
				// total length in bits
				let total_length = u32::from_str_radix(&binary[*i+1..*i+16].into_iter().collect::<String>(), 2).unwrap();
				*i += 16;
				result += packet(&binary[*i..*i+total_length as usize], &mut index, false, subpackets, &mut new_values);
			}
			else
			{
				// number of subpackets
				let new_subpackets = u32::from_str_radix(&binary[*i+1..*i+12].into_iter().collect::<String>(), 2).unwrap();
				*i += 12;
				result += packet(&binary[*i..], &mut index, true, new_subpackets, &mut new_values);
			}
			*i += index;
			if id == 0
			{
				values.push(new_values.iter().sum());
			}
			else if id == 1
			{
				let mut multiply = 1;
				for value in new_values
				{
					multiply *= value;
				}
				values.push(multiply);
			}
			else if id == 2
			{
				values.push(*new_values.iter().min().unwrap());
			}
			else if id == 3
			{
				values.push(*new_values.iter().max().unwrap());
			}
			else if (id == 5 && new_values[0] > new_values[1]) 
					|| (id == 6 && new_values[0] < new_values[1])
					|| (id == 7 && new_values[0] == new_values[1])
			{
				values.push(1);
			}
			else
			{
				values.push(0);	
			}
		}
		if subpackets > 0 && count_packets
		{
			subpackets -= 1;
		}
	}
	return result;
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");
	let mut binary:Vec<char> = Vec::new();
	
	for c in contents.chars()
	{
		let value = format!("{:01$b}", c.to_digit(16).unwrap(), 4);
		binary.append(&mut value.chars().collect::<Vec<char>>());
	}
	let mut result = Vec::new();
	
	println!("First star: {}", packet(&binary[..], &mut 0, true, 1, &mut result));
	println!("Second star: {}", result[0]);
}
