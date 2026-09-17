use analizador_red::{formatear_mac, VistaTramaEthernet};

fn main() {
    // Buffer binario que simula una trama Ethernet II tal como la entrega el SO
    let buffer_tarjeta_red: [u8; 24] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MAC Destino (Broadcast)
        0x00, 0x15, 0x5D, 0x01, 0x1A, 0x02, // MAC Origen
        0x08, 0x00, // Tipo IPv4
        0x48, 0x6F, 0x6C, 0x61, 0x21, 0x21, // Datos (Hola!!)
        0xDE, 0xAD, 0xBE, 0xEF, // CRC
    ];

    println!("=== INICIANDO ANALIZADOR DE RED ZERO-COPY ===\n");

    // Instanciacion de la vista sobre el buffer por referencia
    let vista = VistaTramaEthernet::registrar(&buffer_tarjeta_red);

    // Formateo e impresion de direcciones MAC
    println!("MAC Destino: {}", formatear_mac(vista.direccion_destino()));
    println!("MAC Origen: {}", formatear_mac(vista.direccion_origen()));

    // Protocolo encapsulado
    println!("Protocolo: 0x{:04X}", vista.tipo_protocolo());

    // Tamano total y payload
    println!("Tamano total: {}", vista.raw_data.len());
    let payload = vista.payload();
    println!("Tamano del payload: {}", payload.len());
    println!("Payload: {}", String::from_utf8_lossy(payload));

    // CRC de seguridad
    let crc = vista.crc();
    println!(
        "CRC: 0x{:02X}{:02X}{:02X}{:02X}",
        crc[0], crc[1], crc[2], crc[3]
    );
}
