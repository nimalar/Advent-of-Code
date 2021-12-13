use std::env;
use std::fs::read_to_string;

fn flash(i:usize, j:usize, ref mut octopi:&mut Vec<Vec<u32>>) -> usize
{
	for x in -1i32..2i32
	{
		for y in -1i32..2i32
		{
			let new_x = x + i as i32;
			let new_y = y + j as i32;
			if new_x >= 0 && new_y >= 0 && new_x < octopi.len() as i32 && new_y < octopi[i].len() as i32
			{
				if octopi[new_x as usize][new_y as usize] > 0
				{
					octopi[new_x as usize][new_y as usize] += 1;
				}
			}
		}
	}
	return 1;
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
	let mut octopi:Vec<Vec<u32>> = Vec::new();
	let mut total_flashes = 0;
	let mut previous_flashes = 0;
	
	for row in contents.lines()
	{
		octopi.push(row.chars().map(|value| value.to_digit(10).unwrap()).collect());
	}
	
	let mut step = 1;
	while total_flashes != previous_flashes + 100
	{
		let mut flashes = false;
		let mut first_time = true;
		previous_flashes = total_flashes;
		while first_time || flashes
		{
			flashes = false;
			for i in 0..octopi.len()
			{
				for j in 0..octopi[i].len()
				{
					if first_time
					{
						octopi[i][j] += 1;
						flashes = true;
					}
					else if octopi[i][j] > 9
					{ 
						octopi[i][j] = 0;
						flashes = true;
						total_flashes += flash(i, j, &mut octopi);
					}
				}
			}
			first_time = false;
		}
		if step == 100
		{
			println!("First star: {}", total_flashes);
		}
		step += 1;
	}
	println!("Second star: {}", step - 1);
}
