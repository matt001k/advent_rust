use utility::input::data_input::text_input;
use std::collections::HashMap;
use std::default::Default;

struct Cube<'a> {
    map: HashMap<&'a str, u32>,
}

impl<'a> Cube<'a> {
    fn new(blue: u32, red: u32, green: u32) -> Cube<'a> {
        let mut cube = Cube { map: HashMap::new() };
        cube.map.insert("blue", blue);
        cube.map.insert("red", red);
        cube.map.insert("green", green);
        cube
    }

    fn update(self: &mut Cube<'a>, blue: u32, red: u32, green: u32) -> &mut Cube<'a> {
        self.map.insert("blue", blue);
        self.map.insert("red", red);
        self.map.insert("green", green);
        self
    }

    fn value(self: &Cube<'a>, key: &str) -> u32 {
        match self.map.get(key) {
            Some(&v) => v,
            None => 0,
        }
    }
}

impl<'a> Default for Cube<'a> {
    fn default() -> Cube<'a> {
        let mut cube = Cube { map: HashMap::new() };
        cube.map.insert("blue", 0);
        cube.map.insert("red", 0);
        cube.map.insert("green", 0);
        cube
 }
}

fn main() {
    let exit: String = String::from("exit");
    println!("Please Input The Required Calibration Data \
             And Type \"{}\" Once Done:", exit);
    let v: Vec<u32> = Vec::from(get_games(&text_input(exit)));
    println!("{:?}", v);
}

fn get_games(data: &String) -> Vec<u32> {
    let cube_lut: Cube = Cube::new(14, 12, 13);
    let mut v: Vec<u32> = Vec::new();
    for (i, line) in data.lines().enumerate() {
        let mut val: u32 = 0;
        let mut cube: Cube = Default::default();
        for (j, c) in line.chars().enumerate() {
            if c <= '9' && c >= '0' {
                if val != 0 {
                    val *= 10;
                }
                val |= c as u32 - '0' as u32;
                println!("{:?}", val);
            } else if c != ' ' || c != ',' || c != ';' {
                for key in cube_lut.map.keys() {
                    let mut k = 0;
                    for letter in key.chars() {
                        if line.as_bytes()[j + k] == letter as u8 {
                            k += 1;
                        } else {
                            break;
                        }
                        if k == key.len() {
                            let current = cube.value(key);
                            println!("{}: {:?}", key, current);
                            cube.map.insert(key, current + val);
                        }
                    }
                }
                val = 0;
            }        }
        let iter = cube_lut.map.keys();
        for key in iter {
            if cube.value(key) > cube_lut.value(key) {
                break
            } else if *key == "green" {
                v.push(i as u32);
            }
        }
    }
    v
}
