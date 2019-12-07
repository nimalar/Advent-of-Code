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
	
	for line in contents.lines()
	{
		let parts:Vec<&str> = line.split_whitespace().collect();
		
		//part 3 is offsets
		let offsets:Vec<&str> = parts[2].trim_matches(':').split(',').collect();
		let offset_x:usize = offsets[0].parse().unwrap();
		let offset_y:usize = offsets[1].parse().unwrap();
		
		//part 4 is lengths
		let lengths:Vec<&str> = parts[3].split('x').collect();
		let length:usize = lengths[0].parse().unwrap();
		let height:usize = lengths[1].parse().unwrap();
		
		for y in offset_y..offset_y+height
		{
			for x in offset_x..offset_x+length
			{
				fabric[y][x] += 1;
			}
		}
	}
	
	let mut result = 0;
	
	for y in 0..fabric.len()
	{
		for x in 0..fabric[y].len()
		{
			if fabric[y][x] > 1
			{
				result += 1;
			}
		}
	}
	println!("{}",result);
}

