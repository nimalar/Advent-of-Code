fn main() 
{
	let mut cells:Vec<Vec<i32>> = vec![vec![0i32; 300]; 300];
	let serial = 3613;
	
	for y in 0..cells.len()
	{
		for x in 0..cells[y].len()
		{	
			let rack_id = x + 1 + 10;
			let mut power_level:i32 = ((rack_id * (y + 1) + serial) * rack_id) as i32;
			power_level = ((power_level / 100) % 10) - 5;
			cells[y][x] = power_level;
		}
	}
	let mut max = 0;
	let mut coordinates = (0,0);
	for y in 0..cells.len() - 2
	{
		for x in 0..cells[y].len() - 2
		{	
			let mut sum = 0;
			for i in 0..3
			{
				for j in 0..3
				{
					sum += cells[y+i][x+j];
				}
			}
			if sum > max
			{
				max = sum;
				coordinates = (x + 1, y + 1);
			}
		}
	}
	println!("{:?}", coordinates);
}
