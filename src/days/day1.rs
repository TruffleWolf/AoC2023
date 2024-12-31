use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

pub fn part1(){
    println!("Day1 Part1");
    //let input_doc = File::open("./src/inputs//test/Day1test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day1input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut values:Vec<i32> = Vec::new();
    for line in reader.lines(){
        let row:Vec<char> = match line {
            Ok(s)=>s.chars().collect(),
            Err(_)=>{println!("Failed to read row");break}
        };
        let mut first_digit:i32 = 0;
        for r in &row{
            if r.is_ascii_digit(){
                //println!("{r}");
                first_digit = FromStr::from_str(r.to_string().as_str()).expect("Failed to convert to i32");
                break
            }
        }
        let mut last_digit:i32 = 0;
        for r in 1..row.len()+1{
            if row[row.len()-r].is_ascii_digit(){
                //println!("{}",row[row.len()-r]);
                last_digit = FromStr::from_str(row[row.len()-r].to_string().as_str()).expect("Failed to convert to i32");
                break
            }
        }
        values.push((first_digit*10)+last_digit);
    }
    let mut total:i32 = 0;
    for v in values{
        //println!("{v}");
        total+= v;
    }
    println!("Total:{}",total);
}

pub fn part2(){
    println!("Day1 Part2");
    //let input_doc = File::open("./src/inputs//test/Day1test2.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day1input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut values:Vec<i32> = Vec::new();
    let num_strings:Vec<&str> = vec!["one","two","three","four","five","six","seven","eight","nine"];
    for line in reader.lines(){
        
        

        let mut count:u32= 1;
        let mut ref_line:String = line.expect("Filed to parse line");
        for n in &num_strings{
            
            while ref_line.contains(n){
                let index = match ref_line.find(n){
                    Some(x)=>x+1,
                    None=>{println!("find index broken");0},
                };
                let new_char = match char::from_digit(count, 10){
                    Some(x)=>x,
                    None=>{println!("failed to conver char");'x'},
                };
                ref_line.insert(index,new_char);
            }
            count+=1;
        }

        let row:Vec<char> = ref_line.chars().collect();
        //println!("{}",ref_line);
        let mut first_digit:i32 = 0;
        for r in &row{
            if r.is_ascii_digit(){
                //println!("{r}");
                first_digit = FromStr::from_str(r.to_string().as_str()).expect("Failed to convert to i32");
                break
            }
        }
        let mut last_digit:i32 = 0;
        for r in 1..row.len()+1{
            if row[row.len()-r].is_ascii_digit(){
                //println!("{}",row[row.len()-r]);
                last_digit = FromStr::from_str(row[row.len()-r].to_string().as_str()).expect("Failed to convert to i32");
                break
            }
        }
        values.push((first_digit*10)+last_digit);
    }
    let mut total:i32 = 0;
    for v in values{
        //println!("{v}");
        total+= v;
    }
    println!("Total:{}",total);
}