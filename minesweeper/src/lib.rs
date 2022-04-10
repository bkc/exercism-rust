/// Store column and row offsets compared to current point
struct Offset {
    row: isize,
    col: isize,
}

/// array of Offsets to use when getting mine_count
const POINT_OFFSETS: [Offset; 8] = [
    Offset { row: -1, col: -1 },
    Offset { row: -1, col: 0 },
    Offset { row: -1, col: 1 },
    Offset { row: 0, col: -1 },
    Offset { row: 0, col: 1 },
    Offset { row: 1, col: -1 },
    Offset { row: 1, col: 0 },
    Offset { row: 1, col: 1 },
];

const MINE_ASCII_CODE: u8 = 42;
const ZERO_ASCII_CODE: u8 = 48;

/// Calculate mine count for given row and column
fn get_mine_count(minefield: &[&str], current_row: usize, current_column: usize) -> u8 {
    let mut minecount: u8 = 0;

    if minefield[current_row].as_bytes()[current_column] == MINE_ASCII_CODE {
        return minecount;
    }

    for offset in POINT_OFFSETS {
        let row_offset = current_row as isize + offset.row;
        let col_offset = current_column as isize + offset.col;
        if row_offset < 0 || col_offset < 0 || row_offset >= minefield.len() as isize {
            continue;
        }
        let row = minefield[row_offset as usize].as_bytes();
        if col_offset >= row.len() as isize {
            continue;
        }
        if row[col_offset as usize] == MINE_ASCII_CODE {
            minecount += 1
        }
    }
    minecount
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_count = minefield.len();
    if row_count == 0 {
        return vec![];
    }
    let mut result: Vec<String> = Vec::with_capacity(row_count);
    for (row_index, row) in minefield.iter().enumerate() {
        let mut row_result: Vec<u8> = row.bytes().collect();
        for (column_index, byteval) in row_result.iter_mut().enumerate() {
            let minecount = get_mine_count(minefield, row_index, column_index);
            if minecount != 0 {
                *byteval = minecount + ZERO_ASCII_CODE;
            }
        }
        result.push(String::from_utf8(row_result).unwrap());
    }
    result
}
