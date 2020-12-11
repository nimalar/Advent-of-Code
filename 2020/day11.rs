use std::env;
use std::fs::File;
use std::io::prelude::*;

fn find_adjacent(ref seats:&Vec<Vec<char>>, i:usize, j:usize, i_diff:i32, j_diff:i32, limited_view:bool) -> usize
{
	let mut i_new = i as i32 + i_diff;
	let mut j_new = j as i32 + j_diff;
	loop
	{
		if i_new >= 0 && j_new >= 0 && i_new < seats.len() as i32 && j_new < seats[i_new as usize].len() as i32
		{		
			if seats[i_new as usize][j_new as usize] == '#'
			{
				return 1;
			}
			else if limited_view || seats[i_new as usize][j_new as usize] == 'L'
			{
				return 0;
			}
			i_new += i_diff;
			j_new += j_diff;
			continue;
		}
		return 0;
	}
}

fn seat_game(mut seats:Vec<Vec<char>>, limited_view:bool) -> usize
{
	let mut changes = 1;
	while changes > 0
	{
		let mut seats_next_round = seats.to_vec();
		changes = 0;
		for i in 0..seats.len()
		{
			for j in 0..seats[i].len()
			{
				if seats[i][j] == '.'
				{
					continue;
				}
				let mut adjacents = 0;
				for i_diff in 0..3
				{
					for j_diff in 0..3
					{
						if !(i_diff == 1 && j_diff == 1)
						{
							adjacents += find_adjacent(&seats, i, j, i_diff - 1, j_diff - 1, limited_view);
						}
					}
				}
				if seats[i][j] == 'L' && adjacents == 0
				{
					seats_next_round[i][j] = '#';
					changes += 1;
				}
				else if seats[i][j] == '#' && (adjacents >= 5 || (limited_view && adjacents >= 4))
				{
					seats_next_round[i][j] = 'L';
					changes += 1;
				}
			}
		}
		seats = seats_next_round.to_vec();
	}
	let mut seat_count = 0;
	for i in 0..seats.len()
	{
		seat_count += seats[i].iter().filter(|&x| *x == '#').count();
	}
	return seat_count;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("sozething went wrong reading the file");
	let mut seats:Vec<Vec<char>> = Vec::new();
	
	for row in contents.lines()
	{	
		seats.push(row.chars().collect());
	}
	
	println!("First star: {}", seat_game(seats.to_vec(), true));
	println!("Second star: {}", seat_game(seats.to_vec(), false));
}
