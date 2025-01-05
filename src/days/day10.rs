use std::fs::File;
use std::io::{BufRead,BufReader};
use crate::vec2;

pub fn part1(){
    println!("Day10 Part1");
    let (start_pos,grid,_relevance)=parse_document();
    let mut total:u32=1;
    let mut current_pos:[i16;2]=[-1,-1];
    let mut next_pos:[i16;2]=[-1,-1];
    for d in vec2::cardinals(){
        let step = grid[(start_pos[1]+d[1])as usize][(start_pos[0]+d[0])as usize];
        if step[0]==start_pos{
            current_pos = [(start_pos[0]+d[0]),(start_pos[1]+d[1])];
            next_pos = step[1];
            break;
        }
        else if step[1]==start_pos{
            current_pos = [(start_pos[0]+d[0]),(start_pos[1]+d[1])];
            next_pos = step[0];
            break;
        }
    }

    while next_pos!=start_pos{
        let step = grid[next_pos[1]as usize][next_pos[0]as usize];
        if step[0]==current_pos{
            current_pos=next_pos;
            next_pos=step[1];
        }
        else if step[1]==current_pos{
            current_pos=next_pos;
            next_pos=step[0];
        }
        else{println!("LOOP BROKEN");break;}
        total+=1
    }
    if total%2==1{total=(total/2)+1;}
    else{total=total/2;}
    println!("Total:{total}");
}


