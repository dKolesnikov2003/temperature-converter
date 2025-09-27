use std::io;

const C: f64 = 32.0;

fn convert_c_to_f (temperature_c: f64) -> f64 {
    temperature_c * 9.0 / 5.0 + C 
}

fn convert_f_to_c(temperature_f: f64) -> f64 {
    (temperature_f - C) * 5.0 / 9.0
}

fn convert_temperature (temperature_value: f64, convertion_mode: i8) -> Option<f64> {
    match convertion_mode {
        1 => Some(convert_c_to_f (temperature_value)),
        2 => Some(convert_f_to_c (temperature_value)),
        _ => None
    }
}


fn main() {
    println!("Temperature converter.");

    println!("Type [1] for C to F or [2] for F to C convertion:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let convertion_mode: i8 = match user_input.trim().parse() {
        Ok(val) if val == 1 || val == 2 => val,
        Ok(_) => {
            eprintln!("Unknown convertion mode (must be 1 or 2).");
            return;
        },
        Err(_) => {
            eprintln!("Unknown convertion mode (must be 1 or 2).");
            return;
        }
    };

    println!("Enter the value to convert:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let temperature_value: f64 = match user_input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid temperature value (must be a number).");
            return;
        }
    };

    match convert_temperature(temperature_value, convertion_mode) {
        Some(result) => println!("Convertion result is: {}°", result),
        None =>  println!("Unknown convertion mode!")
    }
}
