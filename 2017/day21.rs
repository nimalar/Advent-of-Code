use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);

	let mut rules:Vec<Vec<Vec<String>>> = Vec::new();
	// read the rules into vector
	for line in reader.lines()
	{
		let l = line.unwrap();
		let parts:Vec<&str> = l.split(" => ").collect();
		let mut rule:Vec<Vec<String>> = Vec::new();
		for i in 0..2
		{
			let lines:Vec<&str> = parts[i].split('/').collect();
			let mut one_part:Vec<String> = Vec::new();
			for j in 0..lines.len()
			{
				one_part.push(lines[j].to_string());
			}
			rule.push(one_part);
		}
		rules.push(rule);
	}
	let mut input = vec![".#.".to_string(),"..#".to_string(),"###".to_string()];
	// loop this shit 18 times
	for i in 0..18
	{
		let mut value = 0;
		let mut size = 0;
		//divisible by 2
		if input.len() % 2 == 0
		{
			value = 2;
			size = input.len() / 2;
		}
		//divisible by 3
		else
		{	
			value = 3;
			size = input.len() / 3;
		}
		let mut result:Vec<String> = vec!["".to_string(); (value+1)*size];
		// all squares
		let mut x = 0;
		let mut y = 0;
		for j in 0..size.pow(2)
		{
			// all different possibilities * 8
			let mut all_chars:Vec<&str> = Vec::new();
			for k in 0..value
			{
				let chars:&str = &input[y+k][x..x+value];
				all_chars.push(chars);
			}
			let mut reversed = all_chars.to_vec();
			reversed.reverse();
			let mut mirrored:Vec<String> = Vec::new();
			for n in 0..value
			{
				mirrored.push(all_chars[n].chars().rev().collect());
			}
			let mut reversemirror = mirrored.to_vec();
			reversemirror.reverse();
			let mut vec:Vec<Vec<char>> = Vec::new();
			for n in 0..value
			{
				vec.push(all_chars[n].chars().collect());
			}
			let mut right = vec.to_vec();
			for a in 0..vec.len()
			{
				for b in 0..vec[a].len()
				{
					right[b][a] = vec[a][b];
				}
			}
			let mut finaali:Vec<String> = Vec::new();
			for n in 0..value
			{
				finaali.push(right[n].iter().collect());
			}
			let mut finaalireversed = finaali.to_vec();
			finaalireversed.reverse();
			let mut finaalimirrored:Vec<String> = Vec::new();
			for n in 0..value
			{
				finaalimirrored.push(finaali[n].chars().rev().collect());
			}
			let mut finaalireversemirror = finaalimirrored.to_vec();
			finaalireversemirror.reverse();
			for z in 0..rules.len()
			{
				if rules[z][0].len() == value && (all_chars == rules[z][0]
					|| reversed == rules[z][0] || mirrored == rules[z][0] ||
					reversemirror == rules[z][0] || finaali == rules[z][0]
					|| finaalireversed == rules[z][0] 
					|| finaalireversemirror == rules[z][0] 
					|| finaalimirrored == rules[z][0])
				{
					// time to match
					for h in 0..value+1
					{
						result[y/value * (value+1) + h].push_str(&rules[z][1][h]);
					}
				}
			}
			x = (x + value) % input.len();
			if x == 0
			{
				y += value;
			}
		}
		input = result.to_vec();
	}
	
	let mut summa = 0;
	for line in input
	{
		for char in line.chars()
		{
			if char == '#'
			{
				summa += 1;
			}
		}
	}
	println!("{}", summa);
}
