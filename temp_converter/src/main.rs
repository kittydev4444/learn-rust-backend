fn convert_temp(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) + 32.0
}


fn main() {
    let input_temp: f32 = 0.0;
    let converted_temp = convert_temp(input_temp);
    println!("Converted temp: {} degrees Fahrenheit", converted_temp);
}
