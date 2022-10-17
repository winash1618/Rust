use std::collections::HashMap;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mov: String = String::from("mov");
    let inc: String = String::from("inc");
    let dec: String = String::from("dec");
    let jnz: String = String::from("jnz");
    let mut cont : i64 = 0;
    let mut flag = 1;
    let mut n : i64= 0;
    let mut mm = 0;
    while n < program.len() as i64
    {
        if n > mm
        {
            flag = 1;
        }
        let str: Vec<&str> = program[n as usize].split(" ").collect();
        if str[0].eq(&mov) == true
        {
            if registers.contains_key(str[2]) == true
            {
                registers.insert(str[1].to_string(), *registers.get(str[2]).unwrap() as i64);
            }
            else
            {
                registers.insert(str[1].to_string(), str[2].parse::<i32>().unwrap() as i64);
            }
        }
        else if str[0].eq(&inc) == true
        {
                *registers.get_mut(str[1]).unwrap() += 1;
        }
        else if str[0].eq(&dec) == true
        {
                *registers.get_mut(str[1]).unwrap() -= 1;
        }
        else if str[0].eq(&jnz) == true
        {
            if registers.contains_key(str[1]) == true
            {
                if *registers.get(str[1]).unwrap() as i64 != 0
                {
                    n = n + str[2].parse::<i64>().unwrap() as i64  - 1;
                }
            }
            else
            {
                if str[1].parse::<i64>().unwrap() as i64 > 0
                {
                    if cont == 0 && flag == 1
                    {
                        mm = n;
                        cont = str[1].parse::<i64>().unwrap() as i64;
                    }
                }
                if cont > 0
                {
                    flag = 0;
                    cont -= 1;
                    n = n + str[2].parse::<i64>().unwrap() as i64  - 1;
                }
            }
        }
        
        n += 1;
    }
    registers
}

fn main()
{
	// let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz 5 -1","dec a","jnz 5 -1", "inc a"];
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
	let program = vec!["mov a 1", "mov b 1", "mov c 0", "mov d 26", "jnz c 2", "jnz 1 5", "mov c 7", "inc d", "dec c", "jnz c -2", "mov c a", "inc a", "dec b", "jnz b -2", "mov b c", "dec d", "jnz d -6", "mov c 18", "mov d 11", "inc a", "dec d", "jnz d -2", "dec c", "jnz c -5"];
	// let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
	// let program = vec!["mov a -10", "mov b a", "inc a", "dec b", "jnz a -2"];

	println!("{:?}", simple_assembler(program));
}


/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// use std::collections::HashMap;

// fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
//     let mut registers = HashMap::new();
    
//     let cor = |x: &str, reg: &HashMap<String, i64>| match x.parse::<i64>() { Ok(y) => y, Err(_) => reg[x] };
    
//     let mut p = 0;
//     while let Some(i) = program.get(p) {
//         let args: Vec<&str> = i.split_whitespace().collect();
//         match args[0] {
//             "mov" => { registers.insert(args[1].to_string(), cor(args[2], &registers)); }
//             "inc" => { *registers.get_mut(args[1]).unwrap() += 1; }
//             "dec" => { *registers.get_mut(args[1]).unwrap() -= 1; }
//             "jnz" => { if cor(args[1], &registers) != 0 { p = (p as i64 + cor(args[2], &registers) - 1) as usize; } }
//             _ => ()
//         }
//         p+=1;
//     }
//     registers
// }