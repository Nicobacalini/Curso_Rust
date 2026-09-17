use Gestor_Mem_Virutal::MemoriaVirtual;

fn main() {
    let mut ram = MemoriaVirtual::inicializar();

    println!("--- Empezando asignacion de memoria ---");

    // Busqueda del primer bloque libre
    match ram.buscar_bloque_libre() {
        Some(indice) => println!("Bloque libre encontrado en la posicion fisica: {}", indice),
        None => println!("Alerta! Desbordamiento de memoria, RAM llena."),
    }

    // Intento de escritura en area reservada del Kernel (posicion 0)
    match ram.asignar_bloque(0, 99, false) {
        Ok(_) => println!("Escritura exitosa en bloque 0"),
        Err(e) => println!("Error esperado al escribir en bloque 0: {:?}", e),
    }

    // Intento de escritura en un bloque libre
    if let Some(libre) = ram.buscar_bloque_libre() {
        match ram.asignar_bloque(libre, 99, false) {
            Ok(_) => println!("Bloque {} asignado con exito al proceso 99", libre),
            Err(e) => println!("Fallo inesperado: {:?}", e),
        }
    }
}
