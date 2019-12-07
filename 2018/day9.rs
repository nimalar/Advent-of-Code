fn main() 
{
	let player_count = 471;
	let marble_count = 72026;
	let mut latest_marble = 0;
	let mut player_score:Vec<i32> = vec!(0; player_count);
	let mut marble_circle:Vec<i32> = vec!(0; 1);
	let mut current_marble_index = 0;
	let mut current_player = 0;
	
	while latest_marble < marble_count
	{
		latest_marble += 1;
		if latest_marble % 23 != 0
		{
			current_marble_index = (current_marble_index + 1) % (marble_circle.to_vec().len()) + 1;
			marble_circle.insert(current_marble_index, latest_marble);
		}
		else
		{
			player_score[current_player] += latest_marble;
			let length = marble_circle.to_vec().len() as i32;
			current_marble_index = ((((current_marble_index as i32 - 7) % length) + length) % length) as usize;
			player_score[current_player] += marble_circle.remove(current_marble_index);
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

