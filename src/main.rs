use std::{fmt, thread::sleep, time::Duration};

const FIELD_HEIGHT: usize = 60;
const FIELD_WIDTH: usize = 150;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Alive => write!(f, "■"),
            Cell::Dead => write!(f, " "),
        }
    }
}
fn main() {
    let mut field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [[Cell::Dead; FIELD_WIDTH]; FIELD_HEIGHT];

    // set field pattern
    for v in 0..FIELD_WIDTH {
        field[FIELD_HEIGHT / 2][v] = Cell::Alive;
    }

    loop {
        draw_field(field);
        field = generate_next_field(field);
        sleep(Duration::from_secs_f32(0.2));
        print!("\x1b[{}A", FIELD_HEIGHT);
    }
}

fn draw_field(field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) {
    for row in field {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn generate_next_field(
    field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT],
) -> [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] {
    let mut new_field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] =
        [[Cell::Dead; FIELD_WIDTH]; FIELD_HEIGHT];
    for (row_no, row) in field.iter().enumerate() {
        for (column_no, cell) in row.iter().enumerate() {
            let cnt = get_living_cells_count(&field, column_no as i32, row_no as i32);
            new_field[row_no][column_no] = dead_or_alive(cell, cnt)
        }
    }
    new_field
}

fn dead_or_alive(target_cell: &Cell, living_cells_count: i32) -> Cell {
    match living_cells_count {
        n if n < 2 => Cell::Dead,
        n if (n == 2) && *target_cell == Cell::Alive => Cell::Alive,
        n if (n == 2) && *target_cell == Cell::Dead => Cell::Dead,
        n if (n == 3) => Cell::Alive,
        n if (n > 3) => Cell::Dead,
        _ => Cell::Dead,
    }
}

fn get_living_cells_count(field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT], x: i32, y: i32) -> i32 {
    let mut count = 0;
    for y_i in (y - 1)..=(y + 1) {
        if (y_i < 0) || (y_i >= FIELD_HEIGHT as i32) {
            continue;
        }
        for x_i in (x - 1)..=(x + 1) {
            if (x_i < 0) || (x_i >= FIELD_WIDTH as i32) {
                continue;
            }
            if (y_i == y) && (x_i == x) {
                continue;
            }
            count += match field[y_i as usize][x_i as usize] {
                Cell::Alive => 1,
                Cell::Dead => 0,
            }
        }
    }
    count
}
