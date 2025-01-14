use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn part1(){
    println!("Day19 Part1");
    let (map,parts)=parse_document();

    let mut total:i32=0;
    for p in parts{
        if parse_part(&map, p,String::from("in")){
            for v in p{
                total+=v;
            }
        }
    }
    println!("Total:{}",total);
}

pub fn part2(){
    println!("Day19 Part2");
    let (map,_parts)=parse_document();
    let base_range:[[i32;2];4]=[[1,4000],[1,4000],[1,4000],[1,4000]];
    let total:i64 = possibility_step(&map, base_range, String::from("in"));
    println!("Total:{}",total);
}

fn possibility_step(map:&HashMap<String,Vec<String>>,pos_space:[[i32;2];4],step:String)->i64{
    //println!("{}",step);
    let command = map.get(&step).unwrap();
    let mut possibility:i64=0;
    let mut cur_space = pos_space;
    for c in command{
        if c.contains(":"){
            let index = c[0..1].parse::<usize>().expect("failed to parse usize");
            let split:Vec<&str>= c.split(":").collect();
            let value = split[0][2..].parse::<i32>().expect("failed to parse i32");
            let mut new_space:[[i32;2];4]=cur_space;
            match &c[1..2]{
                ">"=>{
                    if new_space[index][1]>value{
                        new_space[index]=[value+1,new_space[index][1]];
                        cur_space[index]=[cur_space[index][0],value];
                        
                        match split[1]{
                            "A"=>possibility+=calc_possibilites(new_space),
                            "R"=>(),
                            _=>possibility+= possibility_step(map, new_space, String::from(split[1]))
                        }
                    }
                },
                "<"=>{
                    if new_space[index][0]<value{
                        new_space[index]=[new_space[index][0],value-1];
                        cur_space[index]=[value,cur_space[index][1]];
                        
                        match split[1]{
                            "A"=>possibility+=calc_possibilites(new_space),
                            "R"=>(),
                            _=>possibility+= possibility_step(map, new_space, String::from(split[1]))
                        }
                    }
                },
                _=>println!("Invalid comparison"),
            }
        }
        else{
            let next = c.as_str();
            match next{
                "A"=>possibility+=calc_possibilites(cur_space),
                "R"=>(),
                _=>possibility+= possibility_step(map, cur_space, String::from(next))
            }
        }
    }
    return possibility;
}

fn calc_possibilites(input:[[i32;2];4])->i64{
    let output = (input[0][1]-input[0][0]+1)as i64*(input[1][1]-input[1][0]+1)as i64*(input[2][1]-input[2][0]+1)as i64*(input[3][1]-input[3][0]+1)as i64;
    return output;
}

fn parse_part(map:&HashMap<String,Vec<String>>,part:[i32;4],step:String)->bool{
    let command = map.get(&step).unwrap();
    for c in command{
        if c.contains(":"){
            let index = c[0..1].parse::<usize>().expect("failed to parse usize");
            let split:Vec<&str>= c.split(":").collect();
            let value = split[0][2..].parse::<i32>().expect("failed to parse i32");
            match &c[1..2]{
                ">"=>{
                    if part[index]>value{
                        match split[1]{
                            "A"=>return true,
                            "R"=>return false,
                            _=>return parse_part(map, part, String::from(split[1]))
                        }
                    }
                },
                "<"=>{
                    if part[index]<value{
                        match split[1]{
                            "A"=>return true,
                            "R"=>return false,
                            _=>return parse_part(map, part, String::from(split[1]))
                        }
                    }
                },
                _=>println!("Invalid comparison"),
            }
        }   
        else{
            let next = c.as_str();
            match next{
                "A"=>return true,
                "R"=>return false,
                _=>return parse_part(map, part, String::from(next))
            }
        }
    }
    println!("Command Failed at {}",step);
    return false;
}

fn parse_document()->(HashMap<String,Vec<String>>,Vec<[i32;4]>){
    let mut part_list:Vec<[i32;4]>=Vec::new();
    let mut map:HashMap<String,Vec<String>>=HashMap::new();
    //let input_doc = File::open("./src/inputs//test/Day19test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day19input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut part_mode:bool=false;
    for line in reader.lines(){
        let line_str:String = line.expect("Could not parse line");
        if line_str.len()==0{part_mode=!part_mode;}
        else{
            if part_mode{
                let mut new_part:[i32;4]=[0,0,0,0];
                let cut_str = &line_str[1..line_str.len()-1];
                let split1:Vec<&str>=cut_str.split(",").collect();
                for s in 0..split1.len(){
                    let value = &split1[s][2..];
                    new_part[s]= value.parse().expect("failed to parse i32");
                }
                part_list.push(new_part);
            }
            else{
                let cut_str = &line_str[0..line_str.len()-1];
                let split1:Vec<&str>=cut_str.split("{").collect();
                let new_key:String = String::from(split1[0]);
                let mut new_value:Vec<String>=Vec::new();
                let split2:Vec<&str>=split1[1].split(",").collect();
                for s in split2{
                    if s.contains(":"){
                        let mut new_i ="";
                        let end=&s[1..];
                        match &s[0..1]{
                            "x"=>new_i="0",
                            "m"=>new_i="1",
                            "a"=>new_i="2",
                            "s"=>new_i="3",
                            _=>println!("Failed to parse xmas")
                        }
                        let mut command=String::from(new_i);
                        command.push_str(end);
                        new_value.push(command);
                    }
                    else{
                        new_value.push(String::from(s));
                    }
                }

                map.insert(new_key, new_value);
            }
        }
    }

    return (map,part_list);
}