fn main() 
{
	let mut result = 0;
	let mut second_result = 0;
	for i in 193651..649729+1
	{
		let digits:Vec<_> = i.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
		let mut double_found = false;
		let mut better_double_found = false;
		for j in 1..digits.len()
		{
			if digits[j-1] > digits[j]
			{
				break;
			}
			else if digits[j-1] == digits[j]
			{
				double_found = true;
			}
			if digits[j-1] == digits[j] && (j < 2 || digits[j-2] != digits[j]) && (j == digits.len() - 1 || digits[j+1] != digits[j]) 
			{
				better_double_found = true;
			}
			if j == (digits.len() - 1) && double_found
			{
				result += 1;
				if better_double_found
				{
					second_result += 1;
				}
			}
		}
	}
	println!("First star: {:?}", result);
	println!("First star: {:?}", second_result);
}
