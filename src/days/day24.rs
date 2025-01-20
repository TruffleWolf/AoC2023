use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn part1(){
    println!("Day24 Part1");
    let (pos_data,vel_data)=parse_document();
    
    let mut total:u32 = 0;
    let test_area = [200000000000000.0,400000000000000.0];
    for a in 0..pos_data.len(){
        for b in a+1..pos_data.len(){
            if a!=b{
                let denom:f64 = vel_data[b][1]*vel_data[a][0]-vel_data[b][0]*vel_data[a][1];
                if denom==0.0{continue;}//parallel
                let coords = [pos_data[a][0]-pos_data[b][0],pos_data[a][1]-pos_data[b][1]];
                let off:f64 = (vel_data[b][0]*coords[1]-vel_data[b][1]*coords[0])/denom;
                let intersect = [pos_data[a][0]+off*vel_data[a][0],pos_data[a][1]+off*vel_data[a][1]];
                //println!("{}/{}-{:?} From off{}",a,b,intersect,off);
                if intersect[0]>=test_area[0]&&intersect[0]<=test_area[1]&&intersect[1]>=test_area[0]&&intersect[1]<=test_area[1]{
                    

                    if (intersect[0]-pos_data[a][0])*vel_data[a][0]>0.0&&(intersect[1]-pos_data[a][1])*vel_data[a][1]>0.0{
                        // in A's future
                        if (intersect[0]-pos_data[b][0])*vel_data[b][0]>0.0&&(intersect[1]-pos_data[b][1])*vel_data[b][1]>0.0{
                            //in B's future
                            total+=1;
                        }
                    }

                }
            }
        }
    }
    println!("Total:{}",total);
}

pub fn part2(){
    
    println!("Day24 Part2");

//     pts = ToExpression["{" <> StringReplace[#, "@" -> ","] <> "}"] & /@ 
//     Import["D:\\CodeRepos\\Rust\\AoC2023\\src\\inputs\\Day24input.txt",
//       "Lines"];
 
//  AtTime[pt_, t_] := {pt[[1]] + t*pt[[4]], pt[[2]] + t*pt[[5]], pt[[3]] + t*pt[[6]]}
//  magicRock = {sx, sy, sz, dx, dy, dz};
//  part2solution = 
//    Solve[(AtTime[magicRock, t1] == AtTime[pts[[1]], t1] && 
//       AtTime[magicRock, t2] == AtTime[pts[[2]], t2] && 
//       AtTime[magicRock, t3] == AtTime[pts[[3]], t3]), {sx, sy, sz, dx, 
//      dy, dz, t1, t2, t3}];
 
// Print["Part 2 is ", sx + sy + sz /. part2solution[[1]]];

    println!("Solved with Mathematica, will not work with alt inputs");
    println!("Total:871983857253169");
    

}


fn parse_document()->(Vec<[f64;3]>,Vec<[f64;3]>){

    //let input_doc = File::open("./src/inputs//test/Day24test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day24input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut pos:Vec<[f64;3]>=Vec::new();
    let mut vel:Vec<[f64;3]>=Vec::new();
    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let split1:Vec<&str> = line_str.split(" @ ").collect();
        let pos_split:Vec<&str>=split1[0].split(", ").collect();
        let vel_split:Vec<&str>=split1[1].split(", ").collect();
        let mut new_pos:[f64;3]=[0.0,0.0,0.0];
        let mut new_vel:[f64;3]=[0.0,0.0,0.0];
        
        for p in 0..pos_split.len(){
            let new_str = pos_split[p].trim();
            new_pos[p]=new_str.parse::<f64>().expect("Failed to parse position.");
        }
        
        for v in 0..vel_split.len(){
            let new_str = vel_split[v].trim();
            new_vel[v]=new_str.parse::<f64>().expect("Failed to parse veloctiy.");
        }
        pos.push(new_pos);
        vel.push(new_vel);
    }

    return (pos,vel);
}