use std::cmp;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;
use std::collections::VecDeque;

pub fn part1(){
    println!("Day5 Part1");

    let (seed_list,soil_map,fert_map,water_map,light_map,temp_map,hum_map,loc_map) = parse_document();
    let mut parse_list:Vec<i64> = Vec::with_capacity(seed_list.len());
    let maps:Vec<Vec<[i64;3]>>= vec![soil_map,fert_map,water_map,light_map,temp_map,hum_map,loc_map];
    for seed in &seed_list{
        let mut numb:i64 = *seed;

        for map in &maps{
            for list in map{
                //println!("{}/{}/{}",list[0],list[1],list[2]);
                if numb >=list[1] && numb < list[1]+list[2]{
                    numb = list[0]+(numb-list[1]);
                    break;
                    //println!("{numb}");

                }
            }
        }
        
        parse_list.push(numb);
    }

    let mut lowest:&i64 = &parse_list[0];
    for s in &parse_list{
        //println!("{s}");
        lowest = cmp::min(s,lowest);
    }
    println!("Lowest:{lowest}");
}

pub fn part2(){
    println!("Day5 Part2");
    let mut lowest:i64 = i64::MAX;
    let (seed_list,soil_map,fert_map,water_map,light_map,temp_map,hum_map,loc_map) = parse_document();
    //let mut parse_list:Vec<i64> = Vec::with_capacity(seed_list.len());
    let mut seed_map:VecDeque<[i64;3]> = VecDeque::new();
    let maps:Vec<Vec<[i64;3]>>= vec![soil_map,fert_map,water_map,light_map,temp_map,hum_map,loc_map];

    for s in 0..seed_list.len()-1{
        if s%2==0{
            //println!("{}/{}",seed_list[s],seed_list[s+1]);
            let seed_range:[i64;3]=[seed_list[s],seed_list[s]+seed_list[s+1],0];
            seed_map.push_back(seed_range);
        }
    }
    
    while seed_map.len()>0{
        let mut s = seed_map[0];
        let mut stage:i64 = 0;
        for map in &maps{
            if stage < s[2]{stage+=1;continue;}
            for list in map{
                if s[0]>=list[1]+list[2] || s[1]<=list[1]{
                    continue;
                }
                if s[0]<list[1]{
                    let x:[i64;3]=[s[0],list[1],stage];
                    seed_map.push_back(x);
                    s[0]=list[1];
                }
                if s[1]>list[1]+list[2]{
                    let y:[i64;3]=[list[1]+list[2],s[1],stage];
                    seed_map.push_back(y);
                    s[1]= list[1]+list[2];
                }
                let displace:i64= list[0]-list[1];
                s = [s[0]+displace,s[1]+displace,s[2]];
                break;
            }
            stage +=1;
            //println!("stage:{}/{}",stage,s[0]);
        }
        //println!("seed:{}",s[0]);
        lowest = cmp::min::<i64>(s[0],lowest);
        seed_map.pop_front();
        
    }
    
    println!("Lowest:{lowest}");
}

fn parse_document()->(Vec<i64>,Vec<[i64;3]>,Vec<[i64;3]>,Vec<[i64;3]>,Vec<[i64;3]>,Vec<[i64;3]>,Vec<[i64;3]>,Vec<[i64;3]>){
    let mut seed_list:Vec<i64> = Vec::new();
    let mut soil_map:Vec<[i64;3]> = Vec::new();
    let mut fert_map:Vec<[i64;3]> = Vec::new();
    let mut water_map:Vec<[i64;3]> = Vec::new();
    let mut light_map:Vec<[i64;3]> = Vec::new();
    let mut temp_map:Vec<[i64;3]> = Vec::new();
    let mut hum_map:Vec<[i64;3]> = Vec::new();
    let mut loc_map:Vec<[i64;3]> = Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day5test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day5input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut mode:u16 = 0;
    for line in reader.lines(){
        let row:String = line.expect("Could not read line");
        if row.len()==0{mode+=1;}
        else{
            match mode{
                0=>{
                    //seeds
                    let split:Vec<&str> = row.split(" ").collect();
                    for s in split{
                        if s.parse::<i64>().is_ok(){seed_list.push(FromStr::from_str(s).unwrap())}
                    }
                },
                1=>{
                    //soil
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        soil_map.push(array);
                    }
                    
                },
                2=>{
                    //fertilizer
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        fert_map.push(array);
                    }
                },
                3=>{
                    //water
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        water_map.push(array);
                    }
                },
                4=>{
                    //light
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        light_map.push(array);
                    }
                },
                5=>{
                    //temp
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        temp_map.push(array);
                    }
                }
                6=>{
                    //humidity
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        hum_map.push(array);
                    }
                }
                _=>{
                    //loc
                    let split:Vec<&str> = row.split(" ").collect();
                    let mut new_vec:Vec<i64> = Vec::new();
                    for s in split{
                        if s.parse::<i64>().is_ok(){new_vec.push(FromStr::from_str(s).unwrap())}
                    }
                    if new_vec.len()==3{
                        let array:[i64;3]=[new_vec[0],new_vec[1],new_vec[2]];
                        loc_map.push(array);
                    }
                }
            }
        }
    }
    return (seed_list,soil_map,fert_map,water_map,light_map,temp_map,hum_map,loc_map);
}