fn main() 
{
	let recipe_count = 607331;
	let mut latest_recipe = 1;
	let mut cooking:Vec<usize> = vec![0,1];
	let mut recipe_circle:Vec<i32> = vec![3,7];
	
	while latest_recipe < recipe_count + 10
	{
		let mut new_recipes = recipe_circle[cooking[0]] + recipe_circle[cooking[1]];
		let mut new_recipe_vec:Vec<i32> = Vec::new();
		while new_recipes >= 10
		{
			new_recipe_vec.insert(0, new_recipes % 10);
			new_recipes = new_recipes / 10;
		}
		new_recipe_vec.insert(0, new_recipes);
		recipe_circle.append(&mut new_recipe_vec);
		latest_recipe = recipe_circle.len();
		
		for i in 0..2
		{
			cooking[i] = (cooking[i] + 1 + recipe_circle[cooking[i]] as usize) % recipe_circle.len();
		}
	}
	let mut result:String = String::new();
	for i in recipe_count..recipe_count + 10
	{
		result.push_str(&recipe_circle[i].to_string());
	}
	println!("{}", result);
}