// My intention here was to induce a stroke unto myself by writing unbelieveably bad code
pub fn part2(){
    println!("Day10 Part2");
    let (start_pos,grid, relevance)=parse_document();
    
    let mut current_pos:[i16;2]=[-1,-1];
    let mut next_pos:[i16;2]=[-1,-1];
    let mut path:Vec<[i16;2]>=Vec::new();

    for d in vec2::cardinals(){
        let step = grid[(start_pos[1]+d[1])as usize][(start_pos[0]+d[0])as usize];
        if step[0]==start_pos{
            current_pos = [(start_pos[0]+d[0]),(start_pos[1]+d[1])];
            next_pos = step[1];
            
            break;
        }
        else if step[1]==start_pos{
            current_pos = [(start_pos[0]+d[0]),(start_pos[1]+d[1])];
            next_pos = step[0];
            
            break;
        }
    }
    path.push(current_pos);
    path.push(next_pos);
    while next_pos!=start_pos{
        
        let step = grid[next_pos[1]as usize][next_pos[0]as usize];

        if step[0]==current_pos{
            current_pos=next_pos;
            next_pos=step[1];
        }
        else if step[1]==current_pos{
            current_pos=next_pos;
            next_pos=step[0];
        }
        else{println!("LOOP BROKEN");break;}
        path.push(next_pos);
    }
    let mut a_list:Vec<[i16;2]>=Vec::new();
    let mut b_list:Vec<[i16;2]>=Vec::new();
    let mut dir = [path[0][0]-path[path.len()-1][0],path[0][1]-path[path.len()-1][1]];
    for p in 0..path.len()-1{
        let current_step = path[p];
        let next_step = path[p+1];
        //let dir = [next_step[0]-current_step[1],next_step[1]-current_step[1]];
        match relevance[current_step[1]as usize][current_step[0]as usize]{
           '|'=>{
                if dir == [0,1]{
                    //from above
                    let a_side:[i16;2] = [current_step[0]-1,current_step[1]];
                    let b_side:[i16;2] = [current_step[0]+1,current_step[1]];
                    if !path.contains(&a_side){
                        a_list = parse_island('A', [a_side[0],a_side[1]], &relevance,&path,a_list);
                    }
                    if !path.contains(&b_side){
                        b_list=parse_island('B', [b_side[0],b_side[1]], &relevance,&path,b_list);
                    }
                }
                else if dir ==[0,-1]{
                    //from below
                    let a_side:[i16;2] = [current_step[0]+1,current_step[1]];
                    let b_side:[i16;2] = [current_step[0]-1,current_step[1]];
                    if !path.contains(&a_side){
                        a_list=parse_island('A', [a_side[0],a_side[1]], &relevance,&path,a_list);
                    }
                    if !path.contains(&b_side){
                        b_list=parse_island('B', [b_side[0],b_side[1]], &relevance,&path,b_list);
                    }
                }
                else{println!("Failed to parse |");}
           },
           '-'=>{
                if dir == [1,0]{
                    //from left
                    let a_side:[i16;2] = [current_step[0],current_step[1]+1];
                    let b_side:[i16;2] = [current_step[0],current_step[1]-1];
                    if !path.contains(&a_side){
                        a_list=parse_island('A', [a_side[0],a_side[1]], &relevance,&path,a_list);
                    }
                    if !path.contains(&b_side){
                        b_list=parse_island('B', [b_side[0],b_side[1]], &relevance,&path,b_list);
                    }
                }
                else if dir == [-1,0]{
                    //from right
                    let a_side:[i16;2] = [current_step[0],current_step[1]-1];
                    let b_side:[i16;2] = [current_step[0],current_step[1]+1];
                    if !path.contains(&a_side){
                        a_list=parse_island('A', [a_side[0],a_side[1]], &relevance,&path,a_list);
                    }
                    if !path.contains(&b_side){
                        b_list=parse_island('B', [b_side[0],b_side[1]], &relevance,&path,b_list);
                    }
                }
                else{println!("Failed to parse -");}
           },
           'L'=>{
                if dir == [0,1]{
                    //from above
                    let a_1:[i16;2] = [current_step[0],current_step[1]+1];
                    let a_2:[i16;2] = [current_step[0]-1,current_step[1]];
                    if !path.contains(&a_1){
                        a_list=parse_island('A', [a_1[0],a_1[1]],&relevance,&path,a_list);
                    }
                    if !path.contains(&a_2){
                        a_list=parse_island('A', [a_2[0],a_2[1]],&relevance,&path,a_list);
                    }
                }
                else if dir==[-1,0]{
                    //from right
                    let b_1:[i16;2] = [current_step[0],current_step[1]+1];
                    let b_2:[i16;2] = [current_step[0]-1,current_step[1]];
                    if !path.contains(&b_1){
                        b_list=parse_island('B', [b_1[0],b_1[1]],&relevance,&path,b_list);
                    }
                    if !path.contains(&b_2){
                        b_list=parse_island('B', [b_2[0],b_2[1]],&relevance,&path,b_list);
                    }
                }
                else{println!("Failed to parse L");}
           },
           'J'=>{
                if dir == [0,1]{
                    //from above
                    let b_1:[i16;2] = [current_step[0],current_step[1]+1];
                    let b_2:[i16;2] = [current_step[0]+1,current_step[1]];
                    if !path.contains(&b_1){
                        b_list=parse_island('B', [b_1[0],b_1[1]],&relevance,&path,b_list);
                    }
                    if !path.contains(&b_2){
                        b_list=parse_island('B', [b_2[0],b_2[1]],&relevance,&path,b_list);
                    }
                }
                else if dir ==[1,0]{
                    //from left
                    let a_1:[i16;2] = [current_step[0],current_step[1]+1];
                    let a_2:[i16;2] = [current_step[0]+1,current_step[1]];
                    if !path.contains(&a_1){
                        a_list=parse_island('A', [a_1[0],a_1[1]],&relevance,&path,a_list);
                    }
                    if !path.contains(&a_2){
                        a_list=parse_island('A', [a_2[0],a_2[1]],&relevance,&path,a_list);
                    }
                }
                else{println!("Failed to parse J");}
           },
           '7'=>{
                if dir == [0,-1]{
                    //from below
                    let a_1:[i16;2] = [current_step[0],current_step[1]-1];
                    let a_2:[i16;2] = [current_step[0]+1,current_step[1]];
                    if !path.contains(&a_1){
                        a_list=parse_island('A', [a_1[0],a_1[1]],&relevance,&path,a_list);
                    }
                    if !path.contains(&a_2){
                        a_list=parse_island('A', [a_2[0],a_2[1]],&relevance,&path,a_list);
                    }
                }
                else if dir ==[1,0]{
                    //from left
                    let b_1:[i16;2] = [current_step[0],current_step[1]-1];
                    let b_2:[i16;2] = [current_step[0]+1,current_step[1]];
                    if !path.contains(&b_1){
                        b_list=parse_island('B', [b_1[0],b_1[1]],&relevance,&path,b_list);
                    }
                    if !path.contains(&b_2){
                        b_list=parse_island('B', [b_2[0] ,b_2[1]],&relevance,&path,b_list);
                    }
                }
                else{println!("Failed to parse 7");}
           },
           'F'=>{
                if dir == [0,-1]{
                    //from below
                    let b_1:[i16;2] = [current_step[0],current_step[1]-1];
                    let b_2:[i16;2] = [current_step[0]-1,current_step[1]];
                    if !path.contains(&b_1){
                        b_list=parse_island('B', [b_1[0] ,b_1[1] ],&relevance,&path,b_list);
                    }
                    if !path.contains(&b_2){
                        b_list=parse_island('B', [b_2[0] ,b_2[1] ],&relevance,&path,b_list);
                    }
                }
                else if dir == [-1,0]{
                    //from right
                    let a_1:[i16;2] = [current_step[0],current_step[1]-1];
                    let a_2:[i16;2] = [current_step[0]-1,current_step[1]];
                    if !path.contains(&a_1){
                        a_list=parse_island('A', [a_1[0] ,a_1[1] ],&relevance,&path,a_list);
                    }
                    if !path.contains(&a_2){
                        a_list=parse_island('A', [a_2[0] ,a_2[1] ],&relevance,&path,a_list);
                    }
                }
                else{println!("Failed to pares F");}
           },
           _=>println!("Invalid char found during total"), 
        }
        println!("{}",a_list.len());
        println!("{}",b_list.len());
        dir = [next_step[0]-current_step[0],next_step[1]-current_step[1]];
    }
    println!("TEST");
    // for r in &relevance{
    //     println!("{:?}",r);
    // }
    let mut total_a:Vec<[i16;2]>=Vec::new();
    let mut total_b:Vec<[i16;2]>=Vec::new();
    
    // for r in relevance{
    //     for char in r{
    //         if char =='A'{total_a+=1;}
    //         if char =='B'{total_b+=1;}
            
    //     }
    // }
    for a in a_list{
        if !total_a.contains(&a){total_a.push(a);}
    }
    for b in b_list{
        if !total_b.contains(&b){total_b.push(b);}
    }

    println!("Total:A{}/B{}",total_a.len(),total_b.len());
}

