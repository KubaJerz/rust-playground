use std::env;

fn print_args(args: Vec<String>) -> Result<i32, String> {
    if args.len() == 2 {
       let in_num: Result<i32, std::num::ParseIntError> =  args[1].parse();
       match in_num {
        Ok(num) => {
            println!("Good the arg is: {}", num);
            return Ok(num)
        }, Err(_) => {
            println!("The arg must be an int");
            return Err("The arg must be an int".to_string())
        }

       }
    } else {
        println!("None or too many args were given. Should be just one.");
        return Err(String::from("None or too many args were given. Should be just one."))
    }
}

fn factorial(val: i32) {
    let mut fac = val;
    for i in (1..val).rev(){
        fac = fac * (i);
    }
    println!("{}",fac);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let val = print_args(args);

    if let Ok(number) = val{
        factorial(number);
    }

}