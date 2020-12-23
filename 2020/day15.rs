use std::collections::HashMap;

fn play_game(ref numbers:&Vec<usize>, turns:usize) -> usize
{
	let mut game:HashMap<usize, usize> = HashMap::new();
	let mut last_spoken = numbers[0];
	for i in 1..numbers.len()
	{
		game.insert(last_spoken, i - 1);
		last_spoken = numbers[i];
	}
	
	for i in numbers.len()..turns
	{
		let next_spoken;
		match game.get(&last_spoken)
		{
			Some(value) => next_spoken = i - 1 - value,
			None => next_spoken = 0
		}
		game.insert(last_spoken, i - 1);
		last_spoken = next_spoken;
	}
	return last_spoken;
}

fn main() 
{
	let contents = "0,8,15,2,12,1,4";
	let numbers:Vec<usize> = contents.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

	println!("First star: {}", play_game(&numbers, 2020));
	println!("Second star: {}", play_game(&numbers, 30000000));
}
