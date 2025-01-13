use std::fs::File;
use std::io::{BufRead,BufReader};
use std::cmp;
use crate::vec2::{self, in_bounds};

pub fn part1(){
    println!("Day18 Part1");
    let mut grid = parse_document();
    let mut total:u64 = grid.len()as u64*grid[0].len()as u64;
    let x_size = grid.len()-1;
    println!("{total}");
    for x in 0..grid[0].len(){
        if grid[0][x].len()<1{
            total-=fill_empty(&mut grid, [x as i16,0]);
        }
        if grid[x_size][x].len()<1{
            total-=fill_empty(&mut grid, [x as i16,x_size as i16]);
        }
    }
    println!("{total}");
    let y_size = grid[0].len()-1;
    for y in 0..grid.len(){
        if grid[y][0].len()<1{
            total-=fill_empty(&mut grid, [0,y as i16]);
        }
        if grid[y][y_size].len()<1{
            total-=fill_empty(&mut grid, [y_size as i16,y as i16]);
        }
    }
    
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day18 Part2");
    let commands = parse_document_2();
    let mut total:f64=1.0;
    let mut x_pos:i64 = 0;
    for (d,v) in commands{
        //println!("{v}");
        x_pos += d[0]as i64*v;
        total += d[1]as f64*v as f64*x_pos as f64 +(v as f64/2.0);
    }
    println!("Total:{}",total);
}

fn fill_empty(grid:&mut Vec<Vec<String>>,pos:[i16;2])->u64{
    let mut to_fill:Vec<[i16;2]> = vec![pos];
    let mut filled:u64=0;
    while to_fill.len()>0{
        let mut next_fill:Vec<[i16;2]>=Vec::new();
        for t in to_fill{
            grid[t[1]as usize][t[0]as usize]=String::from(".");
            filled+=1;
            for dir in vec2::cardinals(){
                let new_pos:[i16;2]=[t[0]+dir[0],t[1]+dir[1]];
                if in_bounds(&new_pos, &grid){
                    if grid[new_pos[1]as usize][new_pos[0]as usize].len()<1 && !next_fill.contains(&new_pos){
                        next_fill.push(new_pos);
                    }
                }
            }
        }
        //println!("{:?}",next_fill);
        to_fill = next_fill;
    }
    return filled;
}

fn parse_document()->Vec<Vec<String>>{
    //let input_doc = File::open("./src/inputs//test/Day18test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day18input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut cur_h:i32=0;
    let mut cur_v:i32=0;
    let mut max_h:i32=0;
    let mut max_v:i32=0;
    let mut min_h:i32=0;
    let mut min_v:i32=0;
    let mut command_list:Vec<String> = Vec::new();
    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        command_list.push(line_str.clone());
        let new_line:Vec<&str> = line_str.split(" ").collect();
        let change:u32 = new_line[1].parse::<u32>().unwrap();
        match new_line[0]{
            "R"=>{
                cur_h+=change as i32;
                max_h=cmp::max(max_h, cur_h);
                
            },
            "L"=>{
                cur_h-=change as i32;
                min_h=cmp::min(min_h, cur_h);
            },
            "U"=>{
                cur_v -= change as i32;
                min_v = cmp::min(min_v,cur_v);
            },
            "D"=>{
                cur_v+= change as i32;
                max_v = cmp::max(cur_v,max_v);
            },
            _=>println!("INVALD CHAR"),
        }
        
    }
    //println!("UP{},DOWN{},LEFT{},RIGHT{}",min_v,max_v,min_h,max_h);
    let mut grid:Vec<Vec<String>>=Vec::with_capacity((max_v-min_v)as usize);
    for _y in 0..max_v-min_v+1{
        let mut row:Vec<String>=Vec::with_capacity((max_h-min_h)as usize);
        for _x in 0..max_h-min_h+1{
            row.push(String::from(""));
        }
        grid.push(row);
    }
    let mut current_pos = [min_h.abs()as usize,min_v.abs()as usize];
    grid[current_pos[1]][current_pos[0]]=String::from("START");
    for line in command_list{
        let new_line:Vec<&str> = line.split(" ").collect();
        let change:u32 = new_line[1].parse::<u32>().unwrap();
        
        match new_line[0]{
            "R"=>{
                for _c in 1..=change{
                    current_pos[0]+=1;
                    grid[current_pos[1]][current_pos[0]]=String::from(new_line[2]);
                }
            },
            "L"=>{
                for _c in 1..=change{
                    current_pos[0]-=1;
                    grid[current_pos[1]][current_pos[0]]=String::from(new_line[2]);
                }
            },
            "U"=>{
                for _c in 1..=change{
                    current_pos[1]-=1;
                    grid[current_pos[1]][current_pos[0]]=String::from(new_line[2]);
                }
            },
            "D"=>{
                for _c in 1..=change{
                    current_pos[1]+=1;
                    grid[current_pos[1]][current_pos[0]]=String::from(new_line[2]);
                }
            },
            _=>println!("INVALD CHAR"),
        }
    }

    return grid;
}

fn parse_document_2()->Vec<([i16;2],i64)>{
    let mut output:Vec<([i16;2],i64)>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day18test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day18input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str:String = line.expect("Could not parse line");
        let split:Vec<&str> = line_str.split(" ").collect();
        let command:Vec<char>= split[2].chars().collect();
        let directions:Vec<[i16;2]> = vec![[1,0],[0,1],[-1,0],[0,-1]];
        let index = command[command.len()-2].to_digit(10).unwrap();
        let dir = directions[index as usize];
        let mut new_string=String::new();
        for i in 2..command.len()-2{
            new_string.push(command[i]);
        }
        let value = i64::from_str_radix(&new_string, 16).expect("Failed to parse");
        output.push((dir,value));
    }
    return output;
}