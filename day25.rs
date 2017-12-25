fn main() {
    let mut tape:Vec<usize> = vec![0];
	let mut state = "A";
	let mut index:i32 = 0;
	let mut summa = 0;
	
	for i in 0..12172063
	{
		if index == tape.len() as i32
		{
			tape.push(0);
		}
		else if index < 0
		{
			tape.insert(0,0);
			index = 0;
		}
	
		if state == "A"
		{
			if tape[index as usize] == 0
			{
				tape[index as usize] = 1;
				index += 1;
				state = "B";
			}
			else
			{
				tape[index as usize] = 0;
				index -= 1;
				state = "C";
			}
		}
		else if state == "B"
		{
			if tape[index as usize] == 0
			{
				tape[index as usize] = 1;
				index -= 1;
				state = "A";
			}
			else
			{
				tape[index as usize] = 1;
				index -= 1;
				state = "D";
			}
		}
		else if state == "C"
		{
			if tape[index as usize] == 0
			{
				tape[index as usize] = 1;
				index += 1;
				state = "D";
			}
			else
			{
				tape[index as usize] = 0;
				index += 1;
				state = "C";
			}
		}
		else if state == "D"
		{
			if tape[index as usize] == 0
			{
				tape[index as usize] = 0;
				index -= 1;
				state = "B";
			}
			else
			{
				tape[index as usize] = 0;
				index += 1;
				state = "E";
			}
		}
		else if state == "E"
		{
			if tape[index as usize] == 0
			{
				tape[index as usize] = 1;
				index += 1;
				state = "C";
			}
			else
			{	
				tape[index as usize] = 1;
				index -= 1;
				state = "F";
			}
		}
		else if state == "F"
		{
			if tape[index as usize] == 0
			{
				tape[index as usize] = 1;
				index -= 1;
				state = "E";
			}
			else
			{
				tape[index as usize] = 1;
				index += 1;
				state = "A";
			}
		}
	}
	for j in tape
	{
		summa += j;
	}
	println!("{}", summa);
}
