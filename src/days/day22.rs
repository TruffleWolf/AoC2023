use std::cmp;
use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn part1(){
    println!("Day22 Part1");
    let mut brick_list = parse_document();

    let mut falling:bool = true;
    println!("Parse complete");
    //drop all the bricks
    while falling{
        falling = false;
        for b in 0..brick_list.len(){
            let mut can_fall = true;
            'sq: for square in 0..brick_list[b].len(){
                if brick_list[b][square][2]==1{
                    can_fall= false;
                    break;
                }
                let new_square =[brick_list[b][square][0],brick_list[b][square][1],brick_list[b][square][2]-1];
                for alt_b in 0..brick_list.len(){
                    if alt_b!=b{
                        if brick_list[alt_b].contains(&new_square){
                            can_fall = false;
                            break 'sq;
                        }
                    }
                }
            }
            if can_fall{
                for s in 0..brick_list[b].len(){
                    brick_list[b][s][2]= brick_list[b][s][2]-1;
                }
                falling = true;
            }
        }
    }
    println!("fall complete");
    //calculate supports for each brick
    let mut supp_list:Vec<Vec<usize>>=Vec::new();
    let mut crit_list:Vec<u32>=Vec::new();
    for b in 0..brick_list.len(){
        let mut supports:Vec<usize> = Vec::new();

        for square in 0..brick_list[b].len(){
            if brick_list[b][square][2]==1{
                supports = Vec::new();
                break;
            }
            let new_square =[brick_list[b][square][0],brick_list[b][square][1],brick_list[b][square][2]-1];
            for alt_b in 0..brick_list.len(){
                if alt_b!=b{
                    if brick_list[alt_b].contains(&new_square){
                        if !supports.contains(&alt_b){
                            supports.push(alt_b);
                        }
                        break;
                    }
                }
            }
        }

        supp_list.push(supports);
        crit_list.push(0);
    }
    println!("Supports complete");
    //count the relevant bricks

    for s in &supp_list{
        if s.len()==1{
            crit_list[s[0]]+=1;
        }
    }
    let mut total:u32 = 0;
    for c in crit_list{
        if c == 0{total+=1;}
    }
    println!("Total:{}",total);

}

pub fn part2(){
    println!("Day22 Part2");
    let mut brick_list = parse_document();

    let mut falling:bool = true;
    println!("Parse complete");
    //drop all the bricks
    while falling{
        falling = false;
        for b in 0..brick_list.len(){
            let mut can_fall = true;
            'sq: for square in 0..brick_list[b].len(){
                if brick_list[b][square][2]==1{
                    can_fall= false;
                    break;
                }
                let new_square =[brick_list[b][square][0],brick_list[b][square][1],brick_list[b][square][2]-1];
                for alt_b in 0..brick_list.len(){
                    if alt_b!=b{
                        if brick_list[alt_b].contains(&new_square){
                            can_fall = false;
                            break 'sq;
                        }
                    }
                }
            }
            if can_fall{
                for s in 0..brick_list[b].len(){
                    brick_list[b][s][2]= brick_list[b][s][2]-1;
                }
                falling = true;
            }
        }
    }
    println!("fall complete");
    //calculate supports for each brick
    let mut supp_list:Vec<Vec<usize>>=Vec::new();
    let mut crit_list:Vec<u32>=Vec::new();
    for b in 0..brick_list.len(){
        let mut supports:Vec<usize> = Vec::new();

        for square in 0..brick_list[b].len(){
            if brick_list[b][square][2]==1{
                supports = Vec::new();
                break;
            }
            let new_square =[brick_list[b][square][0],brick_list[b][square][1],brick_list[b][square][2]-1];
            for alt_b in 0..brick_list.len(){
                if alt_b!=b{
                    if brick_list[alt_b].contains(&new_square){
                        if !supports.contains(&alt_b){
                            supports.push(alt_b);
                        }
                        break;
                    }
                }
            }
        }

        supp_list.push(supports);
        crit_list.push(0);
    }
    println!("Supports complete");
    //count the relevant bricks

    for s in &supp_list{
        if s.len()==1{
            crit_list[s[0]]+=1;
        }
    }
    let mut total = 0;
    for pos in 0..crit_list.len(){
        if crit_list[pos] != 0{
            let mut broken_list:Vec<usize> = vec![pos];
            let mut breaking:bool = true;
            while breaking{
                breaking = false;
                for s in 0..supp_list.len(){
                    if s != pos && supp_list[s].len()>0 && !broken_list.contains(&s){
                        let mut will_break = true;
                        for support in &supp_list[s]{
                            will_break = will_break && broken_list.contains(support);
                        }
                        if will_break{
                            breaking = true;
                            broken_list.push(s);
                        }
                    }
                }
            }

            total+=broken_list.len()-1;
        }
        println!("{pos}");
    }

    println!("Total:{}",total);

}


fn parse_document()->Vec<Vec<[i16;3]>>{
    let mut output:Vec<Vec<[i16;3]>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day22test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day22input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let mut new_brick:Vec<[i16;3]>=Vec::new();
        let split:Vec<&str> = line_str.split("~").collect();
        let start_split:Vec<&str> = split[0].split(",").collect();
        let end_split:Vec<&str> = split[1].split(",").collect();
        let mut start_point:[i16;3] = [0,0,0];
        let mut end_point:[i16;3]=[0,0,0];
        for s in 0..start_split.len(){
            start_point[s] = start_split[s].parse().expect("failed to parse start i16");
        }
        for s in 0..end_split.len(){
            end_point[s] = end_split[s].parse().expect("failed to parse end i16");
        }
        if start_point[0]!=end_point[0]{
            let start = cmp::min(start_point[0], end_point[0]);
            let end = cmp::max(start_point[0], end_point[0]);
            for i in start..=end{
                new_brick.push([i,start_point[1],start_point[2]]);
            }
        }   
        else if start_point[1]!= end_point[1]{
            let start = cmp::min(start_point[1], end_point[1]);
            let end = cmp::max(start_point[1], end_point[1]);
            for i in start..=end{
                new_brick.push([start_point[0],i,start_point[2]]);
            }
        }
        else if start_point[2]!= end_point[1]{
            let start = cmp::min(start_point[2], end_point[2]);
            let end = cmp::max(start_point[2], end_point[2]);
            for i in start..=end{
                new_brick.push([start_point[0],start_point[1],i]);
            }
        }
        else{
            new_brick.push(start_point);
        }
        output.push(new_brick);
    }

    return output;
}