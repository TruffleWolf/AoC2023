use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

//ilk 0 is broadcast,ilk 1 is flip flop, ilk 2 is conjunction
struct Module{
    ilk:u8,
    inputs: Vec<String>,
    outputs: Vec<String>,
    states:Vec<bool>,
}

pub fn part1(){
    println!("Day20 Part1");
    let mut commands = parse_document();

    // for (c,data)in commands{
    //     println!("{}/{:?}/{:?}/{:?}/{:?}",c,data.ilk,data.inputs,data.outputs,data.states)
    // }

    let mut high_p:u64 = 0;
    let mut low_p:u64 = 1000;
    for _i in 0..1000{
        let mut command_queue:Vec<(String,String,bool)>=vec![("broadcaster".to_string(),"button".to_string(),false)];
        let mut step:usize = 0;
        
        while step<command_queue.len(){
            let (id,sender,sig_in)=command_queue[step].clone();
            let default_mod = Module{
                ilk: 0,
                inputs:Vec::new(),
                outputs:Vec::new(),
                states:Vec::new(),
            };
            let data = commands.entry(id.clone()).or_insert(default_mod);
            match data.ilk{
                0=>{
                    let mut addition:u64=0;
                    for o in &data.outputs{
                        command_queue.push((o.clone(),id.clone(),sig_in.clone()));
                        addition+=1;
                    }
                    if sig_in{high_p+=addition;}
                    else {low_p+=addition;}
                },
                1=>{
                    if !sig_in{
                        let mut addition:u64=0;
                        if data.states[0]{
                            data.states[0]=false;
                            for o in &data.outputs{
                                command_queue.push((o.clone(),id.clone(),false));
                                addition+=1
                            }
                            low_p+=addition;
                        }
                        else{
                            data.states[0]=true;
                            for o in &data.outputs{
                                command_queue.push((o.clone(),id.clone(),true));
                                addition+=1
                            }
                            high_p+=addition;
                        }
                    }
                },
                2=>{
                    let index = data.inputs.iter().position(|x| x==&sender).unwrap();
                    data.states[index] = sig_in;
                    let mut sig_out:bool = true;
                    for d in &data.states{
                        sig_out = sig_out && *d;
                    }
                    let mut addition:u64=0;
                    for o in &data.outputs{
                        command_queue.push((o.clone(),id.clone(),!sig_out));
                        addition+=1;
                    }
                    if sig_out{
                        low_p+=addition;
                    }
                    else{
                        high_p+=addition;
                    }
                },
                _=>{println!("INVALID TYPE DETECTED")}
            }
            step+=1;
        }

    }
    
    println!("Total:{}*{}={}",high_p,low_p,high_p*low_p);
}

