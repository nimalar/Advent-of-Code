use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn get_matches(ref rules:&HashMap<usize, Vec<Vec<usize>>>, ref letters:&HashMap<usize, char>, rule_vec:Vec<usize>, ref mut tested:&mut HashSet<Vec<usize>>, ref mut fabulous:&mut HashSet<Vec<char>>) -> usize
{
	let mut fab_vec:Vec<char> = Vec::new();
	for i in 0..rule_vec.len()
	{
		if !letters.contains_key(&rule_vec[i])
		{
			break;
		}
		fab_vec.push(*letters.get(&rule_vec[i]).unwrap());
	}
	if fab_vec.len() == rule_vec.len()
	{
		fabulous.insert(fab_vec);
		return 1;
	}

	for i in 0..rule_vec.len()
	{	
		if letters.contains_key(&rule_vec[i])
		{
			continue;
		}
		let subrules = rules.get(&rule_vec[i]).unwrap();
		
		'outer: for subrule in subrules
		{
			let mut rule_vec_copy = rule_vec.to_vec();
			for j in 0..subrule.len()
			{
				if j == 0
				{
					rule_vec_copy[i] = subrule[j];
					continue;
				}
				rule_vec_copy.insert(i + j, subrule[j]);
			}
						
			if !tested.contains(&rule_vec_copy)
			{
				tested.insert(rule_vec_copy.to_vec());
				get_matches(rules, letters, rule_vec_copy.to_vec(), *tested, *fabulous);
			}
			else
			{
				return 0;
			}
		}
	}
	return 0;
}

fn match_loops(row:&[char], ref fabulous:&HashSet<Vec<char>>, ref rule42:&HashSet<Vec<char>>, ref rule31:&HashSet<Vec<char>>, removed_31:usize, removed_42:usize) -> usize
{
	if fabulous.contains(row) && removed_42 >= removed_31
	{
		return 1;
	}
	else
	{
		let mut result = 0;
		for i in 0..row.len()
		{
			if rule42.contains(&row[..i])
			{
				result += match_loops(&row[i..], &fabulous, &rule42, &rule31, removed_31, removed_42 + 1);
			}			
			if rule31.contains(&row[i..])
			{
				result += match_loops(&row[..i], &fabulous, &rule42, &rule31, removed_31 + 1, removed_42);
			}
		}
		if result > 0
		{
			return 1;
		}
	}
	return 0;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut rules:HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
	let mut letters:HashMap<usize, char> = HashMap::new();
	let mut fabulous:HashSet<Vec<char>> = HashSet::new();
	let mut rule42:HashSet<Vec<char>> = HashSet::new();
	let mut rule31:HashSet<Vec<char>> = HashSet::new();
	let mut input_section = 0;
	let mut result_1 = 0;
	let mut result_2 = 0;
	
	for row in contents.lines()
	{
		if row.trim() == ""
		{
			// empty line, all rules collected
			get_matches(&rules, &letters, vec![0], &mut HashSet::new(), &mut fabulous);
			get_matches(&rules, &letters, vec![42], &mut HashSet::new(), &mut rule42);
			get_matches(&rules, &letters, vec![31], &mut HashSet::new(), &mut rule31);
			input_section += 1;
			continue;
		}

		// gather rules
		if input_section == 0
		{
			let parts:Vec<&str> = row.split(": ").collect();
			if parts[1].contains("\"")
			{
				letters.insert(parts[0].parse().unwrap(), parts[1].trim_matches('\"').chars().next().unwrap());
				continue;
			}
			let subrule_parts:Vec<&str> = parts[1].split(" | ").collect();
			let mut subrule_vec:Vec<Vec<usize>> = Vec::new();
			for part in subrule_parts
			{
				subrule_vec.push(part.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());
				
			}
			rules.insert(parts[0].parse().unwrap(), subrule_vec);
			continue;
		}
		
		if fabulous.contains(&row.trim().chars().collect::<Vec<char>>())
		{
			result_1 += 1;
			result_2 += 1;
		}
		else
		{
			result_2 += match_loops(&row.trim().chars().collect::<Vec<char>>()[..], &fabulous, &rule42, &rule31, 0, 0);
		}
	}
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
