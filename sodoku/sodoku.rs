fn get_sodoku_grid() -> [[u8; 9]; 9] {
    let sodoku_grid: [[u8; 9]; 9] = [
        [3, 0, 0, 0, 9, 0, 8, 2, 0],
        [0, 1, 0, 6, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 3, 0, 0, 7, 6],
        [0, 9, 1, 0, 0, 0, 6, 4, 0],
        [0, 0, 0, 0, 2, 0, 0, 0, 8],
        [6, 0, 8, 9, 0, 0, 0, 0, 0],
        [7, 0, 6, 3, 0, 9, 2, 5, 4],
        [1, 2, 3, 5, 0, 8, 0, 6, 9],
        [0, 4, 0, 2, 0, 7, 0, 0, 0],
    ];

    return sodoku_grid;
}

fn get_row_values(sodoku_grid: [[u8; 9]; 9], row: u8) -> [u8; 9] {
    return sodoku_grid[row as usize];
}

fn get_col_values(sodoku_grid: [[u8; 9]; 9], col: u8) -> [u8; 9] {
    let mut col_values: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for i in 0..9 {
        col_values[i as usize] = sodoku_grid[i][col];
    }

    return col_values;
}




fn get_soloution_space(sodoku_grid: [[u8; 9]; 9], row: u8, col: u8) -> [u8; 9] {
    let mut solution_space: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let row_values = get_row_values(sodoku_grid, row);
    let col_values = get_col_values(sodoku_grid, col);

    for i in 0..9 {
        let val: u8 = row_values[i];
        println!("{}", val);
    }

    return solution_space;
}


fn show_grid(sodoku_grid: [[u8; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", sodoku_grid[i][j]);
        }
        println!("");
    }
}

fn main(){   
    let sodoku_grid: [[u8; 9]; 9] = get_sodoku_grid();
    show_grid(sodoku_grid);
    let solution_space = get_soloution_space(sodoku_grid, 0, 1);
    println!("{:?}", solution_space);
}