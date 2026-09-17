use Pool_Conexiones::{Pool, RecursoConexion};

fn main() {
    let mut pool = Pool {
        recursos: vec![
            RecursoConexion {
                id: 1,
                en_uso: false,
            },
            RecursoConexion {
                id: 2,
                en_uso: false,
            },
        ],
        log_eventos: Vec::new(),
    };

    {
        // Adquisicion y auditoria del recurso 0
        let conexion = pool.adquirir_y_auditar(0);

        // Marcado del recurso como activo
        conexion.en_uso = true;
    } // Finaliza el prestamo mutable de conexion al salir del bloque

    // Drop manual del pool para simular liberacion anticipada
    drop(pool);
}
