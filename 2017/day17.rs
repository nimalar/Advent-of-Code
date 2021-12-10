
fn main()
{
	part_1();
	part_2();
}

fn part_1() {

	let input = 394;
	let mut current_pos = 0;
	let mut buffer:Vec<usize> = Vec::new();
	let mut index = 0;
	buffer.push(0);
	
	while current_pos < 2017
	{
		index = (index + input) % buffer.len() + 1;
		current_pos += 1;
		buffer.insert(index, current_pos); 
	}
	
	//find the position of '2017'
	println!("{}", buffer[buffer.iter().position(|&s| s == 2017).unwrap()+1]);
}

fn part_2() {

	let input = 394;
	let mut current_pos = 0;
	let mut index = 0;
	let mut buffer_len = 1;
	let mut last = 0;
	
	while current_pos < 50000000
	{
		index = (index + input) % buffer_len + 1;
		current_pos += 1;
		buffer_len += 1;
		if index == 1
		{
			last = current_pos;
		}
	}
	println!("{}", last);
}

