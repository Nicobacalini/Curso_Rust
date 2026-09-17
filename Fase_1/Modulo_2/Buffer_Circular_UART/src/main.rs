use Buffer_Circular_UART::UartRingBuffer;

fn main() {
    println!("=== INICIANDO CONTROLADOR UART DE HARDWARE ===\n");

    // Inicializacion del buffer principal
    let mut uart = UartRingBuffer::nuevo();

    // Recepcion de bytes de telemetria
    println!("--- Recibiendo 3 bytes de telemetria ---");
    uart.escribir(100);
    uart.escribir(101);
    uart.escribir(102);

    // Procesamiento de datos en orden FIFO
    println!("\n--- Procesando datos en CPU ---");
    if let Some(dato) = uart.leer() {
        println!("Dato procesado con exito: {}", dato);
    }
    if let Some(dato) = uart.leer() {
        println!("Dato procesado con exito: {}", dato);
    }

    // Rafaga de datos que satura el buffer y genera overflow
    println!("\n--- Llegada de rafaga de alta velocidad ---");
    uart.escribir(103);
    uart.escribir(104);
    uart.escribir(105);
    uart.escribir(106);

    // Bloque temporal para probar la destruccion anticipada con drop
    println!("\n--- Apagado repentino del modulo ---");
    {
        let mut uart_temporal = UartRingBuffer::nuevo();
        uart_temporal.escribir(200);
        uart_temporal.escribir(201);

        println!("Saliendo del ambito del controlador temporal...");
        // Al salir del bloque uart_temporal se destruye con 2 bytes sin leer
    }

    println!("\n--- Fin del programa principal ---");
}
