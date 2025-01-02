use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

pub fn part1(){
    println!("Day4 Part1");

    let (winners,numbers_list) = parse_document();

    let mut total:i32 = 0;
    for w in 0..winners.len(){
        let mut addition:i32 = 0;
        for number in &winners[w]{
            if numbers_list[w].contains(number){
                if addition ==0{addition = 1;}
                else{addition=addition*2;}
            }
        }
        total+=addition;
    }
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day4 Part2");
    let (winners,numbers_list) = parse_document();

    let mut card_count:Vec<i64> = vec![1;winners.len()];
    let mut total:i64 = 0;

    for w in 0..winners.len(){
        let mut duplicates:usize = 0;
        for number in &winners[w]{
            if numbers_list[w].contains(number){
                duplicates +=1;
            }
        }
        for i in 0..duplicates{
            if w+i+1<winners.len(){card_count[w+i+1] += 1*card_count[w]};
        }
        total += card_count[w]
    }
    println!("Total:{total}");  
}

fn parse_document()->(Vec<Vec<i32>>,Vec<Vec<i32>>){
    let mut winners:Vec<Vec<i32>> = Vec::new();
    let mut numbers:Vec<Vec<i32>> = Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day4test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day4input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let row = line.expect("Could not read file");
        let split1:Vec<&str> = row.split(": ").collect();
        if split1.len()>1{
            let split2:Vec<&str> = split1[1].split(" | ").collect();
            let mut win_row:Vec<i32> = Vec::new();
            let mut num_row:Vec<i32> = Vec::new();
            let win_split:Vec<&str> = split2[0].split(" ").collect();
            let num_split:Vec<&str> = split2[1].split(" ").collect();

            for w in win_split{
                if w.parse::<i32>().is_ok(){win_row.push(FromStr::from_str(w).unwrap())}
            }
            for n in num_split{
                if n.parse::<i32>().is_ok(){num_row.push(FromStr::from_str(n).unwrap())}
            }

            winners.push(win_row);
            numbers.push(num_row);
        }
    }
    return (winners,numbers);
}