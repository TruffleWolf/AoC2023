use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::str::FromStr;

pub fn part1(){
    println!("Day8 Part1");

    let (move_list,network)=parse_document();
    let mut node:String = String::from("AAA");
    let mut count:usize = 0;
    while node != String::from("ZZZ"){
        let next_move:usize = move_list[count%move_list.len()];
        let step = match network.get(&node){
            Some(x)=>x,
            None=>{panic!("Failed to find node");}
        };
        node = step[next_move].clone();
        count+=1;
    }
    println!("Total:{count}");
    
}

pub fn part2(){
    println!("Day8 Part2");
    let (move_list,network)=parse_document();
    let mut node_list:Vec<String>=Vec::new();
    let mut node_counts:Vec<[usize;2]>=Vec::new();
    for k in network.keys(){
        let end = match k.get(2..3){
            Some(x)=>x,
            None=>"",
        };
        if end == "A"{ 
            node_list.push(k.clone());
            node_counts.push([0,0]);
        }
    }
    
    
    for i in 0..node_list.len(){
        let mut end = match node_list[i].get(2..3){
            Some(x)=>x,
            None=>"",
        };
        let mut count:usize = 0;
        while end != "Z"{
            let next_move:usize = move_list[count%move_list.len()];
            let step = match network.get(&node_list[i]){
                Some(x)=>x,
                None=>{panic!("Failed to find node");}
            };
            node_list[i] = step[next_move].clone();
            end = match node_list[i].get(2..3){
                Some(x)=>x,
                None=>"",
            };
            count+=1;
        }
        node_counts[i][0]= count;
        // this second while loop setup is technically not needed as the path from a to z is always identical as z to z
        // it has been left in as there is nothing stopping an input from creating a shorter or longer a to z path
        // let mut new_count:usize = 1;
        // let next_move:usize = move_list[count%move_list.len()];
        //     let step = match network.get(&node_list[i]){
        //         Some(x)=>x,
        //         None=>{panic!("Failed to find node");}
        //     };
        //     node_list[i] = step[next_move].clone();
        //     end = match node_list[i].get(2..3){
        //         Some(x)=>x,
        //         None=>"",
        //     };


        // count+=1;
        // while end != "Z"{
        //     let next_move:usize = move_list[count%move_list.len()];
        //     let step = match network.get(&node_list[i]){
        //         Some(x)=>x,
        //         None=>{panic!("Failed to find node");}
        //     };
        //     node_list[i] = step[next_move].clone();
        //     end = match node_list[i].get(2..3){
        //         Some(x)=>x,
        //         None=>"",
        //     };
        //     new_count+=1;
        //     count+=1;
        // }
        // node_counts[i][1]=new_count;
    }  
    let factor = find_prime_factor(node_counts[0][0]);
    let mut total:i64 = node_counts[0][0]as i64/factor;
    for n in 1..node_counts.len(){
        //println!("{},{}",node_counts[n][0],node_counts[n][1]);
        total = total * (node_counts[n][0]as i64/factor);
    }
    println!("Total:{}",total*factor);
}

fn find_prime_factor(input:usize)->i64{
    let mut largest_prime:i64  =-1;
    let mut value:i64 = input as i64;
    while value%2 ==0{
        largest_prime = 2;
        value /= 2;
    }

    let mut i:i64 = 3;
    while i*i <= value{
        while value%i == 0{
            largest_prime = i;
            value /= i;
        }
        i+=2
    }

    if value >2{
        largest_prime = value
    }
    return largest_prime;
}

fn parse_document()->(Vec<usize>,HashMap<String,[String;2]>){
    let mut move_list:Vec<usize> = Vec::new();
    let mut network:HashMap<String,[String;2]> = HashMap::new();
    //let input_doc = File::open("./src/inputs//test/Day8test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day8input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        if move_list.len() ==0{
            for l in line_str.chars(){
                if l =='L'{move_list.push(0);}
                else if l=='R'{move_list.push(1);}
            }
            continue;
        }
        if line_str.len()>2{
            let line_str = String::from_str(line_str.trim()).expect("failed to trim string");
            let split1:Vec<&str> = line_str.split(" = ").collect();
            let split2:Vec<&str> = split1[1].split(", ").collect();
            let left_path = match split2[0].strip_prefix("("){
                Some(x)=>x,
                None=>{println!("failed to strip (");"ZZZ"}
            };
            let right_path = match split2[1].strip_suffix(")"){
                Some(x)=>x,
                None=>{println!("failed to strip )");"ZZZ"}
            };
            network.insert(String::from_str(split1[0]).expect("Failed to key"), [String::from_str(left_path).expect("Failed to value0"),String::from_str(right_path).expect("Failed to value1")]);
        }
    }

    return (move_list,network);
}