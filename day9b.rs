use std::collections::VecDeque;

fn main() 
{
	let player_count = 471;
	let marble_count = 7202600;
	let mut latest_marble = 0;
	let mut player_score:Vec<i64> = vec!(0; player_count);
	let mut marbles_circle:VecDeque<i64> = VecDeque::new();
	let mut current_player = 0;
	marbles_circle.push_back(latest_marble);
	
	while latest_marble < marble_count
	{
		latest_marble += 1;
		if latest_marble % 23 != 0
		{
			let movable = marbles_circle.pop_front().unwrap();
			marbles_circle.push_back(movable);
			marbles_circle.push_back(latest_marble);
		}
		else
		{
			player_score[current_player] += latest_marble;
			for _i in 0..7
			{
				let movable = marbles_circle.pop_back().unwrap();
				marbles_circle.push_front(movable);
			}
			player_score[current_player] += marbles_circle.pop_back().unwrap();
			let new_first = marbles_circle.pop_front().unwrap();
			marbles_circle.push_back(new_first);
		}
		current_player = (current_player + 1) % player_count;
	}
	let mut max_score = 0;
	for player in player_score
	{
		if player > max_score
		{
			max_score = player;
		}
	}
	println!("{:?}", max_score);
}

