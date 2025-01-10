use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

pub fn part1(){
    println!("Day15 Part1");
    let hash = parse_document();
    let mut total:u32 = 0;
    for h in hash{
        total+=get_hash(&h);
    }
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day15 Part2");
    let hash = parse_document();
    let mut box_vec:Vec<Vec<String>>=Vec::new();
    let empty_vec:Vec<String> = Vec::new();
    for _i in 0..256{
        box_vec.push(empty_vec.clone());
    }

    let mut lens_map:HashMap<String,i64>=HashMap::new();
    for h in hash{
        
        let char_list:Vec<char> = h.chars().collect();
        let mut new_id:String = String::new();
        for i in 0..char_list.len()-2{
            new_id.push(char_list[i]);
        }
        if char_list[char_list.len()-1]=='-'{new_id.push(char_list[char_list.len()-2]);}
        let box_id = get_hash(&new_id) as usize;
        new_id.push_str(&box_id.to_string());
        println!("{new_id}");
        if char_list[char_list.len()-1]=='-'{
            if box_vec[box_id].contains(&new_id){
                let value = lens_map.entry(new_id.clone()).or_insert(0);
                *value = 0;
                let i = box_vec[box_id].iter().position(|x| *x==new_id).unwrap();
                box_vec[box_id].remove(i);
            }
        }
        else if char_list[char_list.len()-2]=='='{
            let value = lens_map.entry(new_id.clone()).or_insert(0);
            let new_value = char_list[char_list.len()-1].to_digit(10).unwrap();
            *value = new_value as i64;
            if !box_vec[box_id].contains(&new_id){box_vec[box_id].push(new_id.clone());}
        }
        else{println!("Invalid char detected");}
    }

    let mut total:i64=0;
    for i in 0..box_vec.len(){
        for x in 0..box_vec[i].len(){
            let value = ((i+1)*(x+1))as i64;
            let lens = match lens_map.get(&box_vec[i][x]){
                Some(x) => x,
                None=>{println!("Failed to find key"); &0},
            };
            total+= value * lens;
        }
    }
    println!("Total:{total}");
}

fn get_hash(input:&String)->u32{
    let mut value:u32 = 0;
    for ascii in input.as_bytes(){
        value += *ascii as u32;
        value *= 17;
        value = value%256;
    }
    return value;
}

fn parse_document()->Vec<String>{
    let mut total_vec:Vec<String>= Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day15test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day15input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let str_vec:Vec<&str> = line_str.split(',').collect();
        for s in str_vec{
            total_vec.push(String::from(s));
        }
    }
    return total_vec;
}