use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp;

pub fn part1(){
    println!("Day7 Part1");

    let (hand_list,bid_list) = parse_document();
    let card_scores:HashMap<char,u16>= HashMap::from([
        ('2',0),('3',1),('4',2),('5',3),('6',4),('7',5),('8',6),
        ('9',7),('T',8),('J',9),('Q',10),('K',11),('A',12),
    ]);
    //Array[0]= hand list index,[1] = the hand strength
    let mut hand_positions:Vec<[i32;2]>=Vec::with_capacity(hand_list.len());
    for h in 0..hand_list.len(){
        let mut card_counts:Vec<u8>=vec![0,0,0,0,0,0,0,0,0,0,0,0,0];
        for chars in hand_list[h].chars(){
            match card_scores.get(&chars){
                Some(s)=>card_counts[*s as usize]+=1,
                None=>println!("Char {} not found",&chars)
            }
        }
        if card_counts.contains(&5){
            // five of a kind
            hand_positions.push([h as i32,6])
        }
        else if card_counts.contains(&4){
            // four of a kind
            hand_positions.push([h as i32,5])
        }
        else if card_counts.contains(&3){
            if card_counts.contains(&2){
                // full house
                hand_positions.push([h as i32,4])
            }
            else{
                // three of a kind
                hand_positions.push([h as i32,3])
            }
        }
        else if card_counts.contains(&2){
            let mut count:u8 = 0;
            for f in &card_counts{
                if f == &2{count+=1;}
            }
            if count >1{
                // two pair
                hand_positions.push([h as i32,2])
            }
            else{
                // pair
                hand_positions.push([h as i32,1])
            }
        }
        else{
            // high card
            hand_positions.push([h as i32,0])
        }
    }
    let mut final_positions:Vec<[i32;2]>= Vec::with_capacity(hand_positions.len());
    final_positions.push(hand_positions[0]);
    for h in 1..hand_positions.len(){
        let mut count:usize = 0;
        while count < final_positions.len(){

            if hand_positions[h][1]==final_positions[count][1]{
                if is_worse(hand_positions[h][0],final_positions[count][0],&hand_list,&card_scores){
                    final_positions.insert(count, hand_positions[h]);
                    break;
                }
            }
            else if hand_positions[h][1]<final_positions[count][1]{
                final_positions.insert(count, hand_positions[h]);
                break;
            }
            count +=1;
            if count == final_positions.len(){
                final_positions.push(hand_positions[h]);
                break;
            }

        }
    }

    let mut total:i32 = 0;
    let mut count:i32 = 1;
    for f in &final_positions{
        total += bid_list[f[0]as usize]*count;
        count+=1;
    }
    println!("Total:{total}");

}

pub fn part2(){
    println!("Day7 Part2");
    let (hand_list,bid_list) = parse_document();
    let card_scores:HashMap<char,u16>= HashMap::from([
        ('2',1),('3',2),('4',3),('5',4),('6',5),('7',6),('8',7),
        ('9',8),('T',9),('J',0),('Q',10),('K',11),('A',12),
    ]);
    //Array[0]= hand list index,[1] = the hand strength
    let mut hand_positions:Vec<[i32;2]>=Vec::with_capacity(hand_list.len());
    for h in 0..hand_list.len(){
        let mut card_counts:Vec<u8>=vec![0,0,0,0,0,0,0,0,0,0,0,0,0];
        for chars in hand_list[h].chars(){
            match card_scores.get(&chars){
                Some(s)=>card_counts[*s as usize]+=1,
                None=>println!("Char {} not found",&chars)
            }
        }
        // joker conversion
        let joker_count:u8 = card_counts[0];
        let mut best_size:u8 = 0;
        card_counts[0] = 0;
        //println!("{joker_count}");
        for c in &card_counts{
            best_size = cmp::max(best_size, *c);
        }
        let c_size:usize = card_counts.len();
        for c in 1..c_size{
            if card_counts[c_size-c]== best_size{
                card_counts[c_size-c]+=joker_count;
                //println!("converted to {}",c_size-c);
                break;
            }
        }

        if card_counts.contains(&5){
            // five of a kind
            hand_positions.push([h as i32,6])
        }
        else if card_counts.contains(&4){
            // four of a kind
            hand_positions.push([h as i32,5])
        }
        else if card_counts.contains(&3){
            if card_counts.contains(&2){
                // full house
                hand_positions.push([h as i32,4])
            }
            else{
                // three of a kind
                hand_positions.push([h as i32,3])
            }
        }
        else if card_counts.contains(&2){
            let mut count:u8 = 0;
            for f in &card_counts{
                if f == &2{count+=1;}
            }
            if count >1{
                // two pair
                hand_positions.push([h as i32,2])
            }
            else{
                // pair
                hand_positions.push([h as i32,1])
            }
        }
        else{
            // high card
            hand_positions.push([h as i32,0])
        }
    }
    // for c in &hand_positions{
    //     println!("Power:{}",c[1])
    // }
    let mut final_positions:Vec<[i32;2]>= Vec::with_capacity(hand_positions.len());
    final_positions.push(hand_positions[0]);
    for h in 1..hand_positions.len(){
        let mut count:usize = 0;
        while count < final_positions.len(){

            if hand_positions[h][1]==final_positions[count][1]{
                if is_worse(hand_positions[h][0],final_positions[count][0],&hand_list,&card_scores){
                    final_positions.insert(count, hand_positions[h]);
                    break;
                }
            }
            else if hand_positions[h][1]<final_positions[count][1]{
                final_positions.insert(count, hand_positions[h]);
                break;
            }
            count +=1;
            if count == final_positions.len(){
                final_positions.push(hand_positions[h]);
                break;
            }

        }
    }

    let mut total:i32 = 0;
    let mut count:i32 = 1;
    for f in &final_positions{
        total += bid_list[f[0]as usize]*count;
        count+=1;
    }
    println!("Total:{total}");
}

fn parse_document()->(Vec<String>,Vec<i32>){
    let mut hand_list:Vec<String>=Vec::new();
    let mut bid_list:Vec<i32>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day7test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day7input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let row:Vec<&str> = line_str.split(" ").collect();
        if row.len()>1{
            hand_list.push(String::from(row[0]));
            bid_list.push(FromStr::from_str(row[1]).expect("Failed to parse i32"));
        }
    }


    return (hand_list,bid_list);
}

fn is_worse(origin:i32,target:i32,list:&Vec<String>,scores:&HashMap<char,u16>)->bool{
    let hand1:Vec<char> = list[origin as usize].clone().chars().collect();
    let hand2:Vec<char> = list[target as usize].clone().chars().collect();
    
    for i in 0..hand1.len(){
        let score1 = match scores.get(&hand1[i]){
            Some(x)=>x,
            None=>{println!("Card not found");&0}
        };
        let score2 = match scores.get(&hand2[i]){
            Some(x)=>x,
            None=>{println!("Card not found");&0}
        };
        if score1<score2{return true;}
        else if score1>score2{return false;}
    }

    //println!("identical hand found");
    return false
}