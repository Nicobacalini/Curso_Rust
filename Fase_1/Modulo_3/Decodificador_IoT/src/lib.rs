//! # Decodificador IoT
//!
//! Modulo para la decodificacion y procesamiento de mensajes de red IoT
//! utilizando enumeraciones compuestas (`enum`), guarda de patrones (*match guards*)
//! y vinculacion de patrones (*@ binding*).

/// Nivel de prioridad para comandos IoT.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Prioridad {
    Baja,
    Media,
    Alta,
}

/// Variantes de mensajes soportados por el protocolo IoT.
#[derive(Debug, PartialEq, Clone)]
pub enum MensajeIot {
    /// Ping sin datos para verificacion de latencia.
    Ping,
    /// Telemetria de sensor con identificador (u16) y lectura numerica (f32).
    Telemetria(u16, f32),
    /// Comando con identificador de instruccion (u8) y nivel de prioridad.
    Comando(u8, Prioridad),
}

/// Broker para el conteo y procesamiento de tramas IoT.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct BrokerIot {
    pub mensajes_procesados: u32,
}

impl BrokerIot {
    /// Inicializa un nuevo broker con contador a cero.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Decodificador_IoT::BrokerIot;
    ///
    /// let broker = BrokerIot::nuevo();
    /// assert_eq!(broker.mensajes_procesados, 0);
    /// ```
    pub fn nuevo() -> Self {
        Self {
            mensajes_procesados: 0,
        }
    }

    /// Decodifica y procesa cualquier trama entrante usando coincidencia de patrones exhaustiva.
    /// Retorna la cadena resultante del procesamiento.
    pub fn procesar_mensaje(&mut self, mensaje: MensajeIot) -> String {
        self.mensajes_procesados += 1;

        match mensaje {
            MensajeIot::Ping => {
                let text = String::from("Ping de latencia recibido");
                println!("{}", text);
                text
            }
            MensajeIot::Telemetria(id, lectura) if lectura > 80.0 => {
                let text = format!("Alerta critica de sobrecalentamiento: sensor {}", id);
                println!("{}", text);
                text
            }
            MensajeIot::Telemetria(id, lectura) => {
                let text = format!("Telemetria normal: sensor {}, lectura {}", id, lectura);
                println!("{}", text);
                text
            }
            MensajeIot::Comando(id, prio @ Prioridad::Alta) => {
                let text = format!("Alerta de ejecucion inmediata (prioridad {:?}): comando {}", prio, id);
                println!("{}", text);
                text
            }
            MensajeIot::Comando(id, prioridad) => {
                let text = format!("Comando en cola: comando {}, prioridad {:?}", id, prioridad);
                println!("{}", text);
                text
            }
        }
    }

    /// Filtro rapido para registrar pings usando `if let`.
    pub fn registrar_latencia(&self, mensaje: &MensajeIot) -> bool {
        if let MensajeIot::Ping = mensaje {
            println!("Registrando latencia de red...");
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let mut broker = BrokerIot::nuevo();
        let msg = MensajeIot::Ping;
        assert!(broker.registrar_latencia(&msg));
        let res = broker.procesar_mensaje(msg);
        assert_eq!(res, "Ping de latencia recibido");
        assert_eq!(broker.mensajes_procesados, 1);
    }

    #[test]
    fn test_telemetria_normal() {
        let mut broker = BrokerIot::nuevo();
        let res = broker.procesar_mensaje(MensajeIot::Telemetria(101, 24.5));
        assert_eq!(res, "Telemetria normal: sensor 101, lectura 24.5");
    }

    #[test]
    fn test_telemetria_critica_match_guard() {
        let mut broker = BrokerIot::nuevo();
        let res = broker.procesar_mensaje(MensajeIot::Telemetria(102, 95.0));
        assert_eq!(res, "Alerta critica de sobrecalentamiento: sensor 102");
    }

    #[test]
    fn test_comando_alta_prioridad_binding() {
        let mut broker = BrokerIot::nuevo();
        let res = broker.procesar_mensaje(MensajeIot::Comando(4, Prioridad::Alta));
        assert_eq!(res, "Alerta de ejecucion inmediata (prioridad Alta): comando 4");
    }

    #[test]
    fn test_comando_prioridad_media() {
        let mut broker = BrokerIot::nuevo();
        let res = broker.procesar_mensaje(MensajeIot::Comando(5, Prioridad::Media));
        assert_eq!(res, "Comando en cola: comando 5, prioridad Media");
    }
}
