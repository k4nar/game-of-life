extern crate rand;

use std::fmt;

static WIDTH: usize = 80;
static HEIGHT: usize = 40;

#[derive(Copy, Clone)]
enum State {
    Alive = 1,
    Dead = 0,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match *self {
            State::Alive => 'o',
            State::Dead => ' '
        };

        write!(f, "{}", state)
    }
}

struct Grid {
    content: Vec<Vec<State>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "".to_string();
        for line in &self.content {
            let mut string_line = "".to_string();
            for column in line {
                string_line = string_line + &column.to_string();
            }
            output = output + &string_line + "\n";
        }

        write!(f, "{}", output)
    }
}

fn am_i_alive(grid: &Grid, cell: &State, cell_x: usize, cell_y: usize) -> State {
    let alive_neighbours = grid.content.iter().enumerate().map(|(x, line)| {
        line.iter().enumerate().map(|(y, cell)| {
            if (x >= cell_x.saturating_sub(1) && x <= cell_x + 1) &&
               (y >= cell_y.saturating_sub(1) && y <= cell_y + 1) &&
               !(x == cell_x && y == cell_y) {
                return match *cell {
                    State::Alive => 1,
                    State::Dead => 0,
                }
            }
            0
        }).fold(0, std::ops::Add::add)
    }).fold(0, std::ops::Add::add);

    match *cell {
        State::Alive if alive_neighbours < 2 || alive_neighbours > 3 => State::Dead,
        State::Dead if alive_neighbours == 3 => State::Alive,
        state => state
    }
}

// RULES
// 1. Fewer to 2 cells around it it dies of isolation
// 2. More than 3 cells around it it dies of owercrowding
// 3. 3 living cells around a dead one make it alive

fn main() {
    let mut grid = Grid {
        content: vec![vec![State::Dead; WIDTH]; HEIGHT],
    };

    // initialization
    grid.content = grid.content.iter_mut().map(|line| {
        line.iter_mut().map(|_| {
            match rand::random() {
                true => State::Alive,
                false => State::Dead
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    loop {
        grid.content = grid.content.iter().enumerate().map(|(x, line)| {
            line.iter().enumerate().map(|(y, cell)| {
                am_i_alive(&grid, cell, x, y)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        print!("{}[2J", 27 as char);  // clear the screen
        println!("{}", grid);
    }

}
