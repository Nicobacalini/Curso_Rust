//! # Gestor de Memoria Virtual
//!
//! Modulo para la simulacion de un gestor de memoria virtual en RAM con bloques
//! reservados para el kernel, bloques libres y bloques asignados a procesos.

/// Cantidad total de bloques de memoria fisica.
pub const CANTIDAD_BLOQUES: usize = 8;

/// Representa el estado de un bloque de memoria virtual.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EstadoBloque {
    Libre,
    Asignado { pid: u32, solo_lectura: bool },
    ReservadoKernel,
}

/// Errores posibles durante la asignacion o acceso a memoria.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorMemoria {
    BloqueProtegidoKernel,
    BloqueOcupado,
    IndiceFueraDeRango,
}

/// Gestor de memoria RAM representado como un arreglo de bloques.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MemoriaVirtual {
    pub bloques: [EstadoBloque; CANTIDAD_BLOQUES],
}

impl MemoriaVirtual {
    /// Inicializa la memoria con bloques de kernel, asignados y libres.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Gestor_Mem_Virutal::MemoriaVirtual;
    ///
    /// let ram = MemoriaVirtual::inicializar();
    /// assert_eq!(ram.bloques.len(), 8);
    /// ```
    pub fn inicializar() -> Self {
        Self {
            bloques: [
                EstadoBloque::ReservadoKernel,
                EstadoBloque::Libre,
                EstadoBloque::Libre,
                EstadoBloque::Asignado {
                    pid: 42,
                    solo_lectura: true,
                },
                EstadoBloque::Libre,
                EstadoBloque::ReservadoKernel,
                EstadoBloque::Libre,
                EstadoBloque::Libre,
            ],
        }
    }

    /// Busca el primer bloque libre disponible en la memoria RAM.
    /// Retorna `Some(indice)` o `None` si la memoria esta llena.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Gestor_Mem_Virutal::MemoriaVirtual;
    ///
    /// let ram = MemoriaVirtual::inicializar();
    /// assert_eq!(ram.buscar_bloque_libre(), Some(1));
    /// ```
    pub fn buscar_bloque_libre(&self) -> Option<usize> {
        for (indice, bloque) in self.bloques.iter().enumerate() {
            if *bloque == EstadoBloque::Libre {
                return Some(indice);
            }
        }
        None
    }

    /// Intenta asignar un bloque de memoria a un proceso especifico.
    /// Retorna `Result<(), ErrorMemoria>` indicando exito o el error de memoria ocurrido.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Gestor_Mem_Virutal::{MemoriaVirtual, ErrorMemoria};
    ///
    /// let mut ram = MemoriaVirtual::inicializar();
    /// assert_eq!(ram.asignar_bloque(1, 99, false), Ok(()));
    /// assert_eq!(ram.asignar_bloque(0, 99, false), Err(ErrorMemoria::BloqueProtegidoKernel));
    /// ```
    pub fn asignar_bloque(
        &mut self,
        indice: usize,
        pid: u32,
        solo_lectura: bool,
    ) -> Result<(), ErrorMemoria> {
        if indice >= CANTIDAD_BLOQUES {
            return Err(ErrorMemoria::IndiceFueraDeRango);
        }

        match self.bloques[indice] {
            EstadoBloque::Libre => {
                self.bloques[indice] = EstadoBloque::Asignado { pid, solo_lectura };
                Ok(())
            }
            EstadoBloque::ReservadoKernel => Err(ErrorMemoria::BloqueProtegidoKernel),
            EstadoBloque::Asignado { .. } => Err(ErrorMemoria::BloqueOcupado),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inicializar() {
        let ram = MemoriaVirtual::inicializar();
        assert_eq!(ram.bloques[0], EstadoBloque::ReservadoKernel);
        assert_eq!(ram.bloques[1], EstadoBloque::Libre);
    }

    #[test]
    fn test_buscar_bloque_libre() {
        let ram = MemoriaVirtual::inicializar();
        assert_eq!(ram.buscar_bloque_libre(), Some(1));
    }

    #[test]
    fn test_asignar_bloque_libre() {
        let mut ram = MemoriaVirtual::inicializar();
        assert_eq!(ram.asignar_bloque(1, 99, false), Ok(()));
        assert_eq!(
            ram.bloques[1],
            EstadoBloque::Asignado {
                pid: 99,
                solo_lectura: false
            }
        );
    }

    #[test]
    fn test_asignar_bloque_kernel_error() {
        let mut ram = MemoriaVirtual::inicializar();
        assert_eq!(
            ram.asignar_bloque(0, 99, false),
            Err(ErrorMemoria::BloqueProtegidoKernel)
        );
    }

    #[test]
    fn test_asignar_bloque_ocupado_error() {
        let mut ram = MemoriaVirtual::inicializar();
        assert_eq!(
            ram.asignar_bloque(3, 99, false),
            Err(ErrorMemoria::BloqueOcupado)
        );
    }

    #[test]
    fn test_asignar_bloque_fuera_de_rango_error() {
        let mut ram = MemoriaVirtual::inicializar();
        assert_eq!(
            ram.asignar_bloque(10, 99, false),
            Err(ErrorMemoria::IndiceFueraDeRango)
        );
    }
}
