//! # Controlador de Semaforo
//!
//! Modulo para la emulacion de un sistema de control de semaforo de trafico
//! mediante enumeraciones (`enum`), coincidencia de patrones (`match`) y transiciones
//! de estado deterministas.

/// Representa los tres estados posibles de un semaforo de trafico.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}

impl Semaforo {
    /// Devuelve la siguiente luz del semaforo segun la secuencia:
    /// **Rojo -> Verde -> Amarillo -> Rojo**.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Semaforo::Semaforo;
    ///
    /// let semaforo = Semaforo::Rojo;
    /// assert_eq!(semaforo.siguiente(), Semaforo::Verde);
    /// assert_eq!(Semaforo::Verde.siguiente(), Semaforo::Amarillo);
    /// assert_eq!(Semaforo::Amarillo.siguiente(), Semaforo::Rojo);
    /// ```
    pub fn siguiente(&self) -> Semaforo {
        match self {
            Semaforo::Rojo => Semaforo::Verde,
            Semaforo::Verde => Semaforo::Amarillo,
            Semaforo::Amarillo => Semaforo::Rojo,
        }
    }

    /// Devuelve la duracion en segundos del estado activo:
    /// - **Rojo**: 30 segundos
    /// - **Amarillo**: 5 segundos
    /// - **Verde**: 25 segundos
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Semaforo::Semaforo;
    ///
    /// assert_eq!(Semaforo::Rojo.duracion_segundos(), 30);
    /// assert_eq!(Semaforo::Amarillo.duracion_segundos(), 5);
    /// assert_eq!(Semaforo::Verde.duracion_segundos(), 25);
    /// ```
    pub fn duracion_segundos(&self) -> u32 {
        match self {
            Semaforo::Rojo => 30,
            Semaforo::Amarillo => 5,
            Semaforo::Verde => 25,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transiciones_estados() {
        assert_eq!(Semaforo::Rojo.siguiente(), Semaforo::Verde);
        assert_eq!(Semaforo::Verde.siguiente(), Semaforo::Amarillo);
        assert_eq!(Semaforo::Amarillo.siguiente(), Semaforo::Rojo);
    }

    #[test]
    fn test_duracion_segundos() {
        assert_eq!(Semaforo::Rojo.duracion_segundos(), 30);
        assert_eq!(Semaforo::Amarillo.duracion_segundos(), 5);
        assert_eq!(Semaforo::Verde.duracion_segundos(), 25);
    }

    #[test]
    fn test_ciclo_completo() {
        let mut luz = Semaforo::Rojo;
        luz = luz.siguiente(); // Verde
        assert_eq!(luz, Semaforo::Verde);
        luz = luz.siguiente(); // Amarillo
        assert_eq!(luz, Semaforo::Amarillo);
        luz = luz.siguiente(); // Rojo
        assert_eq!(luz, Semaforo::Rojo);
    }
}
