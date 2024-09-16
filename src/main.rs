fn main() {
    println!("Welcome, I can convert temperatures in Fahrenheit and Celsius.");
    let source_temp_unit = 'source_unit: loop {
        match prompt_for_source_temperature_unit() {
            Ok(unit) => {
                break 'source_unit unit;
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    };

    let target_temp_unit = if source_temp_unit == "F" { "C" } else { "F" };

    println!("Source temperature unit: {}", source_temp_unit);
    println!("Target temperature unit: {}", target_temp_unit);

    let source_temp = 'source_temp: loop {
        match prompt_for_source_temperature() {
            Ok(temp) => {
                break 'source_temp temp;
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    };

    println!("Source temperature: {}", source_temp);

    let target_temp = if source_temp_unit == "F" {
        fahrenheit_to_celsius(source_temp)
    } else {
        celsius_to_fahrenheit(source_temp)
    };

    println!("Target temperature: {}", target_temp);
}

fn prompt_for_source_temperature_unit() -> Result<String, String> {
    println!("Source temperature unit? (F/C)");
    let mut source_unit = String::new();

    std::io::stdin()
        .read_line(&mut source_unit)
        .expect("Failed to read line");

    let source_unit = source_unit.trim().to_string();
    if source_unit != "F" && source_unit != "C" {
        return Err("Invalid source unit. Please enter F or C.".to_string());
    }

    Ok(source_unit)
}

fn prompt_for_source_temperature() -> Result<f64, String> {
    println!("Source temperature?");
    let mut source_temp = String::new();

    std::io::stdin()
        .read_line(&mut source_temp)
        .expect("Failed to read line");

    let source_temp = source_temp.trim().to_string();
    if source_temp.parse::<f64>().is_err() {
        return Err("Invalid source temperature. Please enter a number.".to_string());
    }

    let source_temp = source_temp.parse::<f64>().unwrap();
    Ok(source_temp)
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    format!("{:.4}", celsius).parse().unwrap()
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    format!("{:.4}", fahrenheit).parse().unwrap()
}
