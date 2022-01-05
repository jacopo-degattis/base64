
const MAPPING: [&str; 62] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","w","x","y","z","0","1","3","4","5","6","7","8","9","+","/"];

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

fn bin_to_dec(mut bin_vec: Vec<u8>) -> u32 {
    let mut current_exp = 0;
    let mut current_value: u32 = 0;
    bin_vec.reverse();
    for i in bin_vec {
        current_value += i as u32 * u32::pow(2, current_exp);
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

fn encode(string: String) -> String {
    let mut conversion_data: Vec<u8> = Vec::new();

    for i in string.chars() {
        let binary_value = bin(i as u8);
        conversion_data.extend(binary_value);
    }

    let mut temp_vec: Vec<Vec<u8>> = Vec::new();

    println!("# Length: {}", conversion_data.len());

    let mut counter = 0;
    for i in 1..conversion_data.len()+1 {
        
        if i % 6 == 0 {
            let current_slice = conversion_data[i-6..i].to_vec();
            counter += 1;
            println!("Current, {:?}", current_slice);

            temp_vec.push(current_slice);
        }

        println!("Current value, {}", i);

    }

    if conversion_data.len() % 6 != 0 {
        temp_vec.push(conversion_data[counter*6..conversion_data.len()].to_vec());
        for i in 0..6 - temp_vec[temp_vec.len()-1].len() {
            let a = temp_vec.len()-1;
            temp_vec[a].push(0);
        }
        
        let mut value: String = bin_to_char(temp_vec).unwrap();
        value += "=";
        return value

    }

    let mut value: String = bin_to_char(temp_vec).unwrap();

    value
}

fn main() {
    let e_value = encode("telefono".to_string());

    println!("Encoded value: {}", e_value);
}