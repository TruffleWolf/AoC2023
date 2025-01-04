use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

pub fn part1(){
    println!("Day9 Part1");
    let main_list:Vec<Vec<i64>> = parse_document();
    let mut total:i64 = 0;

    //total = recur_list(&main_list[29]);
    for m in main_list{
        let addition:i64 = recur_list(&m);
        //println!("{addition}");
        total+=addition;
    }

    println!("Total:{total}");
}

pub fn part2(){
    println!("Day9 Part2");

    let main_list:Vec<Vec<i64>> = parse_document();
    let mut total:i64 = 0;

    //total = recur_f_list(&main_list[2]);
    for m in main_list{
        let addition:i64 = recur_f_list(&m);
        //println!("{addition}");
        total+=addition;
    }

    println!("Total:{total}");
}

fn recur_f_list(input:&Vec<i64>)->i64{
    let mut new_list:Vec<i64>= Vec::with_capacity(input.len()-1);

    //println!("{:?}",input);
    let mut total:bool = true;
    for i in 0..input.len()-1{
        new_list.push(input[i+1]-input[i]);
        total =total&&input[i+1]==input[i];
    }

    if total{
        //println!("{:?}",input);
        return input[0];
    }
    else{
        let value:i64 = input[0]-recur_f_list(&new_list);
        return value;
    }
}

fn recur_list(input:&Vec<i64>)->i64{
    let mut new_list:Vec<i64>= Vec::with_capacity(input.len()-1);

    //println!("{:?}",input);
    let mut total:bool = true;
    for i in 0..input.len()-1{
        new_list.push(input[i+1]-input[i]);
        total =total&&input[i+1]==input[i];
    }

    if total{
        //println!("{:?}",input);
        return input[input.len()-1];
    }
    else{
        let value:i64 = input[input.len()-1]+recur_list(&new_list);
        return value;
    }
    
}

fn parse_document()->Vec<Vec<i64>>{
    let mut result:Vec<Vec<i64>> = Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day9test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day9input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let row:Vec<&str> = line_str.split(" ").collect();
        let mut int_row:Vec<i64> = Vec::new();
        if row.len()>1{
            for r in row{
                if r.parse::<i64>().is_ok(){int_row.push(FromStr::from_str(r).expect("Could not parse i64"));}
            }
            result.push(int_row);
        }
    }

    return result;
}