use Monitoreo_sensor_I2C::DriverI2C;

fn main() {
    // Escenario 1: Driver operando bajo condiciones estables
    let driver_estable = DriverI2C {
        voltaje_linea: 3.3,
        sensor_conectado: true,
    };

    println!("=== 1. INICIANDO MONITOREO DE TEMPERATURA (ESTABLE) ===");

    // Lecturas periodicas en bucle (Muestra lecturas exitosas y ChecksumIncorrecto en pasos enteros)
    for paso in 0..=5 {
        match driver_estable.leer_temperatura_sensor(paso) {
            Ok(t) => println!("[TELEMETRIA] Temperatura actual: {:.2} C", t),
            Err(e) => println!("Fallo de hardware en paso {}: {:?}", paso, e),
        }
    }

    println!("\n=== 2. SIMULANDO CAIDA DE TENSION EN BUS I2C (RUIDO EN LINEA) ===");
    // Escenario 2: Driver con voltaje bajo la tolerancia (RuidoEnLinea)
    let driver_ruidoso = DriverI2C {
        voltaje_linea: 2.5,
        sensor_conectado: true,
    };

    match driver_ruidoso.leer_temperatura_sensor(1) {
        Ok(_) => println!("Fallo en el test: La lectura no debio completarse"),
        Err(e) => println!("Fallo detectado de forma segura por el driver: {:?}", e),
    }

    println!("\n=== 3. SIMULANDO SENSOR DESCONECTADO (DISPOSITIVO NO RESPONDE) ===");
    // Escenario 3: Driver con sensor desconectado (DispositivoNoResponde)
    let driver_desconectado = DriverI2C {
        voltaje_linea: 3.3,
        sensor_conectado: false,
    };

    match driver_desconectado.leer_temperatura_sensor(1) {
        Ok(_) => println!("Fallo en el test: El dispositivo esta desconectado"),
        Err(e) => println!("Fallo detectado de forma segura por el driver: {:?}", e),
    }
}
