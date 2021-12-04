use std::env;
use std::fs::File;
use std::io::prelude::*;

fn calculate_score(ref board:&Vec<Vec<usize>>, ref called:&[usize]) -> usize
{
	let mut score = 0;
	for row in *board
	{
		for number in row
		{
			if !called.contains(&number)
			{
				score += number;
			}
		}
	}
	score
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut first_line = true;
	let mut numbers:Vec<usize> = Vec::new();
	let mut called:Vec<usize> = Vec::new();
	let mut boards:Vec<Vec<Vec<usize>>> = Vec::new();
	let mut winners:Vec<usize> = Vec::new();
	let mut winners_last:Vec<usize> = Vec::new();
	
	for row in contents.lines()
	{
		if first_line
		{
			numbers = row.split(",").map(|value| value.parse().unwrap()).collect();
			first_line = false;
			continue;
		}
		if row.is_empty()
		{
			boards.push(Vec::new());
			continue;
		}
		let values:Vec<usize> = row.split_whitespace().map(|value| value.parse().unwrap()).collect();
		boards.last_mut().unwrap().push(values);
	}
	
	while winners.len() < boards.len()
	{
		let latest = numbers.remove(0);
		called.push(latest);
		for i in 0..boards.len()
		{
			if !winners.contains(&i)
			{
				'this_board: for y in 0..boards[i].len()
				{
					for x in 0..boards[i][y].len()
					{
						if boards[i][y][x] == latest
						{
							let mut sum_vertical = 0;
							let mut sum_horizontal = 0;
							for k in 0..boards[i][y].len()
							{
								if called.contains(&boards[i][y][k])
								{
									sum_vertical += 1;
								}
								if called.contains(&boards[i][k][x])
								{
									sum_horizontal += 1;
								}
							}
							if sum_vertical == 5 || sum_horizontal == 5
							{
								winners.push(i);
								winners_last.push(called.len());
							}
							break 'this_board;
						}
					}
				}
			}
		}
	}
	
	let mut score:Vec<usize> = Vec::new();
	score.push(calculate_score(&boards[winners[0]], &called[..winners_last[0]]));
	score.push(calculate_score(&boards[*winners.last().unwrap()], &called));
	
	println!("First star: {}", called[winners_last[0] - 1] * score[0]);
	println!("Second star: {}", called.last().unwrap() * score[1]);
}
