use std::io;

struct Temperature<'a> {
  temp: f64,
  unit: &'a str,
}

fn main() {
    println!("Please input a temperature...");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: Temperature = get_temperature_from_string(temperature.trim());

    if temperature.unit == "celsius" {
        println!(
            "{}C is equal to {}F",
            temperature.temp,
            celsius_to_fahrenheit(temperature.temp)
        );
    } else if temperature.unit == "fahrenheit" {
        println!(
            "{}F is equal to {}C",
            temperature.temp,
            fahrenheit_to_celsius(temperature.temp)
        );
    }
}

fn get_temperature_from_string(temp_string: &str) -> Temperature {
    let temp_string_bytes = temp_string.as_bytes();
    let mut temp: &str = "";
    let mut unit = "fahrenheit";
    for (i, &item) in temp_string_bytes.iter().enumerate() {
        if item == b'F' || item == b'f' {
            temp = &temp_string[0..i];
            break;
        } else if item == b'C' || item == b'c' {
            temp = &temp_string[0..i];
            unit = "celsius";
            break;
        }
    }
    let temp: f64 = match temp.parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid temperature!"),
    };
    return Temperature {
      temp,
      unit,
    };
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0) / 5.0 + 32.0
}