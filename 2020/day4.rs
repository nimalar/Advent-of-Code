use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn validate_passport(ref fields:&HashMap<&str, &str>) -> usize
{
	// years
	let byr:usize = (fields.get("byr").unwrap()).parse().unwrap();
	let iyr:usize = (fields.get("iyr").unwrap()).parse().unwrap();
	let eyr:usize = (fields.get("eyr").unwrap()).parse().unwrap();
	if byr < 1920 || byr > 2002 || iyr < 2010 || iyr > 2020 || eyr < 2020 || eyr > 2030
	{
		return 0;
	}
	
	// height
	let height = fields.get("hgt").unwrap();
	if height.contains("in")
	{
		let value = (&height[0..height.find("in").unwrap()]).parse::<usize>().unwrap();
		if value < 59 || value > 76
		{
			return 0;
		}
	}
	else if height.contains("cm")
	{
		let value = (&height[0..height.find("cm").unwrap()]).parse::<usize>().unwrap();
		if value < 150 || value > 193
		{
			return 0;
		}
	}
	else
	{
		return 0;
	}
	
	// hair colour
	let hair = fields.get("hcl").unwrap();
	if hair.len() != 7 || hair.find("#").unwrap() != 0
	{
		return 0;
	}
	for c in hair[1..].chars()
	{
		if c < '0' || (c > '9' && c < 'a') || c > 'f'
		{
			return 0;
		}
	}
	
	// eye colour
	if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(fields.get("ecl").unwrap())
	{
		return 0;
	}
	
	// passport id
	let pid = fields.get("pid").unwrap();
	if pid.len() != 9
	{
		return 0;
	}
	for c in pid.chars()
	{
		if c < '0' || c > '9'
		{
			return 0;
		}
	}	

	// everything ok
	return 1;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	contents += "\n";
	let mut fields:HashMap<&str, &str> = HashMap::new();
	let mut result_1 = 0;
	let mut result_2 = 0;
	let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

	for row in contents.lines()
	{
		if row.trim() == ""
		{
			// empty line, change of passport
			if fields.keys().len() >= 7
			{
				result_1 += 1;
				result_2 += validate_passport(&fields);
			}
			fields = HashMap::new();
		}
		else
		{
			let parts:Vec<&str> = row.split_whitespace().collect();
			for part in parts
			{
				let data:Vec<&str> = part.split(":").collect();
				if required_fields.contains(&data[0])
				{
					fields.insert(data[0], data[1]);
				}
			}
		}
	}
	
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
