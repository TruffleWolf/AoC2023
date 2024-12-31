use std::{io, time::Instant};

mod days;
//mod vec2;


fn main() {
    loop{
        println!("Please enter an Advent of Code 2023 Day:");
        let mut input: String = String::new();
        let quit_strings = [String::from("q"),String::from("Q"),String::from("Quit"),String::from("QUIT"),String::from("quit"),String::from("exit"),String::from("EXIT")];
        const INVALID_MSG: &str = "Invalid Entry";
        io::stdin().read_line(&mut input)
                .expect("Failed to read");
        
        
        if quit_strings.iter().any(|e| input.contains(e)){
            println!("Exiting");
            return
        }
        let day: i32 = match input.trim().parse(){
            Ok(num)=>num,
            Err(_) => -1,
        };
        if day >0 && day<26{
            loop{
                println!("Part 1 or 2:");
                let mut input: String = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Failed to read");
                if quit_strings.iter().any(|e| input.contains(e)){
                    println!("Exiting");
                    return
                }
                let part: i32 = match input.trim().parse(){
                    Ok(num)=>num,
                    Err(_) => -1,
                };
                if part == 1 || part == 2{
                    let start_t = Instant::now();
                    days::call_daypart(day,part);
                    let end_t = start_t.elapsed();
                    println!("Time to complete:{:.2?}",end_t);
                    break
                }
                else{println!("{}",INVALID_MSG);}

            }
        }
        
        else{println!("{}",INVALID_MSG);}
        
    }
}