pub fn part2(){
    println!("Day20 Part2");
    //I manually read the input to find the values that would allow the conjoining into RX, then multiplied the first occurance of each to find when they would coincide.

    let mut commands = parse_document();

    let mut answer_vec:Vec<u64>= vec![0,0,0,0];
    let mut count:u64= 1;

    while answer_vec.contains(&0){
        let mut command_queue:Vec<(String,String,bool)>=vec![("broadcaster".to_string(),"button".to_string(),false)];
        let mut step:usize = 0;
        let mut rx_count:u32 = 0;
        while step<command_queue.len(){
            let (id,sender,sig_in)=command_queue[step].clone();
            let default_mod = Module{
                ilk: 0,
                inputs:Vec::new(),
                outputs:Vec::new(),
                states:Vec::new(),
            };
            let data = commands.entry(id.clone()).or_insert(default_mod);
            match data.ilk{
                0=>{
                    for o in &data.outputs{
                        command_queue.push((o.clone(),id.clone(),sig_in.clone()));
                    }
                },
                1=>{
                    if !sig_in{
                        if data.states[0]{
                            data.states[0]=false;
                            for o in &data.outputs{
                                command_queue.push((o.clone(),id.clone(),false));
                            }
                        }
                        else{
                            data.states[0]=true;
                            for o in &data.outputs{
                                command_queue.push((o.clone(),id.clone(),true));
                            }
                        }
                    }
                },
                2=>{
                    let index = data.inputs.iter().position(|x| x==&sender).unwrap();
                    data.states[index] = sig_in;
                    let mut sig_out:bool = true;
                    for d in &data.states{
                        sig_out = sig_out && *d;
                    }
                    for o in &data.outputs{
                        command_queue.push((o.clone(),id.clone(),!sig_out));

                    }
                    if id == "ft" && sig_out{
                        rx_count+=1;
                    }
                    else if id == "vz" && !sig_out && answer_vec[0]==0{
                        answer_vec[0]= count;
                    }
                    else if id == "bq" && !sig_out && answer_vec[1]==0{
                        answer_vec[1]= count;
                    }
                    else if id == "qh" && !sig_out && answer_vec[2]==0{
                        answer_vec[2]= count;
                    }
                    else if id == "lt" && !sig_out && answer_vec[3]==0{
                        answer_vec[3]= count;
                    }
                },
                _=>{println!("INVALID TYPE DETECTED")}
            }
            step+=1;
        }
        if rx_count>0{println!("RX:{} at {}",rx_count,count)}
        //if count%100==0{println!("{}",count);}
        //println!("{}",count);
        count+=1;
    }
    println!("Fastest Children:{:?}",answer_vec);
    let mut total = answer_vec[0];
    for i in 1..answer_vec.len(){
        total*=answer_vec[i];
    }
    println!("Total:{}",total);
}



fn parse_document()->HashMap<String,Module>{
    let mut modules:HashMap<String,Module>=HashMap::new();
    let mut output_map:HashMap<String,Vec<String>>=HashMap::new();
    //let input_doc = File::open("./src/inputs//test/Day20test.txt").expect("File not found.");
    let input_doc = File::open("./src/inputs/Day20input.txt").expect("File not found.");
    let reader = BufReader::new(input_doc);
    let mut conjoins:Vec<String>=Vec::new();
    for line in reader.lines(){
        let line_str:String = line.expect("Could not parse line");
        let split1:Vec<&str>=line_str.split(" -> ").collect();
        let mut ilk:u8 = 0;
        #[allow(unused_assignments)]
        let mut id:String = String::from("");
        let mut states:Vec<bool>=Vec::with_capacity(1);
        let inputs:Vec<String>=Vec::new();
        match &split1[0][0..1]{
            "&"=>{
                ilk = 2;
                id = String::from(&split1[0][1..]);
                conjoins.push(id.clone());
            },
            "%"=>{
                ilk = 1;
                id = String::from(&split1[0][1..]);
                states.push(false);
            },
            _=>{
                id = String::from(&split1[0][0..]);
                states.push(false);
            },
        }
        let split2:Vec<&str>=split1[1].split(", ").collect();
        let mut outputs:Vec<String>=Vec::new();
        for s in split2{
            outputs.push(String::from(s));
        }
        output_map.insert(id.clone(),outputs.clone());
        let new_mod = Module{
            ilk: ilk,
            inputs:inputs,
            outputs:outputs,
            states:states,
        };
        modules.insert(id,new_mod);

    }
    for c in conjoins{
        let mut new_inputs:Vec<String>= Vec::new();
        let mut new_states:Vec<bool>=Vec::new();
        for (id,data) in &output_map{
            if data.contains(&c){
                new_inputs.push(id.clone().to_string());
                new_states.push(false);
            }
        }
        let default_mod = Module{
            ilk: 0,
            inputs:Vec::new(),
            outputs:Vec::new(),
            states:Vec::new(),
        };
        let data = modules.entry(c).or_insert(default_mod);
        let new_mod = Module{
            ilk: data.ilk,
            inputs:new_inputs,
            outputs:data.outputs.clone(),
            states:new_states,
        };
        
        *data = new_mod;
    }

    return modules;
}