//! # Analizador de Red Zero-Copy
//!
//! Modulo para el analisis eficiente de tramas Ethernet II utilizando vistas
//! de solo lectura con lifetimes (`'a`) para evitar copias innecesarias en memoria.

/// Vista de solo lectura y Zero-Copy de una trama Ethernet II.
/// El lifetime `'a` vincula este struct con la vida del buffer original.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VistaTramaEthernet<'a> {
    pub raw_data: &'a [u8],
}

impl<'a> VistaTramaEthernet<'a> {
    /// Registra un nuevo buffer de red para su analisis.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use analizador_red::VistaTramaEthernet;
    ///
    /// let buffer = [0u8; 32];
    /// let vista = VistaTramaEthernet::registrar(&buffer);
    /// assert_eq!(vista.raw_data.len(), 32);
    /// ```
    pub fn registrar(buffer: &'a [u8]) -> Self {
        VistaTramaEthernet { raw_data: buffer }
    }

    /// Devuelve un slice de 6 bytes que apunta a la MAC Destino.
    pub fn direccion_destino(&self) -> &'a [u8] {
        &self.raw_data[8..14]
    }

    /// Devuelve un slice de 6 bytes que apunta a la MAC Origen.
    pub fn direccion_origen(&self) -> &'a [u8] {
        &self.raw_data[14..20]
    }

    /// Devuelve el EtherType decodificado en formato Big-Endian.
    pub fn tipo_protocolo(&self) -> u16 {
        let bytes: [u8; 2] = [self.raw_data[20], self.raw_data[21]];
        u16::from_be_bytes(bytes)
    }

    /// Devuelve los datos utiles excluyendo cabecera y CRC.
    pub fn payload(&self) -> &'a [u8] {
        let largo = self.raw_data.len();
        &self.raw_data[22..largo - 4]
    }

    /// Devuelve los ultimos 4 bytes correspondientes al CRC.
    pub fn crc(&self) -> &'a [u8] {
        let largo = self.raw_data.len();
        &self.raw_data[largo - 4..largo]
    }
}

/// Formatea un slice de 6 bytes en una cadena legible en formato "XX:XX:XX:XX:XX:XX".
///
/// # Ejemplos
///
/// ```
/// use analizador_red::formatear_mac;
///
/// let mac_bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
/// assert_eq!(formatear_mac(&mac_bytes), "FF:FF:FF:FF:FF:FF");
/// ```
pub fn formatear_mac(mac: &[u8]) -> String {
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER_TEST: [u8; 32] = [
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAB,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Destino
        0x00, 0x15, 0x5D, 0x01, 0x1A, 0x02, // Origen
        0x08, 0x00,                         // IPv4
        0x48, 0x6F, 0x6C, 0x61, 0x21, 0x21, // Payload
        0xDE, 0xAD, 0xBE, 0xEF,             // CRC
    ];

    #[test]
    fn test_formatear_mac() {
        let mac = [0x00, 0x15, 0x5D, 0x01, 0x1A, 0x02];
        assert_eq!(formatear_mac(&mac), "00:15:5D:01:1A:02");
    }

    #[test]
    fn test_direcciones_mac() {
        let vista = VistaTramaEthernet::registrar(&BUFFER_TEST);
        assert_eq!(formatear_mac(vista.direccion_destino()), "FF:FF:FF:FF:FF:FF");
        assert_eq!(formatear_mac(vista.direccion_origen()), "00:15:5D:01:1A:02");
    }

    #[test]
    fn test_tipo_protocolo() {
        let vista = VistaTramaEthernet::registrar(&BUFFER_TEST);
        assert_eq!(vista.tipo_protocolo(), 0x0800);
    }

    #[test]
    fn test_payload_y_crc() {
        let vista = VistaTramaEthernet::registrar(&BUFFER_TEST);
        assert_eq!(vista.payload(), b"Hola!!");
        assert_eq!(vista.crc(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    }
}
