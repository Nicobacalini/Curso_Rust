// ==============================================================================
// EJERCICIO 10: "Firewall de Red con Pattern Matching Avanzado"
//
// Debes implementar una funcion de filtrado de paquetes de red que clasifique
// el trafico entrante evaluando unicamente la referencia prestada del paquete.
//
// Reglas estrictas de jerarquia (de arriba hacia abajo):
// 1. IP de bucle local:
//    Si la IP de origen es exactamente 127.0.0.1
//    -> Accion::Descartar("Loopback no permitido")
//
// 2. Puertos privilegiados pesados:
//    Si el puerto_destino esta en el rango inclusivo 1..=1024 Y tamano_payload > 4096
//    (usar @ binding sobre el puerto y un match guard 'if' para el payload)
//    -> Accion::Auditar
//
// 3. Prioridad de trafico:
//    Si es_prioritario es true
//    -> Accion::EnrutarPrioritario
//
// 4. Proteccion DoS:
//    Si tamano_payload > 65535
//    -> Accion::Descartar("Payload excede MTU")
//
// 5. Trafico estandar:
//    Cualquier otro paquete que no caiga en las reglas anteriores
//    -> Accion::EnrutarNormal
// ==============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Puerto(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpAddress(pub u8, pub u8, pub u8, pub u8);

pub struct Paquete {
    pub origen: IpAddress,
    pub destino: IpAddress,
    pub puerto_destino: Puerto,
    pub tamano_payload: usize,
    pub es_prioritario: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Accion {
    Descartar(&'static str),
    EnrutarPrioritario,
    EnrutarNormal,
    Auditar,
}

pub fn clasificar_paquete(paquete: &Paquete) -> Accion {
    match paquete {
        Paquete { origen: IpAddress(127, 0, 0, 1), .. } => {
            Accion::Descartar("Loopback no permitido")
        }

        Paquete {
            puerto_destino: Puerto(1..=1024),
            tamano_payload,
            ..
        } if *tamano_payload > 4096 => {
            Accion::Auditar
        }

        Paquete { es_prioritario: true, .. } => {
            Accion::EnrutarPrioritario
        }

        Paquete { tamano_payload, .. } if *tamano_payload > 65535 => {
            Accion::Descartar("Payload excede MTU")
        }

        _ => Accion::EnrutarNormal,
    }
}

fn main() {
    let p = Paquete {
        origen: IpAddress(192, 168, 1, 50),
        destino: IpAddress(10, 0, 0, 1),
        puerto_destino: Puerto(80),
        tamano_payload: 5000,
        es_prioritario: false,
    };

    println!("Clasificacion: {:?}", clasificar_paquete(&p));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall() {
        // 1. Loopback bloqueado
        let p1 = Paquete {
            origen: IpAddress(127, 0, 0, 1),
            destino: IpAddress(192, 168, 1, 1),
            puerto_destino: Puerto(80),
            tamano_payload: 100,
            es_prioritario: false,
        };
        assert_eq!(clasificar_paquete(&p1), Accion::Descartar("Loopback no permitido"));

        // 2. Puerto privilegiado con payload pesado -> Auditar
        let p2 = Paquete {
            origen: IpAddress(10, 0, 0, 1),
            destino: IpAddress(192, 168, 1, 1),
            puerto_destino: Puerto(443),
            tamano_payload: 8000,
            es_prioritario: true, // Debe caer en Auditar antes que en prioritario por jerarquia
        };
        assert_eq!(clasificar_paquete(&p2), Accion::Auditar);

        // 3. Paquete prioritario comun
        let p3 = Paquete {
            origen: IpAddress(10, 0, 0, 1),
            destino: IpAddress(192, 168, 1, 1),
            puerto_destino: Puerto(8080),
            tamano_payload: 500,
            es_prioritario: true,
        };
        assert_eq!(clasificar_paquete(&p3), Accion::EnrutarPrioritario);

        // 4. Payload que excede MTU
        let p4 = Paquete {
            origen: IpAddress(10, 0, 0, 1),
            destino: IpAddress(192, 168, 1, 1),
            puerto_destino: Puerto(9000),
            tamano_payload: 70000,
            es_prioritario: false,
        };
        assert_eq!(clasificar_paquete(&p4), Accion::Descartar("Payload excede MTU"));

        // 5. Trafico normal
        let p5 = Paquete {
            origen: IpAddress(10, 0, 0, 1),
            destino: IpAddress(192, 168, 1, 1),
            puerto_destino: Puerto(8080),
            tamano_payload: 1024,
            es_prioritario: false,
        };
        assert_eq!(clasificar_paquete(&p5), Accion::EnrutarNormal);
    }
}