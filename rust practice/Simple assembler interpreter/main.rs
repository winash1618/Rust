use std::collections::HashMap;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mov: String = String::from("mov");
    let inc: String = String::from("inc");
    let dec: String = String::from("dec");
    let jnz: String = String::from("jnz");
	let mut n : i64= 0;
	while n < program.len() as i64
	{
		println!("{:?}", registers);
		let str: Vec<&str> = program[n as usize].split(" ").collect();
		if str[0].eq(&mov) == true
		{
			if registers.contains_key(str[2]) == true
			{
				if registers.contains_key(str[1]) == true
				{
					*registers.get_mut(str[1]).unwrap() = *registers.get(str[2]).unwrap() as i64;
				}
				else
				{
					registers.insert(str[1].to_string(), *registers.get(str[2]).unwrap() as i64);
				}
			}
			else
			{
				if registers.contains_key(str[1]) == true
				{
					*registers.get_mut(str[1]).unwrap() = *registers.get(str[2]).unwrap() as i64;
				}
				else
				{
					registers.insert(str[1].to_string(), str[2].parse::<i32>().unwrap() as i64);
				}
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
				if *registers.get(str[1]).unwrap() as i64 > 0
				{
					n = n + str[2].parse::<i64>().unwrap() as i64  - 1;
					println!("{}", n);
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
	// let program = vec![
	// 	"mov c 12",
	// 	"mov b 0",
	// 	"mov a 200",
	// 	"dec a",
	// 	"inc b",
	// 	"jnz a -2",
	// 	"dec c",
	// 	"mov a b",
	// 	"jnz c -5",
	// 	"jnz 0 1",
	// 	"mov c a",
	// ];
	println!("{:?}", simple_assembler(program));
}