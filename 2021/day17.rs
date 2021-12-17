use std::cmp;

fn main() 
{
	let target_x = (29, 73);
	let target_y = (-248, -194);
	let mut max = 0;
	let mut sum = 0;
	
	for x in -500..500
	{
		for y in -500..500
		{
			let mut position = (0, 0);
			let mut velocity = (x, y);
			let mut target_reached = false;
			let mut local_max = 0;
			loop
			{
				position.0 += velocity.0;
				position.1 += velocity.1;
				if velocity.0 > 0
				{
					velocity.0 -= 1;
				}
				else if velocity.0 < 0
				{
					velocity.0 += 1;
				}
				velocity.1 -= 1;
				local_max = cmp::max(local_max, position.1);
				if position.0 >= target_x.0 && position.0 <= target_x.1 
					&& position.1 >= target_y.0 && position.1 <= target_y.1
				{
					target_reached = true;
				}
				if target_reached
				{
					max = cmp::max(max, local_max);
				}
				if position.1 < target_y.0 || position.0 > target_x.1
				{
					break;
				}
			}
			if target_reached
			{
				sum += 1;
			}
		}
	}
		
	println!("First star: {}", max);
	println!("Second star: {}", sum);
}
