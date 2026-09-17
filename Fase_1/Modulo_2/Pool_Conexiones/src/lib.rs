//! # Pool de Conexiones
//!
//! Modulo para la emulacion de un gestor de recursos de red y registro de auditoria,
//! utilizando el trait `Drop` para auditar la eliminacion de recursos en uso.

/// Recurso individual de conexion de red.
#[derive(Debug, PartialEq, Eq)]
pub struct RecursoConexion {
    pub id: u32,
    pub en_uso: bool,
}

impl Drop for RecursoConexion {
    fn drop(&mut self) {
        if self.en_uso {
            println!(
                "ADVERTENCIA: Conexion {} eliminada mientras estaba en uso",
                self.id
            );
        }
    }
}

/// Pool que gestiona la coleccion de conexiones y el historial de auditoria.
#[derive(Debug, Default)]
pub struct Pool {
    pub recursos: Vec<RecursoConexion>,
    pub log_eventos: Vec<String>,
}

impl Pool {
    /// Inicializa un pool vacio de conexiones.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Pool_Conexiones::Pool;
    ///
    /// let pool = Pool::nuevo();
    /// assert_eq!(pool.recursos.len(), 0);
    /// assert_eq!(pool.log_eventos.len(), 0);
    /// ```
    pub fn nuevo() -> Self {
        Self {
            recursos: Vec::new(),
            log_eventos: Vec::new(),
        }
    }

    /// Adquiere una referencia mutable a un recurso e inserta un evento en el log de auditoria.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Pool_Conexiones::{Pool, RecursoConexion};
    ///
    /// let mut pool = Pool::nuevo();
    /// pool.recursos.push(RecursoConexion { id: 1, en_uso: false });
    ///
    /// {
    ///     let conexion = pool.adquirir_y_auditar(0);
    ///     conexion.en_uso = true;
    /// }
    /// assert_eq!(pool.log_eventos.len(), 1);
    /// ```
    pub fn adquirir_y_auditar(&mut self, indice: usize) -> &mut RecursoConexion {
        let recurso = &mut self.recursos[indice];
        self.log_eventos
            .push(format!("Conexion {} adquirida", recurso.id));
        recurso
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_nuevo() {
        let pool = Pool::nuevo();
        assert!(pool.recursos.is_empty());
        assert!(pool.log_eventos.is_empty());
    }

    #[test]
    fn test_adquirir_y_auditar() {
        let mut pool = Pool {
            recursos: vec![
                RecursoConexion { id: 1, en_uso: false },
                RecursoConexion { id: 2, en_uso: false },
            ],
            log_eventos: Vec::new(),
        };

        {
            let conexion = pool.adquirir_y_auditar(0);
            assert_eq!(conexion.id, 1);
            conexion.en_uso = true;
        }

        assert_eq!(pool.log_eventos.len(), 1);
        assert_eq!(pool.log_eventos[0], "Conexion 1 adquirida");
        assert!(pool.recursos[0].en_uso);
    }
}
