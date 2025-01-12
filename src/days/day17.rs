use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::{BinaryHeap, HashMap};
use crate::vec2;
// garbage pathfinding code for losers, use the actual one
// pub fn part1(){
//     println!("Day17 Part1");
//     let heat_grid = parse_document();
//     let mut best_heat:Vec<Vec<[u32;3]>> = Vec::with_capacity(heat_grid.len());
//     for h in &heat_grid{
//         let mut new_row:Vec<[u32;3]> = Vec::with_capacity(h.len());
//         for _u in h{
//             new_row.push([999999999,999999999,999999999]);
//         }
//         best_heat.push(new_row);
//     }
//     let row = &best_heat[best_heat.len()-1];
//     let destination:[i16;2]=[(heat_grid.len()-1)as i16,(row.len()-1)as i16];
//     let mut current_location:Vec<[i16;2]>=vec![[0,0]];
//     let mut current_dir:Vec<[i16;2]>=vec![[0,0]];
//     //let initial_path:Vec<[i16;2]>=Vec::new();
//     //let mut known_paths:Vec<Vec<[i16;2]>>=vec![initial_path];
//     let mut known_scores:Vec<u32>=vec![0];
//     let mut dir_count:Vec<usize>=vec![0];
//     let step:usize = 0;
//     //let mut best_paths:Vec<Vec<[i16;2]>>=Vec::new();
//     while step < known_scores.len(){
//         //let mut dir:usize=0;
//         for v in vec2::cardinals(){
            
//             let new_loc = [current_location[step][0]+v[0],current_location[step][1]+v[1]];
//             if new_loc == destination {
//                 if best_heat[new_loc[1]as usize][new_loc[0]as usize][dir_count[step]]>=known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize]{
//                     best_heat[new_loc[1]as usize][new_loc[0]as usize][dir_count[step]]=known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize];
//                     //best_paths.push(known_paths[step].clone());
//                 }
//                 continue;
//             }
//             else if vec2::in_bounds(&new_loc, &heat_grid){
//                 if best_heat[new_loc[1]as usize][new_loc[0]as usize][dir_count[step]]>=known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize]{
//                 //if !known_paths[step].contains(&new_loc){
                
//                     if v == current_dir[step]{
//                         if dir_count[step]<2{
//                             current_location.push(new_loc);
//                             current_dir.push(v);
//                             known_scores.push(known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize]);
//                             dir_count.push(1+dir_count[step]);
//                             best_heat[new_loc[1]as usize][new_loc[0]as usize][dir_count[step]]=known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize];
//                             // let mut new_path =known_paths[step].clone();
//                             // new_path.push(new_loc);
//                             // known_paths.push(new_path);
//                         }
//                     }
//                     else if v!=[current_dir[step][0]*-1,current_dir[step][1]*-1]{
//                         current_location.push(new_loc);
//                         current_dir.push(v);
//                         known_scores.push(known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize]);
//                         dir_count.push(0);
//                         best_heat[new_loc[1]as usize][new_loc[0]as usize][dir_count[step]]=known_scores[step]+heat_grid[new_loc[1]as usize][new_loc[0]as usize];
//                         // let mut new_path =known_paths[step].clone();
//                         // new_path.push(new_loc);
//                         // known_paths.push(new_path);
//                     }
//                 }
//             }
//             //dir+=1;
//         }
//         //step+=1;
//         current_location.swap_remove(0);
//         current_dir.swap_remove(0);
//         known_scores.swap_remove(0);
//         dir_count.swap_remove(0);
//         //known_paths.swap_remove(0);
//     }
//     let row = &best_heat[best_heat.len()-1];
//     // for b in known_paths{
//     //     if b.contains(&[5,1]){println!("{:?}",b);}
//     // }
//     let total = row[row.len()-1];
//     println!("Total:{:?}",total);
// }
pub fn part1(){
    println!("Day17 Part1");
    let heat_grid = parse_document();
    
    let mut heats:HashMap<(i16,i16,[i16;2]),i64> = HashMap::new();
    let mut heap = BinaryHeap::from_iter([(0,(0,0,[0,0]))]);
    let mut total:i64 = -1;
    while total<0{
        let (cost,(x,y,dir))= heap.pop().unwrap();
        if (x,y)==(heat_grid[0].len() as i16-1,heat_grid.len() as i16-1){total=-cost;break;}
        
        if heats.get(&(x,y,dir)).is_some_and(|&i| -cost > i){
            continue;
        }
        for d in vec2::cardinals(){
            if dir == d || dir == [d[0]*-1,d[1]*-1]{continue;}
            let mut next_cost = -cost;
            for step in 1..4{
                let next_pos = [x+(d[0]*step),y+(d[1]*step)];
                if !vec2::in_bounds(&next_pos, &heat_grid){break;}
                next_cost += heat_grid[next_pos[1]as usize][next_pos[0]as usize] as i64;
                let new_key = (next_pos[0],next_pos[1],d);
                if next_cost< *heats.get(&new_key).unwrap_or(&i64::MAX){
                    heats.insert(new_key,next_cost);
                    heap.push((-next_cost,new_key));
                }
            }
        }  
        
    }
    println!("Total:{}",total);
}

pub fn part2(){
    println!("Day17 Part2");
    let heat_grid = parse_document();
    
    let mut heats:HashMap<(i16,i16,[i16;2]),i64> = HashMap::new();
    let mut heap = BinaryHeap::from_iter([(0,(0,0,[0,0]))]);
    let mut total:i64 = -1;
    while total<0{
        let (cost,(x,y,dir))= heap.pop().unwrap();
        if (x,y)==(heat_grid[0].len() as i16-1,heat_grid.len() as i16-1){total=-cost;break;}
        
        if heats.get(&(x,y,dir)).is_some_and(|&i| -cost > i){
            continue;
        }
        for d in vec2::cardinals(){
            if dir == d || dir == [d[0]*-1,d[1]*-1]{continue;}
            let mut next_cost = -cost;
            for step in 1..11{
                let next_pos = [x+(d[0]*step),y+(d[1]*step)];
                if !vec2::in_bounds(&next_pos, &heat_grid){break;}
                next_cost += heat_grid[next_pos[1]as usize][next_pos[0]as usize] as i64;
                if step<4{
                    continue;
                }
                let new_key = (next_pos[0],next_pos[1],d);
                if next_cost< *heats.get(&new_key).unwrap_or(&i64::MAX){
                    heats.insert(new_key,next_cost);
                    heap.push((-next_cost,new_key));
                }
            }
        }  
        
    }
    println!("Total:{}",total);
}

fn parse_document()->Vec<Vec<u32>>{
    let mut grid:Vec<Vec<u32>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day17test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day17input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let str_vec:Vec<&str> = line_str.split("").collect();
        let mut new_row:Vec<u32>=Vec::with_capacity(str_vec.len());
        for s in str_vec{
            if s.parse::<u32>().is_ok(){
                new_row.push(s.parse().expect("failed to parse u32"));
            }
        }
        grid.push(new_row);
    }
    return grid
}