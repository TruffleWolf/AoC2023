use std::fs::File;
use std::io::{BufRead,BufReader};
use crate::vec2;
// use std::collections::HashMap;
// use std::collections::VecDeque;

pub fn part1(){
    println!("Day21 Part1");
    let char_grid = parse_document();
    let mut relvance_grid:Vec<Vec<bool>>= Vec::with_capacity(char_grid.len());
    let mut start_loc:[i16;2]=[0,0];
    for y in 0..char_grid.len(){
        let mut row:Vec<bool>= Vec::with_capacity(char_grid[y].len());
        for x in 0..char_grid[y].len(){
            row.push(false);
            if char_grid[y][x] == 'S'{start_loc = [x as i16,y as i16];}
        }
        relvance_grid.push(row);
    }
    let mut parse_list:Vec<[i16;2]>=vec![start_loc];
    for _i in 0..64{
        let mut new_list:Vec<[i16;2]>=Vec::new();
        for p in &parse_list{
            relvance_grid[p[1]as usize][p[0]as usize]= true;
            new_list = append_next_steps(&char_grid, p, new_list);
        }
        parse_list = new_list;
    }

    println!("Total:{}",parse_list.len());
    
}

pub fn part2(){
    println!("Day21 Part2");
    let char_grid = parse_document();
    let target:i64 = 26501365;
    let length:i64 = char_grid.len() as i64;
    //let half:i64 = length/2;
    
    // let mut start_loc:[i16;2]=[0,0];
    // 'y: for y in 0..char_grid.len(){
    //     for x in 0..char_grid[y].len(){
    //         if char_grid[y][x] == 'S'{
    //             start_loc = [x as i16,y as i16];
    //             break 'y;
    //         }
    //     }
    // }

    // let map1 = search(&char_grid, &start_loc, 64);
    // let mut addition1:i64 = 0;
    // for (distance,quant) in map1{
    //     if distance%2==64%2{addition1+=quant;}
    //     //addition1+=quant;
    // }
    // println!("Part1:{}",addition1);
    // println!("{}",target%length);
    let mut steps:Vec<i64>=Vec::new();
    // for i in 0..3{
    //     let count = half+i*length;
    //     println!("remain{}",count%length);
    //     let mut addition:i64 = 0;
    //     let map = search(&char_grid, &start_loc, count);
    //     for (distance,quant)in map{
    //         if distance%2!=count%2{addition+=quant;}
    //     }
    //     steps.push(addition);
    //     println!("{}/{}",i,addition);
    // }

    // let mut list:Vec<[i16;2]>=vec![start_loc];
    // for i in 1..100000{
    //     let mut new_list:Vec<[i16;2]> = Vec::new();
    //     for loc in &list{
    //         for dir in vec2::cardinals(){
    //             let new_step:[i16;2]= [loc[0]+dir[0],loc[1]+dir[1]];
    //             if char_grid[new_step[1]as usize %char_grid.len()][new_step[0]as usize%char_grid[0].len()]!='#' && !new_list.contains(&new_step){
    //                 new_list.push(new_step);
    //             }
    //         }
    //     }
    //     list = new_list;
    //     if i%length == target%length{println!("{}/{}",i,list.len());}
    // }
    steps.push(3957);
    steps.push(35218);
    steps.push(97615);
    println!("{:?}",steps);
    //let total = parse_quad( [steps[0],steps[1],steps[2]],(target-half)/length);
    let total = parse_quad( [steps[0],steps[1],steps[2]],(target/length)+1);
    //let total:i64 = (quads[0]*26501365*26501365)+(quads[1]*26501365)+quads[2];
    //[65,196,327]
    //[3694,34471,96363]
    //[3957, 35218, 97615]mk2
    //let total:u64 = 3694+34471*202300+202300*(202300-1)*96363/2;
    println!("Total:{}",total);

}

// fn search(grid:&Vec<Vec<char>>,start:&[i16;2],depth:i64)->HashMap<i64,i64>{
//     let mut new_map:HashMap<i64,i64>=HashMap::new();
//     let mut seen:Vec<[i16;2]>=Vec::new();
//     let mut to_parse:VecDeque<[i16;2]>=VecDeque::new();
//     let mut to_count:VecDeque<i64>=VecDeque::new();
//     to_parse.push_back(start.clone());
//     to_count.push_back(0);
//     while to_parse.len()>0{
//         let loc = to_parse.pop_front().unwrap();
//         let count = to_count.pop_front().unwrap();
//         if count == depth+1 || seen.contains(&loc){continue;}
//         let slot = new_map.entry(count).or_insert(0);
//         *slot +=1;
//         seen.push(loc);
//         for dir in vec2::cardinals(){
//             let new_step:[i16;2]= [loc[0]+dir[0],loc[1]+dir[1]];
//             if grid[new_step[1]as usize %grid.len()][new_step[0]as usize%grid[0].len()]!='#'{
//                 to_parse.push_back(new_step);
//                 to_count.push_back(count+1);
//             }
//         }
//     }
//     return new_map;
// }

fn parse_quad(steps:[i64;3],part:i64)->i64{
    // let a = (steps[2]-(2*steps[1])+steps[0])/2;
    // let b = steps[1]-steps[0]-a;
    // let c = steps[0];
    // println!("{}/{}/{}",a,b,c);
    // return (a*part*part)+(b*part)+c;
    let a = steps[0];
    let b = steps[1]-steps[0];
    let c = steps[2]-steps[1];
    return a + b*part + (part*(part-1)/2)*(c-b);
    
}

fn append_next_steps(grid:&Vec<Vec<char>>,location:&[i16;2],known_list:Vec<[i16;2]>)->Vec<[i16;2]>{
    let mut new_list = known_list;
    for dir in vec2::cardinals(){
        let new_step:[i16;2]= [location[0]+dir[0],location[1]+dir[1]];
        if vec2::in_bounds(&new_step, &grid){
            if !new_list.contains(&new_step) && grid[new_step[1]as usize][new_step[0]as usize] != '#'{
                new_list.push(new_step);
            }
        }
    }

    return new_list;
}

fn parse_document()->Vec<Vec<char>>{
    let mut output:Vec<Vec<char>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day21test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day21input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let new_line:Vec<char> = line_str.chars().collect();
        output.push(new_line);
    }

    return output;
}