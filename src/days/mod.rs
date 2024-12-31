
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn call_daypart(day:i32,part:i32){
    match day{
        1 => {
            if part == 1{
                day1::part1();
            }
            else{
                day1::part2();
            }
        },
        2 => {
            if part == 1{
                day2::part1();
            }
            else{
                day2::part2();
            }
        },
        3 => {
            if part == 1{
                day3::part1();
            }
            else{
                day3::part2();
            }
        },
        4 =>{
            if part == 1{
                day4::part1();
            }
            else{
                day4::part2();
            }
        },
        5 =>{
            if part == 1{
                day5::part1();
            }
            else{
                day5::part2();
            }
        },
        6 =>{
            if part == 1{
                day6::part1();
            }
            else{
                day6::part2();
            }
        },
        7 =>{
            if part == 1{
                day7::part1();
            }
            else{
                day7::part2();
            }
        },
        8 =>{
            if part == 1{
                day8::part1();
            }
            else{
                day8::part2();
            }
        },
        9 =>{
            if part == 1{
                day9::part1();
            }
            else{
                day9::part2();
            }
        },
        10 =>{
            if part == 1{
                day10::part1();
            }
            else{
                day10::part2();
            }
        },
        11 =>{
            if part == 1{
                day11::part1();
            }
            else{
                day11::part2();
            }
        },
        12 =>{
            if part == 1{
                day12::part1();
            }
            else{
                day12::part2();
            }
        },
        13 =>{
            if part == 1{
                day13::part1();
            }
            else{
                day13::part2();
            }
        },
        14 =>{
            if part == 1{
                day14::part1();
            }
            else{
                day14::part2();
            }
        },
        15 =>{
            if part == 1{
                day15::part1();
            }
            else{
                day15::part2();
            }
        },
        16 =>{
            if part == 1{
                day16::part1();
            }
            else{
                day16::part2();
            }
        },
        17 =>{
            if part == 1{
                day17::part1();
            }
            else{
                day17::part2();
            }
        },
        18 =>{
            if part == 1{
                day18::part1();
            }
            else{
                day18::part2();
            }
        },
        19 =>{
            if part == 1{
                day19::part1();
            }
            else{
                day19::part2();
            }
        },
        20 =>{
            if part == 1{
                day20::part1();
            }
            else{
                day20::part2();
            }
        },
        21 =>{
            if part == 1{
                day21::part1();
            }
            else{
                day21::part2();
            }
        },
        22 =>{
            if part == 1{
                day22::part1();
            }
            else{
                day22::part2();
            }
        },
        23 =>{
            if part == 1{
                day23::part1();
            }
            else{
                day23::part2();
            }
        },
        24 =>{
            if part == 1{
                day24::part1();
            }
            else{
                day24::part2();
            }
        },
        25 =>{
            if part == 1{
                day25::part1();
            }
            else{
                day25::part2();
            }
        },
        _ => println!("Invalid Day"),
    }
}


