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
fn get_mine_count(minefield: &[&str], current_row: isize, current_column: isize) -> u8 {
    let mut minecount: u8 = 0;
    if minefield[current_row as usize].as_bytes()[current_column as usize] == MINE_ASCII_CODE {
        return minecount;
    }

    for offset in POINT_OFFSETS {
        let row_offset = current_row + offset.row;
        let col_offset = current_column + offset.col;
        if row_offset < 0 || col_offset < 0 {
            continue;
        }
        if let Some(row) = minefield.get(row_offset as usize) {
            if let Some(&byte) = row.as_bytes().get(col_offset as usize) {
                if byte == MINE_ASCII_CODE {
                    minecount += 1
                }
            }
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
            let minecount = get_mine_count(minefield, row_index as isize, column_index as isize);
            if minecount != 0 {
                *byteval = minecount + ZERO_ASCII_CODE;
            }
        }
        result.push(String::from_utf8(row_result).unwrap());
    }
    result
}
