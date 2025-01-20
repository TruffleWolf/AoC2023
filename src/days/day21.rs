use std::fs::File;
use std::io::{BufRead,BufReader};
use crate::vec2;
//use std::collections::HashSet;


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
    //let char_grid = parse_document();

    
    // let target:i64 = 26501365;
    // let length:i64 = char_grid.len() as i64;
    // let half:i64 = length/2;
    
    // let mut start_loc:[i16;2]=[0,0];
    // 'y: for y in 0..char_grid.len(){
    //     for x in 0..char_grid[y].len(){
    //         if char_grid[y][x] == 'S'{
    //             start_loc = [x as i16,y as i16];
    //             break 'y;
    //         }
    //     }
    // }


    // let mut steps:Vec<i64>=Vec::with_capacity(3);
    // for i in 0..3{
    //     let count = half+i*length;
    //     //let mut addition:i64 = 0;
    //     let map = search(&char_grid, &start_loc, count);
    //     // for (distance,quant)in map{
    //     //     if distance%2==count%2{addition+=quant;}
    //     // }
    //     steps.push(map);
    //     println!("{}/{}",i,map);
    // }


    // steps.push(3957);
    // steps.push(35218);
    // steps.push(97615);
    // println!("{:?}",steps);

    // let total = parse_quad( [steps[0],steps[1],steps[2]],target/length);

    //[65,196,327]
    //[3694,34471,96363]
    //[3957, 35218, 97615]mk2
    //let total:u64 = 3694+34471*202300+202300*(202300-1)*96363/2;
    //println!("Total:{}",total);
    

}

// fn search(grid:&Vec<Vec<char>>,start:&[i16;2],depth:i64)->i64{
//     println!("Depth:{depth}");
//     let mut seen:HashSet<[i16;2]>=HashSet::new();
//     seen.insert(start.clone());
//     for i in 0..depth{
//         let mut new_seen:HashSet<[i16;2]>=HashSet::new();
//         if i%10 ==0{println!("{i}");}
//         for loc in &seen{

//             for dir in vec2::cardinals(){
//                 let new_step:[i16;2]= [loc[0]+dir[0],loc[1]+dir[1]];
//                 if grid[new_step[1]as usize %grid.len()][new_step[0]as usize%grid[0].len()]!='#'{
//                     new_seen.insert(new_step);
                    
//                 }
//             }
            
//         }
//         seen = new_seen;
//     }

//     return seen.len() as i64;
// }



// fn parse_quad(steps:[i64;3],part:i64)->i64{

//     let a = steps[2]-2*steps[1]+steps[0];
//     let b = 4*steps[1]-3*steps[0]-steps[2];
//     let c = steps[0];
//     return (a*(part*part)+b*part)/2+c;
    
// }

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