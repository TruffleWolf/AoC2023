use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn part1(){
    println!("Day25 Part1");
    let mut connections = parse_document();
    let mut list:Vec<String> = Vec::new();

    for c in &connections{
        if !list.contains(&c[1]){
            list.push(c[1].clone());
        }
        if !list.contains(&c[0]){
            list.push(c[0].clone())
        }
    }
    println!("Count:{}",list.len());

    let mut group_1:HashSet<String>=HashSet::with_capacity(3);
    let mut group_2:HashSet<String>=HashSet::with_capacity(3);
    group_1.insert(String::from("mgb"));
    group_1.insert(String::from("qns"));
    group_1.insert(String::from("tjd"));
    group_2.insert(String::from("plt"));
    group_2.insert(String::from("jxm"));
    group_2.insert(String::from("dbt"));

    while connections.len()>0{
        let mut new_connections:Vec<[String;2]>=Vec::new();

        for c in &connections{
            if group_1.contains(&c[0]){
                group_1.insert(c[1].clone());
            }
            else if group_1.contains(&c[1]){
                group_1.insert(c[0].clone());
            }
            else if group_2.contains(&c[0]){
                group_2.insert(c[1].clone());
            }
            else if group_2.contains(&c[1]){
                group_2.insert(c[0].clone());
            }
            else{
                new_connections.push(c.clone());
            }

        }


        connections = new_connections;
    }


    let total = group_1.len()*group_2.len();
    println!("Total:{} from {}x{}",total,group_1.len(),group_2.len());
}

pub fn part2(){
    println!("Day25 Part2");
}

fn parse_document()->Vec<[String;2]>{
    let mut output:Vec<[String;2]>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day25test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day25input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let split1:Vec<&str>=line_str.split(": ").collect();
        let split2:Vec<&str>=split1[1].split(" ").collect();
        // use this to find the three splits by feeding the result into graphviz
        
        // for s in split2{
        //     println!("{} -> {}",split1[0],s);
        // }
        // continue;

        let connect_1 = [String::from("mgb"),String::from("plt")];
        let connect_2 = [String::from("qns"),String::from("jxm")];
        let connect_3 = [String::from("tjd"),String::from("dbt")];
        let connect_4 = [String::from("plt"),String::from("mgb")];
        let connect_5 = [String::from("jxm"),String::from("qns")];
        let connect_6 = [String::from("dbt"),String::from("tjd")];
        let invalid_connections:Vec<[String;2]>=vec![connect_1,connect_2,connect_3,connect_4,connect_5,connect_6];
        for s in split2{
            let new_connection =[String::from(split1[0]),String::from(s)];
            if !invalid_connections.contains(&new_connection){output.push(new_connection);}
        }
    }
    return output;
}