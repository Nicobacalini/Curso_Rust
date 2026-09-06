use conversor_temp::{celsius_a_fahrenheit, clasificar_temperatura, fahrenheit_a_celsius};
use std::io;

// Solicita una temperatura por teclado y valida entrada numerica decimal
fn pedir_temperatura(mensaje: &str) -> f64 {
    loop {
        println!("{}", mensaje);
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");

        match entrada.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Entrada no valida. Introduce un numero decimal.\n");
            }
        }
    }
}

// Muestra el menu de opciones y valida seleccion entre 1 y 3
fn pedir_opcion_menu() -> u32 {
    loop {
        println!("--- Menu Conversor de Temperatura ---");
        println!("1. Convertir de Celsius a Fahrenheit");
        println!("2. Convertir de Fahrenheit a Celsius");
        println!("3. Salir del programa");
        println!("Elige una opcion (1-3):");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la opcion");

        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Introduce un numero valido.\n");
                continue;
            }
        };

        if (1..=3).contains(&opcion) {
            return opcion;
        } else {
            println!("Opcion no valida. Elige entre 1 y 3.\n");
        }
    }
}

// Bucle principal de la aplicacion interactiva
fn ejecutar_conversor() {
    loop {
        let opcion = pedir_opcion_menu();

        match opcion {
            1 => {
                let celsius = pedir_temperatura("Introduce los grados Celsius:");
                let fahrenheit = celsius_a_fahrenheit(celsius);
                let sensacion = clasificar_temperatura(celsius);

                println!(
                    "{:.1} C son {:.1} F -> Sensacion: {}.\n",
                    celsius, fahrenheit, sensacion
                );
            }
            2 => {
                let fahrenheit = pedir_temperatura("Introduce los grados Fahrenheit:");
                let celsius = fahrenheit_a_celsius(fahrenheit);
                let sensacion = clasificar_temperatura(celsius);

                println!(
                    "{:.1} F son {:.1} C -> Sensacion: {}.\n",
                    fahrenheit, celsius, sensacion
                );
            }
            3 => {
                println!("Hasta luego!");
                break;
            }
            _ => unreachable!(),
        }
    }
}

// Demostracion inicial iterando un arreglo fijo de temperaturas
fn probar_array_inicial() {
    let temperaturas_prueba: [f64; 3] = [-5.0, 20.0, 36.5];
    println!("=== Pruebas iniciales con Array y For ===");
    for temp in temperaturas_prueba {
        let fahrenheit = celsius_a_fahrenheit(temp);
        let sensacion = clasificar_temperatura(temp);
        println!(
            "{:.1} C equivale a {:.1} F ({})",
            temp, fahrenheit, sensacion
        );
    }
    println!("========================================\n");
}

fn main() {
    probar_array_inicial();
    ejecutar_conversor();
}
