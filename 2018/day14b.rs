fn main() 
{
	let recipe_count = 607331;
	let length = 6;
	let mut cooking:Vec<usize> = vec![0,1];
	let mut recipe_circle:Vec<i32> = vec![3,7];
	
	'outer:loop
	{
		let mut new_recipes = recipe_circle[cooking[0]] + recipe_circle[cooking[1]];
		let mut new_recipe_vec:Vec<i32> = Vec::new();
		let mut inserted = 0;
		while new_recipes >= 10
		{
			new_recipe_vec.insert(0, new_recipes % 10);
			new_recipes = new_recipes / 10;
			inserted += 1;
		}
		new_recipe_vec.insert(0, new_recipes);
		inserted += 1;
		recipe_circle.append(&mut new_recipe_vec);
		
		for i in 0..2
		{
			cooking[i] = (cooking[i] + 1 + recipe_circle[cooking[i]] as usize) % recipe_circle.len();
		}
		
		while inserted > 0 && recipe_circle.len() >= length + inserted
		{
			let mut result:String = String::new();
			for i in recipe_circle.len() - inserted - (length - 1) .. recipe_circle.len() - inserted + 1
			{
				result.push_str(&recipe_circle[i].to_string());
			}
			if result == recipe_count.to_string()
			{
				println!("{}", recipe_circle.len() - length - inserted + 1);
				break 'outer;
			}
			inserted -= 1;
		}
	}
}
