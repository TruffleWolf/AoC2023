use std::fs::File;
use std::io::{BufRead,BufReader};
use std::cmp;

pub fn part1(){
    println!("Day2 Part1");

    let game_list:Vec<Vec<[i32;3]>> = parse_game();

    let mut total:i32 = 0;
    let mut count:i32 = 1;
    for g in game_list{
        let mut possible:bool = true;
        for game in g{
            //println!("{}/{}/{}",game[0],game[1],game[2]);
            possible = possible && game[0]<13 && game[1]<14 && game[2]<15;
        }
        if possible{total+=count;}
        count+=1;
    }
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day2 Part2");

    let game_list:Vec<Vec<[i32;3]>> = parse_game();
    let mut total:i32 = 0;
    for g in game_list{
        let mut rgb:[i32;3] = [0,0,0];
        for game in g{
            rgb[0] = cmp::max(rgb[0],game[0]);
            rgb[1] = cmp::max(rgb[1],game[1]);
            rgb[2] = cmp::max(rgb[2],game[2]);
        }
        //println!("{}/{}/{}",rgb[0],rgb[1],rgb[2]);
        total +=rgb[0]*rgb[1]*rgb[2];
    }
    println!("Total:{total}");
}

fn parse_game()->Vec<Vec<[i32;3]>>{
    let mut games:Vec<Vec<[i32;3]>> =Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day2test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day2input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    for line in reader.lines(){

        let row:String = line.expect("failed to read line");
        let slice1:Vec<&str> = row.split(": ").collect();
        if slice1.len()>1{
            let game_steps:Vec<&str> = slice1[1].split("; ").collect();
            let mut rgb_steps:Vec<[i32;3]> = Vec::new();
            for g in game_steps{
                let mut rgb:[i32;3]=[0,0,0];
                let rgb_strings:Vec<&str> = g.split(", ").collect();
                for r in rgb_strings{
                    if r.contains("red"){
                        let red:Vec<&str> = r.split(" ").collect();
                        rgb[0]=red[0].parse().expect("could not parse red");
                    }
                    else if r.contains("green"){
                        let green:Vec<&str> = r.split(" ").collect();
                        rgb[1]=green[0].parse().expect("could not parse green");
                    }
                    else if r.contains("blue"){
                        let blue:Vec<&str> = r.split(" ").collect();
                        rgb[2]=blue[0].parse().expect("could not parse blue");
                    }
                    else {println!("Failed to find color.");}
                }

                rgb_steps.push(rgb);
            }
            games.push(rgb_steps);
        }
    }
    return games;
}