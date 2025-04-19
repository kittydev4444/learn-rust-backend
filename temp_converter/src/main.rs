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
    let input_temp: f32 = 32.0;
    let input_temp_type = TempType::F;

    let converted_temp_type = match input_temp_type {
        TempType::F => "Celsius",
        TempType::C => "Fahrenheit"
    };

    let converted_temp = convert_temp(input_temp, input_temp_type);
    
    println!("Converted temp: {} degrees {}", converted_temp, converted_temp_type);
}
