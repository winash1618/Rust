use std::collections::HashMap;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mov: String = String::from("mov");
    let inc: String = String::from("inc");
    let dec: String = String::from("dec");
    let jnz: String = String::from("jnz");
	let mut n : usize= 0;
	while n < program.len()
	{
		let str: Vec<&str> = program[n].split(" ").collect();
		if str[0].eq(&mov) == true
		{
			if registers.contains_key(str[2]) == true
			{
				registers.insert(str[1].to_string(), *registers.get(str[1]).unwrap() as i64);
			}
			else
			{
				registers.insert(str[1].to_string(), str[2].parse::<i32>().unwrap() as i64);
			}
		}
		else if str[0].eq(&inc) == true
		{
			if registers.contains_key(str[1]) == true
			{
				*registers.get_mut(str[1]).unwrap() += 1;
			}
		}
		else if str[0].eq(&dec) == true
		{
			if registers.contains_key(str[1]) == true
			{
				*registers.get_mut(str[1]).unwrap() -= 1;
			}
		}
		else if str[0].eq(&jnz) == true
		{
			if registers.contains_key(str[1]) == true
			{
				if registers.get(str[1]).is_some() == true
				{
					n = n + str[2].parse::<usize>().unwrap() as usize;
				}
			}
		}
		n += 1;
	}
    registers
}

fn main()
{
	let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
	println!("{:?}",simple_assembler(program));
}