use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

struct Field
{
	name : String,
	min : Vec<usize>,
	max : Vec<usize>
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut fields:Vec<Field> = Vec::new();
	let mut input_section = 0;
	let mut result_1 = 0;
	let mut my_ticket:Vec<usize> = Vec::new();
	let mut not_possible_fields:Vec<HashSet<usize>> = vec![HashSet::new(); 20];

	for row in contents.lines()
	{
		if row.trim() == "" || row.contains("ticket")
		{
			// empty line, change of section
			input_section += 1;
			continue;
		}

		// gather fields
		if input_section == 0
		{
			let parts:Vec<&str> = row.split(": ").collect();
			let numbers:Vec<&str> = parts[1].split(" or ").collect();
			let min:Vec<usize> = numbers[0].split("-").map(|x| x.parse::<usize>().unwrap()).collect();
			let max:Vec<usize> = numbers[1].split("-").map(|x| x.parse::<usize>().unwrap()).collect();
			fields.push(Field { name: parts[0].to_string(), min : min, max : max});
			continue;
		}
		
		//analyze ticket data
		let parts:Vec<usize> = row.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
		
		if input_section < 3
		{
			my_ticket = parts.to_vec();
		}
		
		'outer: for j in 0..parts.len()
		{
			let mut valid = false;
			for i in 0..fields.len()
			{
				if parts[j] >= fields[i].min[0] && parts[j] <= fields[i].max[1] && (parts[j] <= fields[i].min[1] || parts[j] >= fields[i].max[0])
				{
					if !valid
					{
						for k in 0..i
						{
							not_possible_fields[j].insert(k);
						}
					}
					valid = true;
				}
				else if valid
				{
					not_possible_fields[j].insert(i);
				}
			}
			if !valid
			{
				result_1 += parts[j];
			}
		}
	}
	
	let mut possible_fields:Vec<HashSet<usize>> = vec![HashSet::new(); 20];
	
	for i in 0..not_possible_fields.len()
	{
		for j in 0..fields.len()
		{
			if !not_possible_fields[i].contains(&j)
			{
				possible_fields[i].insert(j);
			}
		}
	}
	
	// loop until each field corresponds to just one field
	loop
	{
		let mut lengths = 0;
		for i in 0..possible_fields.len()
		{
			if possible_fields[i].len() == 1
			{
				lengths += 1;
				let vector = possible_fields.to_vec();
				let value:Vec<&usize> = vector[i].iter().collect();
				for j in 0..possible_fields.len()
				{
					if i != j
					{
						possible_fields[j].remove(&value[0]);
					}
				}
			}
		}
		if lengths == fields.len()
		{
			break;
		}
	}
	
	let mut result_2 = 1;
	for i in 0..fields.len()
	{
		let value:Vec<&usize> = possible_fields[i].iter().collect();
		if fields[*value[0]].name.contains("departure")
		{
			result_2 *= my_ticket[i];
		}
	}
		
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
