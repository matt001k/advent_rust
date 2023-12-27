use utility::input::data_input::text_input;
use utility::math;

fn main() {
    let exit: String = String::from("exit");
    println!("Please Input The Required Calibration Data \
             And Type \"{}\" Once Done:", exit);

    println!("The Sum of The Calibration Values is: {}",
             math::get_sum::<u32>(get_values(&text_input(exit))));
}

fn get_values(data: &String) -> Vec<u32> {
    const STRINGLUT: [&str; 10] = ["zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"];
    let mut v: Vec<u32> = Vec::new();
    let (mut val1, mut val2): (i16, i16) = (-1, -1);
    for (i, c) in data.chars().enumerate() {
        let int: i16 = c as i16 - '0' as i16;
        if c == '\n' {
            if val1 != -1 {
                if val2 == -1 {
                    val2 = val1;
                }
                v.push((val1 * 10 + val2).try_into().unwrap());
                val1 = -1;
                val2 = -1;
            }
        } else if  int <= 9 && int >= 0 {
            assign_values(int, &mut val1, &mut val2);
        } else {
            for (k, s) in STRINGLUT.iter().enumerate() {
                let mut j = 0;
                for letter in s.chars() {
                    if data.as_bytes()[i + j] == letter as u8 {
                        j += 1;
                    } else {
                        break;
                    }
                    if j == s.len() {
                        assign_values(k as i16, &mut val1, &mut val2);
                    }
                }
            }
        }
    }
    v
}

fn assign_values(number: i16, val1: &mut i16, val2: &mut i16) {
    if *val1 == -1 {
        *val1 = number;
    } else {
        *val2 = number;
    }
}

