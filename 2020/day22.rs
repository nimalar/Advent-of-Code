use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet,VecDeque};

fn game(ref mut players:Vec<VecDeque<usize>>, recursive:bool) -> (usize, usize)
{
	let mut result = 0;
	let mut done:HashSet<Vec<VecDeque<usize>>> = HashSet::new();
	while !players[0].is_empty() && !players[1].is_empty() && !done.contains(players)
	{
		result = 1;
		done.insert(players.to_vec());
		let card_1 = players[0].pop_front().unwrap();
		let card_2 = players[1].pop_front().unwrap();
		if recursive && players[0].len() >= card_1 && players[1].len() >= card_2
		{
			let mut players_another = players.to_vec();
			players_another[0].truncate(card_1);
			players_another[1].truncate(card_2);
			result = game(players_another.to_vec(), true).0;
		}
		else
		{
			if card_2 > card_1
			{
				result = 2;
			}
		}
		if result == 1
		{
			players[0].push_back(card_1);
			players[0].push_back(card_2);
		}
		else
		{
			players[1].push_back(card_2);
			players[1].push_back(card_1);
		}
		if done.contains(players)
		{
			result = 1;
		}
	}
	let mut multiplier = 1;
	let mut score = 0;
	while !players[result - 1].is_empty()
	{
		score += players[result - 1].pop_back().unwrap() * multiplier;
		multiplier += 1;
	}
	return (result, score);
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut players:Vec<VecDeque<usize>> = Vec::new();
	
	for row in contents.lines()
	{
		if row.contains("Player")
		{
			players.push(VecDeque::new());
		}
		else if row.trim() != ""
		{
			players.last_mut().unwrap().push_back(row.parse().unwrap());
		}
	}
	
	println!("First star: {}", game(players.to_vec(), false).1);
	println!("Second star: {}", game(players.to_vec(), true).1);
}