// This code will stack overfow without the two || checks,
// however before overflowing it will tell you which "side" of the loop is the "outside" so you can filter it out 
fn parse_island(c:char,pos:[i16;2],grid:&Vec<Vec<char>>,path:&Vec<[i16;2]>,relevance:Vec<[i16;2]>)->Vec<[i16;2]>{
    let mut known_vec:Vec<[i16;2]> = relevance;
    if !vec2::in_bounds(&[pos[0],pos[1]], &grid)||c=='B'{
        println!("{} faltered_base",c);
        return known_vec;
    }
    known_vec.push(pos);
    for dir in vec2::cardinals(){
        if !vec2::in_bounds(&[pos[0]+dir[0],pos[1]+dir[1]], &grid)||c=='B'{
            println!("{} faltered_dir",c);
            return known_vec;
        }
        let next_pos = [pos[0]+dir[0],pos[1]+dir[1]];
        if !known_vec.contains(&next_pos) && !path.contains(&next_pos){
            println!("{:?}",next_pos);
            
            known_vec= parse_island(c,next_pos, grid,path,known_vec);
        }
    }
    return known_vec;

}

fn parse_document()->([i16;2],Vec<Vec<[[i16;2];2]>>,Vec<Vec<char>>){
    let mut start_pos:[i16;2]=[0,0];
    let mut total_grid:Vec<Vec<[[i16;2];2]>>=Vec::new();
    let mut relevance_grid:Vec<Vec<char>>=Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day10test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day10input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut y:i16=0;
    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let mut grid_row:Vec<[[i16;2];2]> = Vec::new();
        let mut rev_row:Vec<char>=Vec::new();
        let mut x:i16=0;
        for c in line_str.chars(){
            match c{
                'S'=>{start_pos=[x,y];grid_row.push([[-1,-1],[-1,-1]])}
                '|'=>grid_row.push([[x,y-1],[x,y+1]]),
                '-'=>grid_row.push([[x-1,y],[x+1,y]]),
                'L'=>grid_row.push([[x,y-1],[x+1,y]]),
                'J'=>grid_row.push([[x,y-1],[x-1,y]]),
                '7'=>grid_row.push([[x-1,y],[x,y+1]]),
                'F'=>grid_row.push([[x,y+1],[x+1,y]]),
                '.'=>grid_row.push([[-2,-2],[-2,-2]]),
                _=>println!("Invalid char found{}",c),
            }
            rev_row.push(c);
            x+=1;
        }
        relevance_grid.push(rev_row);
        total_grid.push(grid_row);
        y+=1;
    }

    return (start_pos,total_grid,relevance_grid);
}