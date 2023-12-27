use utility::input::data_input::text_input;

enum Test {
    Test1,
    Test2,
}

struct Data {
    d: Vec<char>,
    row: usize,
    col: usize,
}

impl Data {
    pub fn new(data: String) -> Data {
        let mut ret = Data {
            d: Default::default(),
            row: 0,
            col: 0,
        };
        for line in data.lines() {
            if ret.col == 0 {
                ret.col = line.len();
            }
            for c in line.chars() {
                ret.d.push(c);
            }
            ret.row += 1;
        }
        ret
    }
    fn get_index(self: &mut Data, row: usize, col: usize) -> char {
        self.d[row * self.col + col]
    }

    fn get_num(self: &mut Data, c: char, val: &mut u8) -> bool {
        let mut ret: bool = false;
        if c >= '0' && c <= '9' {
            ret = true;
            *val = c as u8 - '0' as u8;
        }
        ret
    }

    fn get_special_char(self: &mut Data, row: usize,
                        col: usize, num_check: bool) -> bool {
        let mut ret: bool = false;
        let mut c: char;
        for i in 0..3 {
            let c_read: i32 = col as i32 - 1 + i;
            if c_read > 0 && c_read < self.col as i32 {
                c = self.get_index(row, c_read as usize);
                ret = c != '.' && (c > '9' || c < '0' ||
                                   (num_check && c > '0' && c < '9'));
                if ret == true {
                    break;
                }
            }
        }
        ret
    }

    fn find_adjacent_special_char(self: &mut Data, row: usize,
                                  col: usize, num_check: bool) -> u8  {
        let mut ret: u8 = 0;
        // Check Top Adjacent Values
        if row > 0 {
            ret += if self.get_special_char(row - 1, col, num_check) == true { 1 } else { 0 };
        }
        // Check Middle Row
        ret += if self.get_special_char(row, col, num_check) == true { 1 } else { 0 };
        // Check Bottom Row
        if row < self.row - 1 {
            ret += if self.get_special_char(row + 1, col, num_check) == true { 1 } else { 0 };
        }
        ret
    }

    fn part_1(self: &mut Data, i: usize, j: &mut usize, ret: &mut u32) {
        const TEN: u32 = 10;
        let mut digit: u8 = 0;
        let mut val: u32 = 0;
        let mut offset: usize = 0;
        let mut found: bool = false;
        let mut c: char = self.get_index(i, *j);
        while self.get_num(c, &mut digit) {
            if found == false {
                found = if self.find_adjacent_special_char(i, *j + offset, false) > 0 { true } else { false };
            }
            val = val * TEN + digit as u32;
            offset += 1;
            c = self.get_index(i, *j + offset);
        }
        if found == true {
            *j += offset;
            *ret += val;
        } else {
            *j += 1;
        }
    }

    fn part_2(self: &mut Data, i: usize, j: &mut usize, ret: &mut u32) {
        if self.get_index(i, *j) == '*' && self.find_adjacent_special_char(i, *j, true) > 1 {
            println!("Found at index {} {}", i, *j);
            // Find beginning of number
        }
        *j += 1;
    }

    pub fn get_result(self: &mut Data, test: Test) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..self.row {
            let mut j: usize = 0;
            while j < self.col {
                match test {
                    Test::Test1 => self.part_1(i, &mut j, &mut ret),
                    Test::Test2 => self.part_2(i, &mut j, &mut ret),
                }
            }
        }
        ret
    }
}

fn main() {
    let exit: String = String::from("exit");
    let mut data = Data::new(text_input(exit));
    println!("{}", data.get_result(Test::Test1));
    println!("{}", data.get_result(Test::Test2));
}
