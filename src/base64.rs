use std::env;
use std::process::{exit};

// TODO: move to char instead of &str
const MAPPING: [&str; 64] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","0","1","2","3","4","5","6","7","8","9","+","/"];

fn bin(mut ascii_value: u8) -> Vec<u8> {
    let mut binary_value: Vec<u8> = Vec::new();
    
    while ascii_value != 0 {
        binary_value.push(ascii_value % 2);
        ascii_value = ascii_value / 2;
    }

    binary_value.extend(vec![0; 8 - binary_value.len()]);
    binary_value.reverse();

    binary_value
}

fn bin_to_dec(mut bin_vec: Vec<u8>) -> u8 {
    let mut current_exp = 0;
    let mut current_value: u8 = 0;
    bin_vec.reverse();
    for i in bin_vec {
        current_value += i as u8 * u8::pow(2, current_exp);
        current_exp += 1;
    }

    current_value
}

fn bin_to_char(bin_string: Vec<Vec<u8>>) -> Result<String, std::io::Error> {
    let mut value: String = String::from("");
    
    for word in bin_string {
        let a = bin_to_dec(word);

        value.push_str(MAPPING[a as usize]);
    }

    Ok(value)
}

fn char_to_bin(character: char) -> Vec<u8> {
    let arr: Vec<u8> = bin(character as u8);

    arr
}

fn encode(string: &String) -> String {
    let mut conversion_data: Vec<u8> = Vec::new();

    for i in string.chars() {
        let binary_value = bin(i as u8);

        conversion_data.extend(binary_value);
    }

    let mut temp_vec: Vec<Vec<u8>> = Vec::new();

    let mut counter = 0;
    for i in 1..conversion_data.len()+1 {
        
        if i % 6 == 0 {
            let current_slice = conversion_data[i-6..i].to_vec();
            counter += 1;
            temp_vec.push(current_slice);
        }

    }

    if conversion_data.len() % 6 != 0 {
        temp_vec.push(conversion_data[counter*6..conversion_data.len()].to_vec());
        for _i in 0..6 - temp_vec[temp_vec.len()-1].len() {
            let a = temp_vec.len()-1;
            temp_vec[a].push(0);
        }
        
        let mut value: String = bin_to_char(temp_vec).unwrap();
        value += "=";
        return value

    }

    let value: String = bin_to_char(temp_vec).unwrap();

    value
}

fn get_int_by_mapping(character: char) -> i8 {
    let mut index: i8 = 0;
    for value in MAPPING {
        if value.chars().nth(0).unwrap() == character {
            return index;
        }
        index += 1;
    }

    return -1 as i8;
    // return 1 as u8;
}

fn decode(string: &str) -> String {
    let mut full_vec: Vec<u8> = Vec::new();
    let mut temp_vec: Vec<Vec<u8>> = Vec::new();
    let mut encoded_string: String = String::from(string);

    // Remove padding characters from string
    encoded_string = encoded_string.replace("=", "");

    for chr in encoded_string.chars() {
        let ascii_value = get_int_by_mapping(chr);
        let binary_value = bin(ascii_value as u8);
        let six_bit_slice = binary_value[2..binary_value.len()].to_vec();
        full_vec.extend(six_bit_slice);
    }

    // println!("Full vec, {:?}", full_vec);

    for i in 1..full_vec.len()+1 {
        
        if i % 8 == 0 {
            let current_slice = full_vec[i-8..i].to_vec();
            // counter += 1;
            temp_vec.push(current_slice);
        }
    }

    let mut result = String::from("");
    for word in temp_vec {
        let value = bin_to_dec(word);
        let current_char = value as char;
        result.push_str(String::from(current_char).as_str());
    }

    // println!("Got, {:?}", temp_vec);

    result
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("USAGE: ./b64 [OPTIONS] [ARGS]");
        println!("  [OPTIONS]");
        println!("      -e encode payload");
        println!("      -d decode payload");
        println!("  [ARGS]");
        println!("      <value> value to encode/decode in base64");
        exit(1);
    }

    for arg in 1..args.len() {
        match args[arg].as_str() {
            "-e" => println!("{}", encode(&args[arg+1])),
            // TODO: add encode function 
            "-d" => println!("{}", decode(&args[arg+1])),
            _ => ()
        }
    }

    exit(0);
}