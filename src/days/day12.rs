use std::fs::File;
use std::io::{BufRead,BufReader};
use cached::proc_macro::cached;

pub fn part1(){
    println!("Day12 Part1");

    let (bath_line,fault_groups)=parse_document();
    let mut total:u64 = 0;
    for i in 0..bath_line.len(){
        //println!("{:?} : {:?}",bath_line[i],fault_groups[i]);
        let addition = dive(bath_line[i].clone(),fault_groups[i].clone());
        //println!("{addition}");
        total+=addition;
    }

    println!("Total:{total}");
}

pub fn part2(){
    println!("Day12 Part2");

    let (mut bath_line,mut fault_groups)=parse_document();
    let mut total:u64 = 0;
    for i in 0..bath_line.len(){
        bath_line[i].pop();
        bath_line[i].push('?');
        let new_faults:Vec<usize> = fault_groups[i].clone();
        let new_baths:Vec<char>= bath_line[i].clone();
        
        for _x in 0..4{
            fault_groups[i].append(&mut new_faults.clone());
            bath_line[i].append(&mut new_baths.clone());
        }
        bath_line[i].pop();
        bath_line[i].push('.');
        //println!("{:?} | {:?}",bath_line[i],fault_groups[i]);

        let addition = dive(bath_line[i].clone(),fault_groups[i].clone());
        println!("{addition}");
        total+=addition;
    }

    println!("Total:{total}");
}

#[cached]
fn dive(chars:Vec<char>,faults:Vec<usize>)->u64{
    let mut output:u64 = 0;

    if faults.len()==0{
        //println!("{:?}",chars);
        return !chars.contains(&'#') as u64;
    }

    let current:usize = faults[0];
    let new_vec= &faults[1..].to_vec();

    for i in 0..(chars.len()-new_vec.iter().sum::<usize>()-new_vec.len()-current+1){
        
        let chars_check = chars[..i].to_vec();
        if chars_check.contains(&'#'){break;}
        let next = i+current;
        if next <= chars.len() && !chars[i..next].contains(&'.') && chars[next]!='#'{
            let new_chars = chars[next+1..].to_vec();
            output+= dive(new_chars,new_vec.clone());
        }
    }

    return output;
}

fn parse_document()->(Vec<Vec<char>>,Vec<Vec<usize>>){
    let mut damage_groups:Vec<Vec<usize>>=Vec::new();
    let mut bath_line:Vec<Vec<char>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day12test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day12input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let split1:Vec<&str> = line_str.split(" ").collect();
        let mut bath_row:Vec<char> = split1[0].chars().collect();
        bath_row.push('.');
        bath_line.push(bath_row);
        let row:Vec<&str> = split1[1].split(",").collect();
        let mut damage_row:Vec<usize> = Vec::new();
        for r in row{
            if r.parse::<usize>().is_ok(){
                damage_row.push(r.parse().expect("failed to parse i32"));
            }
        }
        
        damage_groups.push(damage_row);
    }
    return (bath_line,damage_groups);
}