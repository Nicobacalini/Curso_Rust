use Router_Eventos::{
    procesar_transaccion, ClienteId, MetadataTransaccion, Transaccion, TransaccionId,
};

// Procesa un conjunto de transacciones encadenadas utilizando el operador `?`.
// Si cualquier transaccion falla (devuelve `Err`), la funcion se interrumpe
// e inmediatamente retorna el error recibido.
fn procesar_lote_encadenado() -> Result<(), String> {
    let tx1 = Transaccion::Pago {
        id: TransaccionId(101),
        cliente: ClienteId(1),
        monto: 250.0,
        meta: MetadataTransaccion {
            prioridad: 2,
            ..Default::default()
        },
    };

    let tx2 = Transaccion::Pago {
        id: TransaccionId(102),
        cliente: ClienteId(2),
        monto: 15_000.0,
        meta: MetadataTransaccion {
            prioridad: 10,
            ..Default::default()
        },
    };

    let tx3 = Transaccion::Reembolso {
        id: TransaccionId(103),
        motivo: String::from("Devolucion por defecto de fabrica"),
        monto: 50.0,
    };

    let tx4 = Transaccion::BloqueoCuenta(ClienteId(2));

    // El operador '?' propaga el Result Ok(String) o corta la ejecucion si devuelve Err(String).
    let r1 = procesar_transaccion(tx1)?;
    println!("OK: {}", r1);

    let r2 = procesar_transaccion(tx2)?;
    println!("OK: {}", r2);

    let r3 = procesar_transaccion(tx3)?;
    println!("OK: {}", r3);

    let r4 = procesar_transaccion(tx4)?;
    println!("OK: {}", r4);

    Ok(())
}

fn probar_transaccion_con_error() -> Result<(), String> {
    let tx_valida = Transaccion::Pago {
        id: TransaccionId(201),
        cliente: ClienteId(5),
        monto: 100.0,
        meta: MetadataTransaccion {
            timestamp: 1700000000,
            ..Default::default()
        },
    };

    let tx_invalida = Transaccion::Pago {
        id: TransaccionId(202),
        cliente: ClienteId(5),
        monto: -50.0, // Falla por monto <= 0.0
        meta: MetadataTransaccion {
            ..Default::default()
        },
    };

    let r1 = procesar_transaccion(tx_valida)?;
    println!("OK Transaccion previa exitosa: {}", r1);

    // Aqui se interrumpe la ejecucion automaticamente por el operador '?'
    let r2 = procesar_transaccion(tx_invalida)?;
    println!("OK Esto no se imprimira: {}", r2);

    Ok(())
}

fn main() {
    println!("=== Procesando Lote Valido con Operador ? ===");
    if let Err(e) = procesar_lote_encadenado() {
        println!("Error procesando lote: {}", e);
    } else {
        println!("--> Todo el lote fue procesado con exito.");
    }

    println!("\n=== Demostracion de Interrupcion Anticipada con ? ===");
    if let Err(err) = probar_transaccion_con_error() {
        println!("Se capturo un error encadenado correctamente: '{}'", err);
    }
}
