
use std::cmp;
use std::fs::File;
use std::io::{BufRead,BufReader};
use crate::vec2;

pub fn part1(){
    println!("Day16 Part1");
    let base_grid = parse_document();
    
    let start_location:[i16;4]=[0,0,1,0];
    println!("Total:{}",get_energy(start_location, &base_grid))
}

pub fn part2(){
    println!("Day16 Part2");
    let base_grid = parse_document();
    let mut max_energy:u32 = 0;
    let mut starts:Vec<[i16;4]>=Vec::new();

    for y in 0..base_grid.len()as i16{
        let start_a:[i16;4]=[0,y,1,0];
        let start_b:[i16;4]=[base_grid[y as usize].len()as i16-1,y,-1,0];
        starts.push(start_a);
        starts.push(start_b);
    }
    for x in 0..base_grid[0].len()as i16{
        let start_a:[i16;4]=[x,0,0,1];
        let start_b:[i16;4]=[x,base_grid.len()as i16-1,0,-1];
        starts.push(start_a);
        starts.push(start_b);
    }

    for s in starts{
        max_energy = cmp::max(max_energy, get_energy(s, &base_grid))
    }

    println!("Best:{max_energy}")
}

fn get_energy(start_loc:[i16;4],grid:&Vec<Vec<char>>)->u32{

    let mut relevance_grid:Vec<Vec<bool>> = Vec::with_capacity(grid.len());

    for b in grid{
        let mut rel_row:Vec<bool> = Vec::with_capacity(b.len());
        for _c in b{
            rel_row.push(false);
        }
        relevance_grid.push(rel_row);
    }

    let mut known_spaces:Vec<[i16;4]>=Vec::new();
    let mut current_location:Vec<[i16;4]> = vec![start_loc];
    known_spaces.push(start_loc);
    relevance_grid[start_loc[1]as usize][start_loc[0]as usize]= true;
    let mut step:usize=0;
    
    while step<current_location.len(){
        known_spaces.push(current_location[step]);
        relevance_grid[current_location[step][1]as usize][current_location[step][0]as usize]=true;
        match grid[current_location[step][1]as usize][current_location[step][0]as usize]{
            '.'=>{
                //println!(".");
            },
            '/'=>{
                //println!("/");
                let current_dir:[i16;2]=[current_location[step][2],current_location[step][3]];
                match current_dir{
                    [0,1]=>{current_location[step][2]=-1;current_location[step][3]=0},
                    [0,-1]=>{current_location[step][2]=1;current_location[step][3]=0},
                    [1,0]=>{current_location[step][2]=0;current_location[step][3]=-1},
                    [-1,0]=>{current_location[step][2]=0;current_location[step][3]=1},
                    _=>println!("INVALID DIR FOUND {:?}",current_dir),
                }
            },
            '\\'=>{
                //println!("\\");
                let current_dir:[i16;2]=[current_location[step][2],current_location[step][3]];
                match current_dir{
                    [0,1]=>{current_location[step][2]=1;current_location[step][3]=0},
                    [0,-1]=>{current_location[step][2]=-1;current_location[step][3]=0},
                    [1,0]=>{current_location[step][2]=0;current_location[step][3]=1},
                    [-1,0]=>{current_location[step][2]=0;current_location[step][3]=-1},
                    _=>println!("INVALID DIR FOUND {:?}",current_dir),
                }
            },
            '-'=>{
                //println!("-");
                if current_location[step][1]!=0{
                    let new_pos_a:[i16;2]=[current_location[step][0]-1,current_location[step][1]];
                    let new_trans_a:[i16;4]=[current_location[step][0]-1,current_location[step][1],-1,0];
                    let new_pos_b:[i16;2]=[current_location[step][0]+1,current_location[step][1]];
                    let new_trans_b:[i16;4]=[current_location[step][0]+1,current_location[step][1],1,0];
                    if vec2::in_bounds(&new_pos_a, grid) && !known_spaces.contains(&new_trans_a){
                        
                        current_location.push(new_trans_a);
                    }
                    if vec2::in_bounds(&new_pos_b, grid) && !known_spaces.contains(&new_trans_b){
                        
                        current_location.push(new_trans_b);
                    }
                    step+=1;
                    continue;
                }
            },
            '|'=>{
                //println!("|");
                if current_location[step][0]!=0{
                    let new_pos_a:[i16;2]=[current_location[step][0],current_location[step][1]-1];
                    let new_trans_a:[i16;4]=[current_location[step][0],current_location[step][1]-1,0,-1];
                    let new_pos_b:[i16;2]=[current_location[step][0],current_location[step][1]+1];
                    let new_trans_b:[i16;4]=[current_location[step][0],current_location[step][1]+1,0,1];
                    if vec2::in_bounds(&new_pos_a, grid) && !known_spaces.contains(&new_trans_a){
                        known_spaces.push(known_spaces[step].clone());
                        current_location.push(new_trans_a);
                    }
                    if vec2::in_bounds(&new_pos_b, grid) && !known_spaces.contains(&new_trans_b){
                        known_spaces.push(known_spaces[step].clone());
                        current_location.push(new_trans_b);
                    }
                    step+=1;
                    continue;
                }

            },
            _=>{println!("INVALID CHAR DECTECTED AT {}/{}",current_location[step][0],current_location[step][1]);},
        }
        let new_pos:[i16;2] = [current_location[step][0]+current_location[step][2],current_location[step][1]+current_location[step][3]];
        let new_transform:[i16;4]=[new_pos[0],new_pos[1],current_location[step][2],current_location[step][3]];
        if vec2::in_bounds(&new_pos, grid) && !known_spaces.contains(&new_transform){
            current_location[step]=new_transform;
        }
        else{step+=1;}

    }

    let mut total:u32 = 0;
    for r in relevance_grid{
        for x in r{
            total+=x as u32;
        }
    }
    return total;

}

fn parse_document()->Vec<Vec<char>>{
    let mut output:Vec<Vec<char>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day16test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day16input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let new_line:Vec<char> = line_str.chars().collect();
        output.push(new_line);
    }

    return output;
}