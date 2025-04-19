use std::io;
enum TempType {
    F,
    C
}


fn convert_temp(temp: f32, temp_type: TempType) -> f32 {
    match temp_type {
        TempType::F => (temp - 32.0) * (5.0 / 9.0),
        TempType::C => (temp * (9.0 / 5.0)) + 32.0
    }
    
}


fn main() {
    let mut user_temp = String::new();

    println!("Enter temperature value:");

    io::stdin()
    .read_line(&mut user_temp)
    .expect("Failed to read input");

    let input_temp: f32 = user_temp.trim().parse().expect("Invalid temperature input");

    let mut input_temp_type = String::new();

    println!("Is the current temperature in Fahrenheit or Celsius: (enter F or C) :");

    io::stdin()
    .read_line(&mut input_temp_type)
    .expect("Failed to read input");

    let current_temp_type = match input_temp_type.trim().to_uppercase().as_str() {
        "F" => TempType::F,
        "C" => TempType::C,
        _ => panic!("Invalid temperature type"),
    };

    let converted_temp_type = match current_temp_type {
        TempType::F => "Celsius",
        TempType::C => "Fahrenheit"
    };

    let converted_temp = convert_temp(input_temp, current_temp_type);

    println!("Converted temp: {} degrees {}", converted_temp, converted_temp_type);
}
