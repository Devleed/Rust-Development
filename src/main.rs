fn main() {
    // println!("{}", log(LogLevel::Error, "Stack overflow!"));
    // println!("{:#?}", sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1]));
    // println!("{}", get_success_rate(5));
    // println!("{}", production_rate_per_hour(4));
    // println!("{} {}", divmod(10, 3).0, divmod(10, 3).1);
    // println!("{}", reverse("Hello"))
    // println!("{}", is_armstrong_number(13));
    // let dead_player = Player {
    //     health: 1,
    //     mana: None,
    //     level: 0
    // };

    // let player = dead_player.revive();

    // println!("{:#?}", player);

    // let mut test: Option<i32> = Some(23);

    // match test.as_mut() {
    //     Some(x) => {
    //         *x = 43;
    //     },
    //     None => println!("None")
    // }

    // println!(" hahahahah - {}", test.unwrap());

    // let mut wizard = Player { health: 20, mana: None, level: 18 };
    // assert_eq!(wizard.cast_spell(30), 0);
    // assert_eq!(wizard.health, 10);
    // assert_eq!(wizard.mana, None);
    // check_dereferencing();

    // let mut nums: Vec<u32> = vec![1, 2, 3, 4, 5,8,6,34];
    // println!("{}", find_median(&mut nums));

    // println!("{}", find_max_in_2d_arr());

    println!("{} ", x_and_o("xox"));
}

// ! LOG LEVEL EXERCISE
#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
pub fn log(level: LogLevel, message: &str) -> String {
    return match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    };
}
pub fn info(message: &str) -> String {
    String::from("[INFO]: ") + message
}
pub fn warn(message: &str) -> String {
    String::from("[WARN]: ") + message
}
pub fn error(message: &str) -> String {
    String::from("[ERROR]: ") + message
}
// ! END HERE

// ! SUB LIST EXERCISE
#[derive(Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (name, equal) = compare_arrays(_first_list, _second_list);

    if equal {
        return name;
    } else {
        return Comparison::Unequal;
    }
}

pub fn compare_arrays<T: PartialEq>(first_array: &[T], second_array: &[T]) -> (Comparison, bool) {
    let (smaller_array, name) = if first_array.len() > second_array.len() {
        (second_array, Comparison::Superlist)
    } else if first_array.len() < second_array.len() {
        (first_array, Comparison::Sublist)
    } else {
        (first_array, Comparison::Equal)
    };

    let mut equal = false;

    for i in smaller_array {
        let is_elem_present = match name {
            Comparison::Superlist => first_array.contains(i),
            Comparison::Sublist => second_array.contains(i),
            _ => first_array.contains(i) && second_array.contains(i),
        };

        if is_elem_present {
            equal = true;
        } else {
            equal = false;
            break;
        }
    }

    return (name, equal);
}

// fn is_elem_in_list(elem: &u8, list: &[u8]) -> bool {
//     for i in 0..list.len() {
//         if list[i] == *elem {
//             return true;
//         }
//     }
//     return false;
// }
// ! END HERE - INCOMPLETE

// ! ASSEBLE LINE EXCERSISE

const CARS_PRODUCED_AT_LOWEST_RATE: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_percentage = get_success_rate(speed) as u128;

    let cars_produced: u16 = speed as u16 * CARS_PRODUCED_AT_LOWEST_RATE as u16;

    let production_rate: f64 = (cars_produced as f64 / 100.0) * success_percentage as f64;

    production_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let result = (production_rate_per_hour(speed) / 60.0) as u32;

    return result;
}

pub fn get_success_rate(speed: u8) -> u8 {
    return match speed {
        1..=4 => 100,
        5..=8 => 90,
        9..=10 => 77,
        _ => 0,
    };
}
// ! END HERE

// ! CHECKING DEREFERENCING
fn check_dereferencing() {
    let mut x = 5;
    let y = &mut x;
    *y = 23;
    // let z = *y;

    // *y = 10;

    // println!("{}", z);
    println!("{}", x);
}

// ! USER STRUCT EXCERSISE
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User { name, age, weight }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

// ! END HERE

// ! QUOTIENT REMAINDER EXCERSISE
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = (dividend as f64 / divisor as f64).floor() as i16;
    let remainder = dividend % divisor;

    return (quotient, remainder);
}

// ! END HERE

// ! REVERSE STRING
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

// ! ARMSTRONG NUMBER
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string().chars().collect::<Vec<char>>();
    let mut sum: u32 = 0;

    for (_, digit) in digits.iter().enumerate() {
        let powered_digit = digit
            .to_string()
            .parse::<u32>()
            .unwrap()
            .pow(digits.len() as u32);

        sum += powered_digit;
    }

    println!("{}", sum);

    sum == num
}

// ! END HERE

// ! OPTION EXCERCISE
#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            let mana = if self.level >= 10 { Some(100) } else { None };
            return Some(Player {
                health: 100,
                mana,
                level: self.level,
            });
        }
        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana.as_mut() {
            Some(mana) => {
                if *mana < mana_cost {
                    return 0;
                }
                *mana = *mana - mana_cost;
                return mana_cost * 2;
            }
            None => {
                if mana_cost > self.health {
                    self.health = mana_cost - self.health;
                } else {
                    self.health = self.health - mana_cost;
                }
                return 0;
            }
        };
    }
}

// ! END HERE

// ! MEDIAN MODE EXCERSISE
fn find_median(nums: &mut Vec<u32>) -> u32 {
    let nums_len = nums.len();

    nums.sort();

    if nums_len % 2 == 0 {
        // if odd
        return (nums[(nums_len / 2) - 1] + nums[(nums_len / 2) + 1]) / 2;
    } else {
        // if even
        return nums[nums_len / 2];
    }
}

// ! END HERE

// ! MAX 2D ARRAY EXCERSISE
fn find_max_in_2d_arr() -> i128 {
    let input = "10 3
1 5 3
4 8 7
6 9 1"
        .lines();

    let vec: Vec<&str> = input.collect();
    let mut result_vec: Vec<i128> = Vec::new();
    let mut max: i128 = 0;

    for (i, num_with_whitespace) in vec.iter().enumerate() {
        let num_without_whitespace: Vec<&str> = num_with_whitespace.split_whitespace().collect();

        if i != 0 {
            for i in (convert_str_to_int(num_without_whitespace[0]) - 1)
                ..=(convert_str_to_int(num_without_whitespace[1]) - 1)
            {
                match result_vec.get(i as usize) {
                    Some(elem) => {
                        let num_to_insert = *elem + convert_str_to_int(num_without_whitespace[2]);

                        result_vec[i as usize] = num_to_insert;

                        if max < num_to_insert {
                            max = num_to_insert;
                        }
                    }
                    None => {
                        if max < convert_str_to_int(num_without_whitespace[2]) {
                            max = convert_str_to_int(num_without_whitespace[2])
                        }

                        result_vec.push(convert_str_to_int(num_without_whitespace[2]));
                    }
                }
            }
        }
    }

    return max;
}

fn convert_str_to_int(input: &str) -> i128 {
    return input.parse::<i128>().unwrap();
}

// ! END HERE

// ! X'S AND O'S EXCERSISE
fn x_and_o(string: &'static str) -> bool {
    let input = string.to_lowercase();
    input.matches("x").count() == input.matches("o").count()
}
// ! END HERE
