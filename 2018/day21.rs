fn main() 
{
	let mut values:Vec<usize> = Vec::new();
	let mut found = false;
	
	// bori 5 65536 4
	let mut reg4 = 0 | 65536;
	//seti 15466939 9 5
	let mut reg5 = 15466939;

	loop
	{
		// bani 4 255 3
		let mut reg3 = reg4 & 255;
		// addr 5 3 5
		reg5 = reg5 + reg3;
		// bani 5 16777215 5
		reg5 = reg5 & 16777215;
		// muli 5 65899 5
		reg5 = reg5 * 65899;
		// bani 5 16777215 5
		reg5 = reg5 & 16777215;
		// gtir 256 4 3
		if 256 > reg4
		{
			// eqrr 5 0 3
			// halt if reg0 = reg5
			if !found
			{
				println!("{:?}", reg5);
				found = true;
			}
			// save this to vec
			if !values.contains(&reg5)
			{
				values.push(reg5);
			}
			else
			{
				// looped
				println!("{:?}", values[values.len()-1]);
				break;
			}
			
			// bori 5 65536 4
			reg4 = reg5 | 65536;
			//seti 15466939 9 5
			reg5 = 15466939;
			continue;
		}
		else
		{
			// seti 0 7 3
			reg3 = 0;
			loop
			{
				// addi 3 1 1
				let mut reg1 = reg3 + 1;
				// muli 1 256 1
				reg1 = reg1 * 256;
				// gtrr 1 4 1
				if reg1 > reg4
				{
					// setr 3 7 4
					reg4 = reg3;
					break;
				}
				else
				{
					// addi 3 1 3
					reg3 += 1;
				}
			}
		}
	}
}
