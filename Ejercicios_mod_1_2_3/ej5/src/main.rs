// ==============================================================================
// EJERCICIO 5: "Parser de Logs con Option y Result"
//
// Recibes una linea de log cruda en formato:
// "[NIVEL] timestamp_u64 mensaje"
//
// Ejemplos validos:
// - "[WARN] 1620000000 Disco al 90%"
// - "[ERROR] 1711928391 Falla en conexion"
// - "[INFO] 42 Inicio de sesion"
//
// Debes parsear la linea y retornar un struct `RegistroLog` sin clonar texto,
// propagando errores con `Result` y el operador `?`.
//
// Validaciones estrictas requeridas:
// 1. La linea debe iniciar con '[' y tener un ']' que cierre la cabecera.
//    De lo contrario -> Err("Cabecera invalida")
// 2. El nivel dentro de los corchetes debe ser exactamente:
//    "INFO" -> Nivel::Info
//    "WARN" -> Nivel::Warn
//    "ERROR" -> Nivel::Error
//    Cualquier otro -> Err("Nivel desconocido")
// 3. El timestamp debe ser un u64 valido numericamente.
//    De lo contrario -> Err("Timestamp invalido")
// 4. El mensaje (tras descartar espacios exteriores con .trim()) no puede estar vacio.
//    De lo contrario -> Err("Mensaje vacio")
// ==============================================================================

#[derive (Debug, PartialEq, Eq)]
pub enum Nivel {
    Info,
    Warn,
    Error,
}

#[derive (Debug , PartialEq, Eq)]
pub struct RegistroLog<'a>{
    pub nivel: Nivel,
    pub timestamp: u64,
    pub mensaje: &'a str,
}

pub fn parsear_linea<'a>(linea: &'a str) -> Result<RegistroLog<'a>, &'static str> {
    // Validar el inicio
    if !linea.starts_with('[') {
        return Err("Cabecera invalida");
    }

    // Ubicar el corchete de cierre
    let fin_cabecera = linea.find(']').ok_or("Cabecera invalida")?;

    // Extraer y matchear el nivel
    let nivel_str = &linea[1..fin_cabecera];
    let nivel = match nivel_str {
        "INFO" => Nivel::Info,
        "WARN" => Nivel::Warn,
        "ERROR" => Nivel::Error,
        _ => return Err("Nivel desconocido"),
    };

    // Extraer el timestamp con match tradicional
    let resto = linea[fin_cabecera + 1..].trim_start();
    let fin_timestamp = resto.find(' ').ok_or("Cabecera invalida")?;

    let timestamp = match resto[..fin_timestamp].parse::<u64>() {
        Ok(num) => num,
        Err(_) => return Err("Timestamp invalido"),
    };

    // Extraer y validar el mensaje restante
    let mensaje = resto[fin_timestamp + 1..].trim();
    if mensaje.is_empty() {
        return Err("Mensaje vacio");
    }

    Ok(RegistroLog {
        nivel,
        timestamp,
        mensaje,
    })
}


fn main() {
    let raw = "[WARN] 1620000000 Disco al 90%";
    match parsear_linea(raw) {
        Ok(reg) => println!("Parseado: {:?}, time: {}, msg: '{}'", reg.nivel, reg.timestamp, reg.mensaje),
        Err(e) => println!("Error al parsear: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_logs() {
        let entrada = "[WARN] 1620000000 Disco al 90%";
        let esperado = RegistroLog {
            nivel: Nivel::Warn,
            timestamp: 1620000000,
            mensaje: "Disco al 90%",
        };
        assert_eq!(parsear_linea(entrada), Ok(esperado));

        assert_eq!(parsear_linea("WARN 123 Hola"), Err("Cabecera invalida"));
        assert_eq!(parsear_linea("[DEBUG] 123 Hola"), Err("Nivel desconocido"));
        assert_eq!(parsear_linea("[INFO] abc Hola"), Err("Timestamp invalido"));
        assert_eq!(parsear_linea("[INFO] 123   "), Err("Mensaje vacio"));
    }
}
