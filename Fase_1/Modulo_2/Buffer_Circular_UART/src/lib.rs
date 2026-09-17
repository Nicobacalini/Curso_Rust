//! # Buffer Circular UART
//!
//! Emulacion de un buffer ring de hardware FIFO de tamano fijo con politica
//! de sobrescritura en desbordamiento (*overflow*) y destruccion segura via `Drop`.

/// Capacidad maxima del buffer circular en bytes.
pub const CAPACIDAD: usize = 4;

/// Emulacion de un buffer circular UART de hardware.
#[derive(Debug, PartialEq, Eq)]
pub struct UartRingBuffer {
    /// Arreglo en el stack para almacenamiento de bytes.
    pub memoria: [u8; CAPACIDAD],
    /// Posicion donde se escribira el proximo byte.
    pub indice_escritura: usize,
    /// Posicion desde donde se leera el proximo byte.
    pub indice_lectura: usize,
    /// Cantidad actual de bytes sin leer.
    pub pendientes: usize,
}

impl UartRingBuffer {
    /// Inicializa un buffer circular vacio con todos sus indices en cero.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Buffer_Circular_UART::UartRingBuffer;
    ///
    /// let buffer = UartRingBuffer::nuevo();
    /// assert_eq!(buffer.pendientes, 0);
    /// ```
    pub fn nuevo() -> Self {
        Self {
            memoria: [0; CAPACIDAD],
            indice_escritura: 0,
            indice_lectura: 0,
            pendientes: 0,
        }
    }

    /// Escribe un byte en el buffer.
    /// Si el buffer esta lleno, sobrescribe el dato mas antiguo y notifica el desbordamiento.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Buffer_Circular_UART::UartRingBuffer;
    ///
    /// let mut buffer = UartRingBuffer::nuevo();
    /// buffer.escribir(100);
    /// assert_eq!(buffer.pendientes, 1);
    /// ```
    pub fn escribir(&mut self, byte: u8) {
        self.memoria[self.indice_escritura] = byte;
        self.indice_escritura = (self.indice_escritura + 1) % CAPACIDAD;

        if self.pendientes == CAPACIDAD {
            self.indice_lectura = (self.indice_lectura + 1) % CAPACIDAD;
            println!("WARNING: OVERFLOW! Se ha sobrescrito el dato mas antiguo.");
        } else {
            self.pendientes += 1;
        }
    }

    /// Lee y remueve el byte mas antiguo disponible siguiendo la politica FIFO.
    /// Retorna `Some(byte)` si hay datos disponibles o `None` si esta vacio.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Buffer_Circular_UART::UartRingBuffer;
    ///
    /// let mut buffer = UartRingBuffer::nuevo();
    /// buffer.escribir(42);
    /// assert_eq!(buffer.leer(), Some(42));
    /// assert_eq!(buffer.leer(), None);
    /// ```
    pub fn leer(&mut self) -> Option<u8> {
        if self.pendientes == 0 {
            return None;
        }

        let byte_leido = self.memoria[self.indice_lectura];
        self.indice_lectura = (self.indice_lectura + 1) % CAPACIDAD;
        self.pendientes -= 1;
        Some(byte_leido)
    }
}

impl Drop for UartRingBuffer {
    fn drop(&mut self) {
        if self.pendientes > 0 {
            println!(
                "CRITICAL: El buffer se ha destruido con {} bytes pendientes.",
                self.pendientes
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_nuevo() {
        let buffer = UartRingBuffer::nuevo();
        assert_eq!(buffer.pendientes, 0);
        assert_eq!(buffer.indice_escritura, 0);
        assert_eq!(buffer.indice_lectura, 0);
    }

    #[test]
    fn test_escribir_y_leer_fifo() {
        let mut buffer = UartRingBuffer::nuevo();
        buffer.escribir(10);
        buffer.escribir(20);

        assert_eq!(buffer.leer(), Some(10));
        assert_eq!(buffer.leer(), Some(20));
        assert_eq!(buffer.leer(), None);
    }

    #[test]
    fn test_buffer_vacio_devuelve_none() {
        let mut buffer = UartRingBuffer::nuevo();
        assert_eq!(buffer.leer(), None);
    }

    #[test]
    fn test_overflow_sobrescribe() {
        let mut buffer = UartRingBuffer::nuevo();
        buffer.escribir(1);
        buffer.escribir(2);
        buffer.escribir(3);
        buffer.escribir(4);
        buffer.escribir(5); // Sobrescribe el 1

        assert_eq!(buffer.pendientes, CAPACIDAD);
        assert_eq!(buffer.leer(), Some(2));
    }
}
