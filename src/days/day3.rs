use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;
use crate::vec2;

pub fn part1(){
    println!("Day3 Part1");

    let (schematic,mut parsed_grid) = parse_document();
    let mut numbers:Vec<i32> = Vec::new();
    for y in 0..schematic.len(){
        for x in 0..schematic[y].len(){
            if !(schematic[y][x].is_digit(10) || schematic[y][x]=='.'){
                parsed_grid[y][x] = true;
                for v in vec2::full_compass(){
                    let new_pos:[usize;2] = [(x as i16 - v[0])as usize,(y as i16 - v[1])as usize];
                    if vec2::in_bounds(&[(x as i16 - v[0]),(y as i16 - v[1])], &parsed_grid){
                        if !parsed_grid[new_pos[1]][new_pos[0]] && schematic[new_pos[1]][new_pos[0]].is_ascii_digit(){
                            let new_number = parse_number(&new_pos,&schematic,&mut parsed_grid);
                            numbers.push(new_number);
                        }
                    }
                }
            }
        }
    }
    let mut total:i32 = 0;
    for n in numbers{
        //println!("{n}");
        total+=n;
    }
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day3 Part2");

    let (schematic,mut parsed_grid) = parse_document();
    let mut numbers:Vec<i32> = Vec::new();

    for y in 0..schematic.len(){
        for x in 0..schematic[y].len(){
            if schematic[y][x]=='*'{
                parsed_grid[y][x] = true;
                let mut adj_ints:Vec<i32>=Vec::new();
                for v in vec2::full_compass(){
                    let new_pos:[usize;2] = [(x as i16 - v[0])as usize,(y as i16 - v[1])as usize];
                    if vec2::in_bounds(&[(x as i16 - v[0]),(y as i16 - v[1])], &parsed_grid){
                        if !parsed_grid[new_pos[1]][new_pos[0]] && schematic[new_pos[1]][new_pos[0]].is_ascii_digit(){
                            let new_number = parse_number(&new_pos,&schematic,&mut parsed_grid);
                            adj_ints.push(new_number);
                        }
                    }
                }
                if adj_ints.len()==2{
                    numbers.push(adj_ints[0]*adj_ints[1]);
                }
                

            }
        }
    }

    let mut total:i32 = 0;
    for n in numbers{
        //println!("{n}");
        total+=n;
    }
    println!("Total:{total}");
    
}

fn parse_number(pos:&[usize;2],grid:&Vec<Vec<char>>,parser:&mut Vec<Vec<bool>>)->i32{
    let mut numb_str = String::from(grid[pos[1]][pos[0]]);
    let mut x_step:i16 = pos[0] as i16+1;
    parser[pos[1]][pos[0]] = true;
    while vec2::in_bounds(&[x_step,pos[1] as i16], grid) && grid[pos[1]][x_step as usize].is_ascii_digit() && !parser[pos[1]][x_step as usize]{
        numb_str.push(grid[pos[1]][x_step as usize]);
        parser[pos[1]][x_step as usize] = true;
        x_step+=1;
    }
    x_step = pos[0]as i16-1;
    while vec2::in_bounds(&[x_step,pos[1] as i16], grid) && grid[pos[1]][x_step as usize].is_ascii_digit() && !parser[pos[1]][x_step as usize]{
        numb_str.insert(0, grid[pos[1]][x_step as usize]);
        parser[pos[1]][x_step as usize] = true;
        x_step-=1;
    }
    let final_int:i32 = FromStr::from_str(&numb_str).expect("Failed to convert to i32");
    return final_int;
}

fn parse_document()->(Vec<Vec<char>>,Vec<Vec<bool>>){
    let mut schematic:Vec<Vec<char>> = Vec::new();
    let mut grid:Vec<Vec<bool>> = Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day3test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day3input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    for line in reader.lines(){
        let row:Vec<char> = match line {
            Ok(s)=>s.chars().collect(),
            Err(_)=>{println!("Failed to read row");break}
        };
        let mut g_line:Vec<bool> = Vec::with_capacity(row.len());
        for _r in &row{
            g_line.push(false);
        }
        grid.push(g_line);
        schematic.push(row);
    }

    return (schematic,grid);
}