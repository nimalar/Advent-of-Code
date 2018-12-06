use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents:String = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut area = vec![vec![(0i32, 0i32); 1000]; 1000];
	
	let mut number = 0;
	
	for line in contents.lines()
	{
		number += 1;
		
		let coordinates:Vec<&str> = line.split(", ").collect();
		let x_coordinate = coordinates[0].parse::<usize>().unwrap();
		let y_coordinate = coordinates[1].parse::<usize>().unwrap();
		
		area[y_coordinate][x_coordinate] = (number, 0);
		
		
		for y in 0..area.len()
		{
			for x in 0..area[y].len()
			{
				let distance = (x_coordinate as i32 - x as i32).abs() + (y_coordinate as i32 - y as i32).abs();
				
				if (area[y][x].1 > distance) || area[y][x].0 == 0
				{
					area[y][x] = (number, distance);
				}
				else if area[y][x].1 == distance && area[y][x].0 != number
				{
					area[y][x] = (-1, distance);
				}
			}
		}		
	}
	
	let mut winner = 0;
	
	for i in 1..number+1
	{
		let mut count = 0;
		let mut infinite = false;
		'outer:for y in 0..area.len()
		{
			for x in 0..area[y].len()
			{
				if area[y][x].0 == i && (y == 0 || x == 0 || y == area.len()-1 || x == area[y].len()-1)
				{
					infinite = true;
					break 'outer;
				}
				else if area[y][x].0 == i
				{
					count += 1;
				}
			}
		}
		if !infinite && count > winner
		{
			winner = count;
		}
	}

	println!("{:?}", winner);
}

