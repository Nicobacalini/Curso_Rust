//! # Router de Eventos
//!
//! Modulo para el modelado de transacciones, clientes y metadata asociada
//! mediante Newtypes, implementacion del rasgo `Default`, enumeraciones de dominio y
//! procesamiento con pattern matching avanzado.

/// Newtype que representa un identificador unico de transaccion.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TransaccionId(pub u64);

/// Newtype que representa un identificador unico de cliente.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ClienteId(pub u64);

/// Estructura de metadatos para la gestion de transacciones.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MetadataTransaccion {
    pub prioridad: u8,
    pub timestamp: u64,
    pub reintentos: u8,
}

impl Default for MetadataTransaccion {
    fn default() -> Self {
        Self {
            prioridad: 1,
            timestamp: 0,
            reintentos: 3,
        }
    }
}

/// Representa los distintos tipos de transacciones en el dominio.
#[derive(Debug, PartialEq, Clone)]
pub enum Transaccion {
    Pago {
        id: TransaccionId,
        cliente: ClienteId,
        monto: f64,
        meta: MetadataTransaccion,
    },
    Reembolso {
        id: TransaccionId,
        motivo: String,
        monto: f64,
    },
    BloqueoCuenta(ClienteId),
}

/// Procesa una transaccion utilizando pattern matching avanzado.
///
/// - Pagos con monto <= 0.0 retornan `Err("Monto invalido")`.
/// - Pagos de alto valor (monto >= 10_000.0) retornan `Ok` indicando auditoria obligatoria.
/// - Pagos estandar retornan `Ok` con la confirmacion del pago.
/// - Reembolsos retornan `Ok` con el `id` y `motivo`.
/// - Bloqueos de cuenta retornan `Ok` con el `ClienteId`.
pub fn procesar_transaccion(tx: Transaccion) -> Result<String, String> {
    match tx {
        Transaccion::Pago { monto, .. } if monto <= 0.0 => Err(String::from("Monto invalido")),
        Transaccion::Pago { id, cliente, monto: monto @ 10_000.0..=f64::MAX, .. } => {
            Ok(format!(
                "Pago de alto valor ({:.2}) [ID: {}] requiere auditoria obligatoria para el cliente {}",
                monto, id.0, cliente.0
            ))
        }
        Transaccion::Pago { id, cliente, monto, .. } => {
            Ok(format!(
                "Pago de {:.2} [ID: {}] procesado exitosamente para el cliente {}",
                monto, id.0, cliente.0
            ))
        }
        Transaccion::Reembolso { id, motivo, .. } => {
            Ok(format!("Reembolso [ID: {}] procesado con motivo: {}", id.0, motivo))
        }
        Transaccion::BloqueoCuenta(cliente_id) => {
            Ok(format!("Cuenta del cliente {} bloqueada exitosamente", cliente_id.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtypes() {
        let tx_id = TransaccionId(101);
        let cliente_id = ClienteId(505);

        assert_eq!(tx_id.0, 101);
        assert_eq!(cliente_id.0, 505);
    }

    #[test]
    fn test_metadata_default() {
        let meta = MetadataTransaccion::default();

        assert_eq!(meta.prioridad, 1);
        assert_eq!(meta.timestamp, 0);
        assert_eq!(meta.reintentos, 3);
    }

    #[test]
    fn test_procesar_pago_normal() {
        let tx = Transaccion::Pago {
            id: TransaccionId(1),
            cliente: ClienteId(10),
            monto: 500.0,
            meta: MetadataTransaccion::default(),
        };

        let res = procesar_transaccion(tx).unwrap();
        assert!(res.contains("500.00"));
        assert!(res.contains("ID: 1"));
        assert!(res.contains("cliente 10"));
    }

    #[test]
    fn test_procesar_pago_monto_invalido() {
        let tx_negativo = Transaccion::Pago {
            id: TransaccionId(1),
            cliente: ClienteId(10),
            monto: -50.0,
            meta: MetadataTransaccion::default(),
        };

        assert_eq!(procesar_transaccion(tx_negativo), Err(String::from("Monto invalido")));

        let tx_cero = Transaccion::Pago {
            id: TransaccionId(2),
            cliente: ClienteId(10),
            monto: 0.0,
            meta: MetadataTransaccion::default(),
        };

        assert_eq!(procesar_transaccion(tx_cero), Err(String::from("Monto invalido")));
    }

    #[test]
    fn test_procesar_pago_alto_valor() {
        let tx = Transaccion::Pago {
            id: TransaccionId(3),
            cliente: ClienteId(10),
            monto: 15_000.0,
            meta: MetadataTransaccion::default(),
        };

        let res = procesar_transaccion(tx).unwrap();
        assert!(res.contains("auditoria obligatoria"));
        assert!(res.contains("15000.00"));
    }

    #[test]
    fn test_procesar_reembolso() {
        let tx = Transaccion::Reembolso {
            id: TransaccionId(4),
            motivo: String::from("Garantia de satisfaccion"),
            monto: 100.0,
        };

        let res = procesar_transaccion(tx).unwrap();
        assert!(res.contains("ID: 4"));
        assert!(res.contains("Garantia de satisfaccion"));
    }

    #[test]
    fn test_procesar_bloqueo_cuenta() {
        let tx = Transaccion::BloqueoCuenta(ClienteId(99));

        let res = procesar_transaccion(tx).unwrap();
        assert!(res.contains("99"));
    }
}
