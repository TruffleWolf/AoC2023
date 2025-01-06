use std::fs::File;
use std::io::{BufRead,BufReader};
use std::cmp;

pub fn part1(){
    println!("Day11 Part1");

    let mut base_grid=parse_document();

    let mut rot_grid:Vec<Vec<char>> = Vec::with_capacity(base_grid[0].len());
    let mut blank_vec:Vec<char>=Vec::new();
    for _b in &base_grid[0]{
        let row:Vec<char>=Vec::with_capacity(base_grid.len());
        rot_grid.push(row);
        blank_vec.push('.');
    }
    
    for b in &base_grid{
        for i in 0..b.len(){
            rot_grid[i].push(b[i]);
        }
    }
    // for r in &rot_grid{
    //     println!("{:?}",r);
    // }
    //additional rows
    let mut count:usize =0;
    while count < base_grid.len(){
        if !base_grid[count].contains(&'#'){
            base_grid.insert(count,blank_vec.clone());
            count+=1;
        }
        count+=1;
    }
    //additional columns
    for y in 0..base_grid.len(){
        
        for x in 1..rot_grid.len()+1{
            
            if !rot_grid[rot_grid.len()-x].contains(&'#'){
                base_grid[y].insert(rot_grid.len()-x, '.');
                
            }
        }
    }

    let mut g_locations:Vec<[i16;2]>=Vec::new();
    for y in 0..base_grid.len(){
        //println!("{:?}",base_grid[y]);
        for x in 0..base_grid[y].len(){
            if base_grid[y][x]=='#'{g_locations.push([x as i16,y as i16])}
        }
    }
    let mut total:i32=0;
    
    for i in 0..g_locations.len()-1{
        let pos:[i16;2]= g_locations[i];
        for x in i..g_locations.len(){
            let tar:[i16;2]=g_locations[x];
            total+=(pos[0]-tar[0]).abs()as i32+(pos[1]-tar[1]).abs()as i32;
        }
    }
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day11 Part2");
    let base_grid=parse_document();
    let mut blank_rows:Vec<usize>=Vec::new();
    let mut blank_col:Vec<usize>=Vec::new();

    let mut rot_grid:Vec<Vec<char>> = Vec::with_capacity(base_grid[0].len());
    
    for _b in &base_grid[0]{
        let row:Vec<char>=Vec::with_capacity(base_grid.len());
        rot_grid.push(row);
    }

    for b in &base_grid{
        for i in 0..b.len(){
            rot_grid[i].push(b[i]);
        }
    }

    for r in 0..base_grid.len(){
        if !base_grid[r].contains(&'#'){blank_rows.push(r);}
    }
    for c in 0..rot_grid.len(){
        if !rot_grid[c].contains(&'#'){blank_col.push(c);}
    }

    let mut g_locations:Vec<[i64;2]>=Vec::new();
    for y in 0..base_grid.len(){
        for x in 0..base_grid[y].len(){
            if base_grid[y][x]=='#'{g_locations.push([x as i64,y as i64])}
        }
    }
    //println!("{:?}",blank_col);
    //println!("{:?}",blank_rows);
    let mut total:i64=0;
    let distance:i64=1000000;
    for i in 0..g_locations.len()-1{
        let pos:[i64;2]= g_locations[i];
        for x in i..g_locations.len(){
            let tar:[i64;2]=g_locations[x];
            let mut distance_x:i64 = (pos[0]-tar[0]).abs();
            let mut distance_y:i64 = (pos[1]-tar[1]).abs();
            let x_range = cmp::min(pos[0],tar[0])..cmp::max(pos[0],tar[0]);
            let y_range = cmp::min(pos[1],tar[1])..cmp::max(pos[1],tar[1]);
            for r in &blank_rows{
                let row = *r as i64;
                if y_range.contains(&row){distance_y+=distance-1;}
            }
            for c in &blank_col{
                let col = *c as i64;
                if x_range.contains(&col){distance_x+=distance-1;}
            }
            //println!("{}/{}",distance_x,distance_y);
            total+=distance_x+distance_y;
        }
    }
    println!("Total:{total}");
}

fn parse_document()->Vec<Vec<char>>{
    let mut base_grid:Vec<Vec<char>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day11test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day11input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let row:Vec<char> = match line {
            Ok(s)=>s.chars().collect(),
            Err(_)=>{println!("Failed to read row");break}
        };
        base_grid.push(row);
    }

    return base_grid;
}