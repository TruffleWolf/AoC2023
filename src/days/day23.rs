use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use crate::vec2;

pub fn part1(){
    println!("Day23 Part1");
    let char_grid = parse_document();

    let mut seen_tiles:Vec<Vec<[i16;2]>>=Vec::new();
    let seen_list:Vec<[i16;2]>=Vec::new();
    seen_tiles.push(seen_list);
    let mut current_tile:Vec<[i16;2]>=Vec::new();
    current_tile.push([1,0]);
    let mut dist_traveled:Vec<u32>=vec![0];
    let mut step:usize = 0;
    let end:[i16;2]=[char_grid[0].len()as i16-2,char_grid.len()as i16-1];

    let mut end_list:Vec<u32>=Vec::new();

    while step<current_tile.len(){
        seen_tiles[step].push(current_tile[step]);
        dist_traveled[step]+=1;
        let n_tile:[i16;2]=vec2::addition(&current_tile[step], &vec2::north());
        let s_tile:[i16;2]=vec2::addition(&current_tile[step], &vec2::south());
        let e_tile:[i16;2]=vec2::addition(&current_tile[step], &vec2::east());
        let w_tile:[i16;2]=vec2::addition(&current_tile[step], &vec2::west());
        let mut next_steps:Vec<[i16;2]>=Vec::with_capacity(3);
        let mut next_dist:Vec<u32>=Vec::with_capacity(3);
        //N
        if vec2::in_bounds(&n_tile, &char_grid) && !seen_tiles[step].contains(&n_tile){
            match char_grid[n_tile[1]as usize][n_tile[0]as usize]{
                '.'=>{
                    next_steps.push(n_tile);
                    next_dist.push(0);
                },
                '^'=>{
                    seen_tiles[step].push(n_tile);
                    next_steps.push(vec2::addition(&n_tile, &vec2::north()));
                    next_dist.push(1);
                },
                _=>(),
            }
        }
        //S
        if vec2::in_bounds(&s_tile, &char_grid) && !seen_tiles[step].contains(&s_tile){
            if s_tile == end{
                end_list.push(dist_traveled[step]);
                //println!("end Found:{}",dist_traveled[step]);
                step+=1;
                continue;
            }
            match char_grid[s_tile[1]as usize][s_tile[0]as usize]{
                '.'=>{
                    next_steps.push(s_tile);
                    next_dist.push(0);
                },
                'v'=>{
                    seen_tiles[step].push(s_tile);
                    next_steps.push(vec2::addition(&s_tile, &vec2::south()));
                    next_dist.push(1);
                },
                _=>(),
            }
        }
        //E
        if vec2::in_bounds(&e_tile, &char_grid) && !seen_tiles[step].contains(&e_tile){
            match char_grid[e_tile[1]as usize][e_tile[0]as usize]{
                '.'=>{
                    next_steps.push(e_tile);
                    next_dist.push(0);
                },
                '>'=>{
                    seen_tiles[step].push(e_tile);
                    next_steps.push(vec2::addition(&e_tile, &vec2::east()));
                    next_dist.push(1);
                },
                _=>(),
            }
        }
        //W
        if vec2::in_bounds(&w_tile, &char_grid) && !seen_tiles[step].contains(&w_tile){
            match char_grid[w_tile[1]as usize][w_tile[0]as usize]{
                '.'=>{
                    next_steps.push(w_tile);
                    next_dist.push(0);
                },
                '<'=>{
                    seen_tiles[step].push(w_tile);
                    next_steps.push(vec2::addition(&w_tile, &vec2::west()));
                    next_dist.push(1);
                },
                _=>(),
            }
        }

        if next_steps.len()==0{
            //println!("Terminated");
            step+=1;
        }
        else{
            
            if next_steps.len()>1{
                for i in 1..next_steps.len(){
                    current_tile.push(next_steps[i]);
                    seen_tiles.push(seen_tiles[step].clone());
                    dist_traveled.push(dist_traveled[step]+next_dist[i])
                }
            }
            current_tile[step]=next_steps[0];
            dist_traveled[step]+=next_dist[0];
        }
    }

    let mut total:u32 = 0;
    for e in end_list{
        if e > total{
            total = e;
        }
    }
    println!("Total:{}",total);
}

pub fn part2(){
    println!("Day23 Part2");
    let char_grid = parse_document();


    let end:[i16;2]=[char_grid[0].len()as i16-2,char_grid.len()as i16-1];
    let mut graph = create_graph(char_grid);
    
    let new_list:Vec<[i16;3]>=Vec::new();
    let mut total:i16 = 0;
    let mut pos_list:Vec<[i16;2]>=vec![[1,0]];
    let mut len_list:Vec<i16>=vec![0];
    let seen_row:Vec<[i16;2]>=vec![[1,0]];
    let mut seen_list:Vec<Vec<[i16;2]>>=vec![seen_row];

    while pos_list.len()>0{
        let prev_step = pos_list.pop().unwrap();
        let length = len_list.pop().unwrap();
        let seen = seen_list.pop().unwrap();
        if prev_step == end{
            if length>total{
                total=length;
                println!("{}",total);
                continue;
            }
        }
        for next in graph.entry(prev_step).or_insert(new_list.clone()){
            if !seen.contains(&[next[0],next[1]]){
                pos_list.push([next[0],next[1]]);
                len_list.push(length+next[2]);
                let mut new_seen = seen.clone();
                new_seen.push([next[0],next[1]]);
                seen_list.push(new_seen);
            }
        }
    }

    println!("Total:{}",total);

}

fn create_graph(grid:Vec<Vec<char>>)->HashMap<[i16;2],Vec<[i16;3]>>{

    let mut graph:HashMap<[i16;2],Vec<[i16;3]>>=HashMap::new();
    let mut parsing:Vec<[i16;2]>=vec![[1,0]];
    let mut seen:Vec<[i16;2]>=Vec::new();
    let new_list:Vec<[i16;3]>=Vec::new();
    while parsing.len()>0{
        let point = parsing.pop().unwrap();
        if seen.contains(&point){continue;}
        graph.insert(point, new_list.clone());

        let mut next_steps:Vec<[i16;2]>=Vec::new();
        for dir in vec2::cardinals(){
            let new_pos = vec2::addition(&point, &dir);
            if vec2::in_bounds(&new_pos, &grid)&&grid[new_pos[1]as usize][new_pos[0]as usize]!='#'{
                next_steps.push(new_pos);
            }
        }

        for step in next_steps{
            let mut length:i16 = 1;
            let mut old_step = point;
            let mut cur_pos = step;

            loop{
                let mut new_steps:Vec<[i16;2]>=Vec::new();
                for dir in vec2::cardinals(){
                    let new_pos = vec2::addition(&cur_pos, &dir);
                    if vec2::in_bounds(&new_pos, &grid)&&grid[new_pos[1]as usize][new_pos[0]as usize]!='#'{
                        new_steps.push(new_pos);
                    }
                }
                if new_steps.len()!=2{
                    break;
                }
                for pos in new_steps{
                    if pos != old_step{
                        length+=1;
                        old_step = cur_pos;
                        cur_pos = pos;
                        break;
                    }
                }
            }
            let cur_list = graph.entry(point).or_insert(new_list.clone());
            cur_list.push([cur_pos[0],cur_pos[1],length]);
            parsing.push(cur_pos);
        }
        seen.push(point);
    }
    return graph;
}


fn parse_document()->Vec<Vec<char>>{
    let mut output:Vec<Vec<char>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day23test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day23input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let new_line:Vec<char> = line_str.chars().collect();
        output.push(new_line);
    }

    return output;
}