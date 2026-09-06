//! # Monitoreo de Sensor I2C
//!
//! Modulo para el control de sensores de temperatura por bus I2C y propagacion
//! de errores de hardware utilizando el operador `?` y `Result<T, ErrorI2C>`.

/// Errores fisicos de comunicacion en el bus I2C.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorI2C {
    DispositivoNoResponde,
    RuidoEnLinea,
    ChecksumIncorrecto,
}

/// Driver para la comunicacion con sensores mediante I2C.
#[derive(Debug, PartialEq, Clone)]
pub struct DriverI2C {
    pub voltaje_linea: f32,
    pub sensor_conectado: bool,
}

impl DriverI2C {
    /// Simula un ping fisico al dispositivo en el bus I2C.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Monitoreo_sensor_I2C::{DriverI2C, ErrorI2C};
    ///
    /// let driver = DriverI2C { voltaje_linea: 3.3, sensor_conectado: true };
    /// assert_eq!(driver.ping_dispositivo(), Ok(()));
    /// ```
    pub fn ping_dispositivo(&self) -> Result<(), ErrorI2C> {
        if self.sensor_conectado {
            Ok(())
        } else {
            Err(ErrorI2C::DispositivoNoResponde)
        }
    }

    /// Simula la lectura de la telemetria en crudo del bus fisico.
    pub fn leer_registro_crudo(&self, paso_simulado: u32) -> Result<f32, ErrorI2C> {
        if self.voltaje_linea < 3.0 {
            Err(ErrorI2C::RuidoEnLinea)
        } else {
            let temperatura_cruda = 20.0 + (paso_simulado as f32 * 1.5);
            Ok(temperatura_cruda)
        }
    }

    /// Simula la verificacion matematica de redundancia ciclica (Checksum).
    pub fn validar_checksum(&self, lectura: f32) -> Result<f32, ErrorI2C> {
        if lectura == lectura.round() {
            Err(ErrorI2C::ChecksumIncorrecto)
        } else {
            Ok(lectura)
        }
    }

    /// Intenta leer el sensor completo encadenando los metodos mediante el operador `?`.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Monitoreo_sensor_I2C::{DriverI2C, ErrorI2C};
    ///
    /// let driver = DriverI2C { voltaje_linea: 3.3, sensor_conectado: true };
    /// assert!(driver.leer_temperatura_sensor(1).is_ok());
    /// ```
    pub fn leer_temperatura_sensor(&self, paso: u32) -> Result<f32, ErrorI2C> {
        self.ping_dispositivo()?;
        let lectura = self.leer_registro_crudo(paso)?;
        self.validar_checksum(lectura)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_exitoso() {
        let driver = DriverI2C {
            voltaje_linea: 3.3,
            sensor_conectado: true,
        };
        assert_eq!(driver.ping_dispositivo(), Ok(()));
    }

    #[test]
    fn test_ping_dispositivo_desconectado() {
        let driver = DriverI2C {
            voltaje_linea: 3.3,
            sensor_conectado: false,
        };
        assert_eq!(driver.ping_dispositivo(), Err(ErrorI2C::DispositivoNoResponde));
    }

    #[test]
    fn test_voltaje_bajo_ruido() {
        let driver = DriverI2C {
            voltaje_linea: 2.5,
            sensor_conectado: true,
        };
        assert_eq!(
            driver.leer_temperatura_sensor(1),
            Err(ErrorI2C::RuidoEnLinea)
        );
    }

    #[test]
    fn test_checksum_incorrecto() {
        let driver = DriverI2C {
            voltaje_linea: 3.3,
            sensor_conectado: true,
        };
        // Para paso = 0, temperatura_cruda = 20.0 (numero entero), falla checksum
        assert_eq!(
            driver.leer_temperatura_sensor(0),
            Err(ErrorI2C::ChecksumIncorrecto)
        );
    }

    #[test]
    fn test_lectura_temperatura_exitosa() {
        let driver = DriverI2C {
            voltaje_linea: 3.3,
            sensor_conectado: true,
        };
        // Para paso = 1, temperatura_cruda = 21.5 (no entero), lectura exitosa
        let res = driver.leer_temperatura_sensor(1);
        assert_eq!(res, Ok(21.5));
    }
}
