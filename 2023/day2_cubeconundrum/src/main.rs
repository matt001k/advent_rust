use utility::input::data_input::text_input;
use utility::math;
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
    let data = text_input(exit);
    let v: Vec<u32> = Vec::from(get_games(&data));
    println!("{:?}", math::get_sum::<u32>(v));
    let v: Vec<u32> = Vec::from(get_powers(&data));
    println!("{:?}", math::get_sum::<u32>(v));
}

fn get_games(data: &String) -> Vec<u32> {
    let cube_lut: Cube = Cube::new(14, 12, 13);
    let mut v: Vec<u32> = Vec::new();
    for (i, line) in data.lines().enumerate() {
        let mut found = 1;
        let mut val: u32 = 0;
        let mut cube: Cube = Default::default();
        for (j, c) in line.chars().enumerate() {
            if c <= '9' && c >= '0' {
                if val != 0 {
                    val *= 10;
                }
                val += c as u32 - '0' as u32;
            } else if c != ' ' && c != ',' && c != ';' {
                for key in cube_lut.map.keys() {
                    let mut k = 0;
                    for letter in key.chars() {
                        if line.as_bytes()[j + k] == letter as u8 {
                            k += 1;
                        } else {
                            break;
                        }
                        if k == key.len() {
                            cube.map.insert(key, val);
                        }
                    }
                    let iter = cube_lut.map.keys();
                    for key in iter {
                        if cube.value(key) > cube_lut.value(key) {
                            found = 0;
                            break;
                        }                         
                    }
                }
                val = 0;
            }
            if found == 0 {
                break;
            }
        }
        if found == 1 {
            v.push((i + 1) as u32);
        }
    }
    v
}

fn get_powers(data: &String) -> Vec<u32> {
    let cube_lut: Cube = Default::default();
    let mut v: Vec<u32> = Vec::new();
    for line in data.lines() {
        let mut val: u32 = 0;
        let mut cube: Cube = Default::default();
        for (j, c) in line.chars().enumerate() {
            if c <= '9' && c >= '0' {
                if val != 0 {
                    val *= 10;
                }
                val += c as u32 - '0' as u32;
            } else if c != ' ' && c != ',' && c != ';' {
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
                            if val > current {
                                cube.map.insert(key, val);
                            }
                        }
                    }
                }
                val = 0;
            }
        }
        let mut power = 0;
        let iter = cube.map.keys();
        for key in iter {
            if power != 0 {
                power *= cube.value(key);
            }
            else {
                power = cube.value(key);
            }

        }                         
        v.push(power);
    }
    v
}
