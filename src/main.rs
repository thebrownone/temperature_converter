fn temperature_conversion (temp: i32, unit: char) -> i32 {
    if unit == 'C' {
        let temp_farenheit = (temp * 9/5) + 32;
        temp_farenheit
    } else if unit == 'F' {
        let temp_celsius = (temp - 32) * 5/9;
        temp_celsius
    } else {
        println!("Invalid unit");
        temp
    }
}

fn main() {
    let temp_celsius = 30;
    let temp_farenheit = 86;
    println!("{} degrees Celsius is {} degrees Fahrenheit", temp_celsius, temperature_conversion(temp_celsius, 'C'));
    println!("{} degrees Fahrenheit is {} degrees Celsius", temp_farenheit, temperature_conversion(temp_farenheit, 'F'));
}


