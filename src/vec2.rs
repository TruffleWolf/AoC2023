
// provides utility when working on 2d grids
// consider changing from i16 to isize

pub fn north()->[i16;2]{
    let n:[i16;2]=[0,-1];
    return n;
}

pub fn north_west()->[i16;2]{
    let n:[i16;2]=[-1,-1];
    return n;
}

pub fn west()->[i16;2]{
    let n:[i16;2]=[-1,0];
    return n;
}

pub fn south_west()->[i16;2]{
    let n:[i16;2]=[-1,1];
    return n;
}

pub fn south()->[i16;2]{
    let n:[i16;2]=[0,1];
    return n;
}


pub fn south_east()->[i16;2]{
    let n:[i16;2]=[1,1];
    return n;
}

pub fn east()->[i16;2]{
    let n:[i16;2]=[1,0];
    return n;
}

pub fn north_east()->[i16;2]{
    let n:[i16;2]=[1,-1];
    return n;
}

pub fn cardinals()->[[i16;2];4]{
    let c:[[i16;2];4] = [north(),east(),south(),west()];
    return c;
}

pub fn full_compass()->[[i16;2];8]{
    let c:[[i16;2];8] = [north(),east(),south(),west(),north_east(),north_west(),south_east(),south_west()];
    return c;
}

pub fn in_bounds<T>(coords:&[i16;2],grid:&Vec<Vec<T>>)->bool{
    let check:bool = coords[0]>-1 && coords[1]>-1 && coords[0]<grid[0].len()as i16 && coords[1]<grid.len() as i16;
    return check;
}

pub fn addition(coords:&[i16;2],offset:&[i16;2])->[i16;2]{
    let new_pos:[i16;2]=[coords[0]+offset[0],coords[1]+offset[1]];

    return new_pos;
}