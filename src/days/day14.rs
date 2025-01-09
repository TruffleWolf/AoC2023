use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn part1(){
    println!("Day14 Part1");

    let mut rocks:Vec<Vec<char>> = parse_document();


    for i in 0..rocks[0].len(){
        let mut new_pos:usize = 9999999;
        let mut current_pos:usize = 0;
        while current_pos < rocks.len(){
            match rocks[current_pos][i]{
                'O'=>{
                    if new_pos < 9000000{
                        rocks[current_pos][i]= '.';
                        rocks[new_pos][i]='O';
                        current_pos = new_pos;
                        new_pos = 9999999;
                    }
                },
                '.'=>{
                    if new_pos >9000000{new_pos=current_pos}
                },
                '#'=>{
                    new_pos = 9999999;
                },
                _=>{println!("INVALID CHAR DETECTED");}
            }
            current_pos+=1;
        }
    }

    let mut total:u32 = 0;
    for r in 0..rocks.len(){
        let mut addition:u32=0;

        for c in &rocks[r]{
            if c ==&'O'{addition+=1;}
        }
        total+=(rocks.len()-r)as u32*addition;
    }
    println!("Total:{total}");
}

pub fn part2(){
    println!("Day14 Part2");

    let mut rocks:Vec<Vec<char>> = parse_document();
    let mut found:Vec<Vec<Vec<char>>> =Vec::new(); 
    let mut searching:bool = true;
    let mut step:u64 = 0;
    while searching{
        rocks = cycle_grid(rocks);
        
        if found.contains(&rocks){searching =false;}
        else{
            found.push(rocks.clone());
            step+=1;
            if step==1000000000{break;}
        }
    }
    let target  = match found.iter().position(|x| x==&rocks){
        Some(a)=>a,
        None=>{println!("Failed to find value");0},
    };
    println!("Duplicate found at {target} on step:{step}");
    let new_vec = &found[target..];
    let target_index = (1000000000-target-1)%(new_vec.len());
    
    rocks = new_vec[target_index].clone();

    // for r in &rocks{
    //     println!("{:?}",r);
    // }

    let mut total:u32 = 0;
    for r in 0..rocks.len(){
        let mut addition:u32=0;

        for c in &rocks[r]{
            if c ==&'O'{addition+=1;}
        }
        total+=(rocks.len()-r)as u32*addition;
    }
    println!("Total:{total}");
}

fn cycle_grid(input:Vec<Vec<char>>)->Vec<Vec<char>>{
    let mut rocks = input;
    let x_size = rocks[0].len();
    let y_size = rocks.len();
    //north
    for i in 0..x_size{
        let mut new_pos:usize = 9999999;
        let mut current_pos:usize = 0;
        while current_pos < y_size{
            match rocks[current_pos][i]{
                'O'=>{
                    if new_pos < 9000000{
                        rocks[current_pos][i]= '.';
                        rocks[new_pos][i]='O';
                        current_pos = new_pos;
                        new_pos = 9999999;
                    }
                },
                '.'=>{
                    if new_pos >9000000{new_pos=current_pos}
                },
                '#'=>{
                    new_pos = 9999999;
                },
                _=>{println!("INVALID CHAR DETECTED");}
            }
            current_pos+=1;
        }
    }

    //west
    for i in 0..y_size{
        let mut new_pos:usize = 9999999;
        let mut current_pos:usize = 0;
        while current_pos < x_size{
            match rocks[i][current_pos]{
                'O'=>{
                    if new_pos < 9000000{
                        rocks[i][current_pos]= '.';
                        rocks[i][new_pos]='O';
                        current_pos = new_pos;
                        new_pos = 9999999;
                    }
                },
                '.'=>{
                    if new_pos >9000000{new_pos=current_pos}
                },
                '#'=>{
                    new_pos = 9999999;
                },
                _=>{println!("INVALID CHAR DETECTED");}
            }
            current_pos+=1;
        }
    }
    

    //south
    for i in 0..x_size{
        let mut new_pos:usize = 9999999;
        let mut current_pos:usize = 1;
        while current_pos < y_size+1{
            match rocks[y_size-current_pos][i]{
                'O'=>{
                    if new_pos < 9000000{
                        rocks[y_size-current_pos][i]= '.';
                        rocks[y_size-new_pos][i]='O';
                        current_pos = new_pos;
                        new_pos = 9999999;
                    }
                },
                '.'=>{
                    if new_pos >9000000{new_pos=current_pos}
                },
                '#'=>{
                    new_pos = 9999999;
                },
                _=>{println!("INVALID CHAR DETECTED");}
            }
            current_pos+=1;
        }
    }

    //east
    for i in 0..y_size{
        let mut new_pos:usize = 9999999;
        let mut current_pos:usize = 1;
        while current_pos < x_size+1{
            match rocks[i][x_size-current_pos]{
                'O'=>{
                    if new_pos < 9000000{
                        rocks[i][x_size-current_pos]= '.';
                        rocks[i][x_size-new_pos]='O';
                        current_pos = new_pos;
                        new_pos = 9999999;
                    }
                },
                '.'=>{
                    if new_pos >9000000{new_pos=current_pos}
                },
                '#'=>{
                    new_pos = 9999999;
                },
                _=>{println!("INVALID CHAR DETECTED");}
            }
            current_pos+=1;
        }
    }

    return rocks;
}

fn parse_document()->Vec<Vec<char>>{
    let mut output:Vec<Vec<char>> = Vec::new();
    //let input_doc = File::open("./src/inputs//test/Day14test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day14input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);

    for line in reader.lines(){
        let line_str = line.expect("Could not parse line");
        let new_line:Vec<char> = line_str.chars().collect();
        output.push(new_line);
    }

    return output
}