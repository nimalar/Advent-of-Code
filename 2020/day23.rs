fn cup_game(ref mut cups:&mut Vec<usize>, start_cup:usize, max_cup:usize, rounds:usize) -> ()
{
	let mut current_cup = start_cup;
	for _i in 0..rounds
	{
		let mut removed:Vec<usize> = Vec::new();
		for _j in 0..3
		{
			let to_be_pushed = cups[current_cup];
			let next_in_line = cups[to_be_pushed];
			removed.push(to_be_pushed);
			cups[current_cup] = next_in_line;
		}
		let mut destination_cup = current_cup - 1;
		while removed.contains(&destination_cup) || destination_cup == 0
		{
			if destination_cup == 0
			{
				destination_cup = max_cup + 1;
			}
			destination_cup -= 1;
		}
		let next_in_line = cups[destination_cup];
		cups[destination_cup] = removed[0];
		cups[removed[2]] = next_in_line;
		current_cup = cups[current_cup];
	}
	let mut first = cups[1];
	let mut answer = "".to_string();
	if cups.len() < 100
	{
		while first != 1
		{
			answer.push_str(&first.to_string());
			first = cups[first];
		}
		println!("First star: {:?}", answer.parse::<usize>().unwrap());
	}
	else
	{
		let second = cups[first];
		println!("Second star: {:?}", first * second);
	}
}

fn main() 
{
	let cups_vec:Vec<u32> = "487912365".chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
	let mut cup_hash:Vec<usize> = vec![0; 10];
	for i in 0..cups_vec.len() - 1
	{
		cup_hash[cups_vec[i] as usize] = cups_vec[i + 1] as usize;
	}
	cup_hash[cups_vec[cups_vec.len() - 1] as usize] = cups_vec[0] as usize;
	cup_game(&mut cup_hash.to_vec(), cups_vec[0] as usize, 9, 100);
	
	cup_hash[cups_vec[cups_vec.len() - 1] as usize] = 10;
	for i in 10..1000000
	{
		cup_hash.push(i + 1);
	}
	cup_hash.push(cups_vec[0] as usize);
	cup_game(&mut cup_hash, cups_vec[0] as usize, 1000000, 10000000);
}
