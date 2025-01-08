use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn part1(){
    println!("Day13 Part1");

    let (base_grids,rot_grids)=parse_document();
    let mut total:i64 = 0;

    for i in 0..base_grids.len(){
        let v_mirror = find_mirror(&base_grids[i],-1);
        
        if v_mirror <0{
            let h_mirror = find_mirror(&rot_grids[i],-1);
            if h_mirror <0{
                println!("Failed to find mirror");
            }
            else{total+=h_mirror}
        }
        else{total+=v_mirror*100}
    }

    println!("Total:{total}");
}

pub fn part2(){
    println!("Day13 Part2");
    let (base_grids,rot_grids)=parse_document();
    let mut total:i64 = 0;
    let mut fail_count:i64 = 0;
    for i in 0..base_grids.len(){
        let mut location:[i64;2]=[-1,-1];

        let v_mirror = find_mirror(&base_grids[i],-1);
        
        if v_mirror <0{
            let h_mirror = find_mirror(&rot_grids[i],-1);
            if h_mirror <0{
                println!("Failed to find mirror");
            }
            else{location[1]=h_mirror}
        }
        else{location[0]=v_mirror}
        // for r in &base_grids[i]{
        //     println!("{:?}",r);
        // }
        // for c in &rot_grids[i]{
        //     println!("{:?}",c);
        // }
        //println!("{:?}",location);
        'y: for y in 0..base_grids[i].len(){
            '_x: for x in 0..base_grids[i][y].len(){
                let mut new_base = base_grids[i].clone();
                let mut new_rot = rot_grids[i].clone();
                if new_base[y][x] == '.'{
                    new_base[y][x]='#';
                    new_rot[x][y]='#';
                }
                else{ 
                    new_base[y][x]='.';
                    new_rot[x][y]='.';
                }

                let v_mirror = find_mirror(&new_base,location[0]);
                if v_mirror == -1{
                    let h_mirror = find_mirror(&new_rot,location[1]);
                    if h_mirror == -1{
                        continue;
                    }
                    else if h_mirror==location[1]{continue;}
                    else{
                        total+= h_mirror;
                        //println!("H{h_mirror}");
                        break 'y;
                    }
                }
                //else if v_mirror == location[0]{continue;}
                else if v_mirror == location[0]{
                    let h_mirror = find_mirror(&new_rot,location[1]);
                    if h_mirror == -1{
                        continue;
                    }
                    else if h_mirror==location[1]{continue;}
                    else{
                        total+= h_mirror;
                        //println!("H{h_mirror}");
                        break 'y;
                    }
                }
                else{
                    total+=v_mirror*100;
                    //println!("V{v_mirror}");
                    break 'y;
                }
            }
            if y+1 == base_grids[i].len(){
                println!("Failed to find new mirror.");
                fail_count+=1;
            }
        }
        //println!("inc{i}");
    }
    println!("Failed:{fail_count}");
    println!("Total:{total}");
}

fn find_mirror(input:&Vec<Vec<char>>,invalid:i64)->i64{

    for i in 1..(input.len()/2)+1{
        let mut valid:bool = true;
        for x in 0..i{
            let side1:Vec<char> = input[i-x-1].clone();
            let side2:Vec<char> = input[i+x].clone();
            //println!("{i}/{x}:{:?} = {:?}",side1,side2);
            valid = valid && side1==side2;
        }
        if valid && i as i64 != invalid{return i as i64;}
    }

    for i in 1..(input.len()/2)+1{
        let mut valid:bool = true;
        for x in 0..i{
            let side1:Vec<char> = input[input.len()-i+x].clone();
            let side2:Vec<char> = input[input.len()-i-x-1].clone();
            //println!("{i}/{x}:{:?} = {:?}",side1,side2);
            valid = valid && side1==side2;
        }
        if valid && (input.len()-i) as i64 != invalid{return (input.len()-i) as i64;}
    }

    return -1;
}

fn parse_document()->(Vec<Vec<Vec<char>>>,Vec<Vec<Vec<char>>>){
    let mut standard_grids:Vec<Vec<Vec<char>>> = Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day13test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day13input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut base_grid:Vec<Vec<char>> = Vec::new();
    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        if line_str.len()==0{
            standard_grids.push(base_grid);
            base_grid = Vec::new();
            continue;
        }

        let new_line:Vec<char> = line_str.chars().collect();
        base_grid.push(new_line);

    }
    standard_grids.push(base_grid);

    let mut rot_grids:Vec<Vec<Vec<char>>> = Vec::with_capacity(standard_grids.len());

    for grid in &standard_grids{
        let mut new_grid:Vec<Vec<char>> = Vec::with_capacity(grid.len());
        for _g in &grid[0]{
            let new_col:Vec<char> = Vec::new();
            new_grid.push(new_col)
        }

        for row in grid{
            for i in 0..row.len(){
                new_grid[i].push(row[i]);
            }
        }
        rot_grids.push(new_grid);
    }

    return(standard_grids,rot_grids)
}