use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
	
	let mut fabric = vec![vec![0i32; 1000]; 1000];
	
	let mut uniques:Vec<i32> = Vec::new();
	
	for line in contents.lines()
	{
		let parts:Vec<&str> = line.split_whitespace().collect();
		
		//part 1 is id
		let id:i32 = parts[0].trim_matches('#').parse().unwrap();
		
		//part 3 is offsets
		let offsets:Vec<&str> = parts[2].trim_matches(':').split(',').collect();
		let offset_x:usize = offsets[0].parse().unwrap();
		let offset_y:usize = offsets[1].parse().unwrap();
		
		//part 4 is lengths
		let lengths:Vec<&str> = parts[3].split('x').collect();
		let length:usize = lengths[0].parse().unwrap();
		let height:usize = lengths[1].parse().unwrap();
		
		let mut overlaps = 0;
		
		for y in offset_y..offset_y+height
		{
			for x in offset_x..offset_x+length
			{
				if fabric[y][x] == 0
				{
					fabric[y][x] = id;
				}
				else
				{
					uniques.retain(|&z| z != fabric[y][x]);
					overlaps += 1;
				}
			}
		}
		
		if overlaps == 0
		{
			uniques.push(id);
		}
	}

	println!("{:?}",uniques);
}

