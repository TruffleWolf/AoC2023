use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

pub fn part1(){
    println!("Day6 Part1");

    let matches:Vec<[i32;2]> = parse_document();

    let mut total:i32 = 1;
    for m in &matches{
        let mut start_value:i32 = 1;
        let mut end_value:i32 =m[0];

        while start_value<m[1]{
            if start_value*(m[0]-start_value) > m[1]{break;}
            start_value+=1;
        }
        while end_value>start_value{
            if end_value*(m[0]-end_value)>m[1]{break;}
            end_value-=1;
        }
        println!("{}-{}",end_value,start_value);
        total = total*(end_value+1-start_value);
    }
    println!("Total:{}",total);
}

pub fn part2(){
    println!("Day6 Part2");

    //let input_doc = File::open("./src/inputs//test/Day6test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day6input.txt").expect("File not found.");
    let mut reader = BufReader::new(input_doc);
    let mut times:String = String::new();
    let mut records:String = String::new();
    reader.read_line(&mut times).expect("could not read line");
    reader.read_line(&mut records).expect("could not read line");
    let time_str = times.trim();
    let record_str = records.trim();
    let time_split:Vec<&str> =time_str.split(" ").collect();
    let rec_split:Vec<&str> = record_str.split(" ").collect();
    let mut total_t_str:String=String::new();
    let mut total_r_str:String=String::new();
    for t in time_split{
        if t.parse::<i64>().is_ok(){total_t_str.push_str(t);}
    }
    for r in rec_split{
        if r.parse::<i64>().is_ok(){total_r_str.push_str(r);}
    }
    let race:[i64;2]= [FromStr::from_str(&total_t_str).expect("could not conver int"),FromStr::from_str(&total_r_str).expect("could not conver int")];

    let mut start_value:i64 = 1;
    let mut end_value:i64 =race[0];

    while start_value<race[1]{
        let calced_value:i64 = match start_value.checked_mul(race[0]-start_value){
            Some(x)=>x,
            None=>break,
        };
        if calced_value > race[1]{break;}
        start_value+=1;
    }
    println!("{start_value}");
    end_value-=start_value;

    println!("Total:{}",(end_value+1-start_value));

}

fn parse_document()->Vec<[i32;2]>{
    let mut button_vec:Vec<[i32;2]>=Vec::new();

    //let input_doc = File::open("./src/inputs//test/Day6test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day6input.txt").expect("File not found.");
    let mut reader = BufReader::new(input_doc);
    let mut times:String = String::new();
    let mut records:String = String::new();
    reader.read_line(&mut times).expect("could not read line");
    reader.read_line(&mut records).expect("could not read line");
    let time_str = times.trim();
    let record_str = records.trim();
    let time_split:Vec<&str> =time_str.split(" ").collect();
    let rec_split:Vec<&str> = record_str.split(" ").collect();
    let mut time_list:Vec<i32> = Vec::new();
    let mut rec_list:Vec<i32> = Vec::new();
    for t in time_split{
        if t.parse::<i32>().is_ok(){time_list.push(FromStr::from_str(t).unwrap())}
    }
    for r in rec_split{
        if r.parse::<i32>().is_ok(){rec_list.push(FromStr::from_str(r).unwrap())}
    }
    for t in 0..time_list.len(){
        button_vec.push([time_list[t],rec_list[t]])
    }
    return button_vec;
}
