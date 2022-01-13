use std::env;
use std::process::{exit};

const MAPPING: [char; 64] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];

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

fn binarr_to_string(bin_string: Vec<Vec<u8>>, padded: u8) -> String {
    let mut value: String = String::from("");
    
    for word in bin_string {
        let a = bin_to_dec(word);

        value.push(MAPPING[a as usize]);
    }

    for _p in 0..padded / 2 {
        value += "=";
    }

    value
}

fn add_padding_if_necessary(
    encoded_binary_string: &mut Vec<Vec<u8>>,
    binary_values: &Vec<u8>,
    counter: usize
) -> u8 {
    if binary_values.len() % 6 != 0 {
        
        let six_bit_slice = &binary_values[counter*6..binary_values.len()];

        encoded_binary_string.push(six_bit_slice.to_vec());

        let mut counter: u8 = 0;
        for _i in 0..6 - encoded_binary_string[encoded_binary_string.len()-1].len() {
            let a = encoded_binary_string.len()-1;
            encoded_binary_string[a].push(0);
            counter += 1;
        }

        return counter
    }

    return 0
}

fn encode(string: &String) -> String {
    let mut counter = 0;
    let mut binary_values: Vec<u8> = Vec::new();
    let mut encoded_binary_string: Vec<Vec<u8>> = Vec::new();

    for i in string.chars() {
        let binary_value = bin(i as u8);
        binary_values.extend(binary_value);
    }

    for i in 1..binary_values.len()+1 {
        
        if i % 6 == 0 {
            let current_slice = binary_values[i-6..i].to_vec();
            encoded_binary_string.push(current_slice);
            counter += 1;
        }

    }

    let padded = add_padding_if_necessary(&mut encoded_binary_string, &binary_values, counter);

    let value: String = binarr_to_string(encoded_binary_string, padded);
    
    value
}

fn get_index_by_char(character: char) -> i8 {
    let mut index: i8 = 0;
    for value in MAPPING {
        if value == character {
            return index;
        }
        index += 1;
    }

    return -1 as i8;
}

fn get_vec_from_slices(encoded_string: String) -> Vec<u8> {
    let mut sixbit_union: Vec<u8> = Vec::new();

    for chr in encoded_string.chars() {
        let ascii_value = get_index_by_char(chr);
        let binary_value = bin(ascii_value as u8);
        let six_bit_slice = binary_value[2..binary_value.len()].to_vec();
        sixbit_union.extend(six_bit_slice);
    }

    sixbit_union
}

fn binarr_to_decoded_string(eigth_bit_arr: Vec<Vec<u8>>) -> String {
    let mut result = String::from("");

    for word in eigth_bit_arr {
        let value = bin_to_dec(word);
        let current_char = value as char;
        result.push_str(String::from(current_char).as_str());
    }

    result
}

fn decode(string: &str) -> String {
    let mut _sixbit_union: Vec<u8> = Vec::new();
    let mut eigth_bit_arr: Vec<Vec<u8>> = Vec::new();
    let mut encoded_string: String = String::from(string);

    // Remove padding from string
    encoded_string = encoded_string.replace("=", "");

    _sixbit_union = get_vec_from_slices(encoded_string);

    for i in 1.._sixbit_union.len()+1 {
        
        if i % 8 == 0 {
            let current_slice = _sixbit_union[i-8..i].to_vec();
            eigth_bit_arr.push(current_slice);
        }
    }

    let result: String = binarr_to_decoded_string(eigth_bit_arr);

    result
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("USAGE: ./b64 [OPTIONS] [ARGS]");
        println!("  [OPTIONS]");
        println!("      -e encode <value>");
        println!("      -d decode <value>");
        println!("  [ARGS]");
        println!("      <value> value to encode/decode in base64");
        exit(1);
    }

    for arg in 1..args.len() {
        match args[arg].as_str() {
            "-e" => println!("{}", encode(&args[arg+1])),
            "-d" => println!("{}", decode(&args[arg+1])),
            _ => ()
        }
    }

    exit(0);
}