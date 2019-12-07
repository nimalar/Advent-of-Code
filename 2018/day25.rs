use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main()
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut constellations:Vec<(i32, i32, i32, i32, i32)> = Vec::new();
	let mut index = 0;
	let mut previous_sum = 0;
	
	// init stars
	for line in contents.lines()
	{
		let values = line.split(',').collect::<Vec<&str>>();
		constellations.push((values[0].parse().unwrap(), values[1].parse().unwrap(), values[2].parse().unwrap(), values[3].parse().unwrap(), index));
		index += 1;
	}
	
	loop
	{
		for i in 0..constellations.len()
		{
			for j in i+1..constellations.len()
			{
				if (constellations[i].0 - constellations[j].0).abs() + (constellations[i].1 - constellations[j].1).abs() + (constellations[i].2 - constellations[j].2).abs() + (constellations[i].3 - constellations[j].3).abs() <= 3
				{
					constellations[j].4 = cmp::min(constellations[i].4, constellations[j].4);
					constellations[i].4 = constellations[j].4;
				}
			}
		}
	
		let mut sum = 0;
		constellations.sort_by(|a, b| (a.4).cmp(&(b.4)));
		for i in 0..constellations.len()
		{
			if i == 0 || constellations[i].4 != constellations[i-1].4
			{	
				sum += 1;
			}
		}
		if sum == previous_sum
		{
			break;
		}
		previous_sum = sum;
	}
	println!("{:?}", previous_sum);
}
