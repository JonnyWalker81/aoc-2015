use std::{
    collections::HashMap,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    part1()?;
    // part2(&input)?;

    Ok(())
}

fn part1() -> Result<()> {
    let row: i64 = 2978;
    let col: i64 = 3083;

    // let row = 7;
    // let col = 7;

    let len = (row * 2).max(col * 2);

    let mut grid: Vec<Vec<i64>> = vec![vec![0; len as usize]; len as usize];

    let mut i = 20151125;
    // for n in 1..grid.len() {
    for n in 1..grid.len() {
        let mut r = n;
        let mut c = 1;
        for _ in 0..n {
            grid[r][c] = i;
            r -= 1;
            c += 1;
            i *= 252533;
            i %= 33554393;
        }
    }

    // print_grid(&grid);
    let code = grid[row as usize][col as usize];
    println!("{code}");

    Ok(())
}

fn print_grid(grid: &Vec<Vec<i64>>) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            print!("{}  ", grid[r][c]);
        }
        println!()
    }
}
