use Decodificador_IoT::{BrokerIot, MensajeIot, Prioridad};

fn main() {
    let mut broker = BrokerIot::nuevo();

    // Mensajes de prueba simulando rafagas de red
    let mensajes = vec![
        MensajeIot::Ping,
        MensajeIot::Telemetria(101, 24.5),
        MensajeIot::Telemetria(102, 95.0),
        MensajeIot::Comando(4, Prioridad::Alta),
        MensajeIot::Comando(5, Prioridad::Media),
    ];

    // Procesamiento en bucle de cada mensaje de red
    for msg in mensajes {
        broker.registrar_latencia(&msg);
        broker.procesar_mensaje(msg);
    }

    println!("\nTotal de mensajes procesados: {}", broker.mensajes_procesados);
}
