//! # Micro Decoder CPU
//!
//! Emulacion de una CPU de 8 bits con ciclo Fetch-Decode-Execute,
//! registros de memoria RAM estatica y soporte para opcodes de instruccion.

/// Tamano de la memoria RAM estatica en bytes.
pub const TAMANO_RAM: usize = 16;

/// Estado de la CPU emulada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CPU {
    /// Registro acumulador de 8 bits para operaciones de datos.
    pub registro_a: u8,
    /// Program Counter: puntero a la siguiente instruccion en ROM.
    pub pc: usize,
    /// Memoria RAM estatica para almacenamiento de variables.
    pub ram: [u8; TAMANO_RAM],
}

impl CPU {
    /// Inicializa la CPU en su estado de reinicio con valores en cero.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Micro_Decoder_Cpu::CPU;
    ///
    /// let cpu = CPU::reset();
    /// assert_eq!(cpu.registro_a, 0);
    /// assert_eq!(cpu.pc, 0);
    /// ```
    pub fn reset() -> Self {
        Self {
            registro_a: 0,
            pc: 0,
            ram: [0; TAMANO_RAM],
        }
    }

    /// Muestra el estado actual de los registros y la memoria RAM por consola.
    pub fn volcar_estado(&self) {
        println!("--- Estado de la CPU ---");
        println!("PC: {}", self.pc);
        println!("Registro A: {:#04X} ({})", self.registro_a, self.registro_a);
        print!("RAM: ");
        for (i, byte) in self.ram.iter().enumerate() {
            print!("[{:02X}: {:#04X}] ", i, byte);
        }
        println!("\n------------------------\n");
    }
}

/// Ejecuta un programa binario sobre la CPU mutando su estado interno.
///
/// # Ejemplos
///
/// ```
/// use Micro_Decoder_Cpu::{CPU, ejecutar_programa};
///
/// let mut cpu = CPU::reset();
/// let programa = [0x01, 42, 0x00]; // LOAD 42 -> HALT
/// ejecutar_programa(&mut cpu, &programa);
/// assert_eq!(cpu.registro_a, 42);
/// ```
pub fn ejecutar_programa(cpu: &mut CPU, programa: &[u8]) {
    println!("=== COMENZANDO EJECUCION DEL PROGRAMA ===");

    loop {
        if cpu.pc >= programa.len() {
            println!("ERROR: El Program Counter se salio de los limites de la memoria ROM. Deteniendo.");
            break;
        }

        let opcode = programa[cpu.pc];

        match opcode {
            0x00 => {
                println!("Instruccion: HALT (0x00) detectada. Apagando CPU.");
                cpu.pc += 1;
                break;
            }
            0x01 => {
                if cpu.pc + 1 >= programa.len() {
                    println!("ERROR: No hay parametro para LOAD");
                    break;
                }
                let valor = programa[cpu.pc + 1];
                cpu.registro_a = valor;
                cpu.pc += 2;
            }
            0x02 => {
                if cpu.pc + 1 >= programa.len() {
                    println!("ERROR: No hay parametro para ADD");
                    break;
                }
                let direccion = programa[cpu.pc + 1] as usize;
                if direccion >= TAMANO_RAM {
                    println!("ERROR: Direccion de RAM fuera de rango en ADD: {}", direccion);
                    break;
                }
                let valor = cpu.ram[direccion];
                cpu.registro_a = cpu.registro_a.wrapping_add(valor);
                cpu.pc += 2;
            }
            0x03 => {
                if cpu.pc + 1 >= programa.len() {
                    println!("ERROR: No hay parametro para STORE");
                    break;
                }
                let direccion = programa[cpu.pc + 1] as usize;
                if direccion >= TAMANO_RAM {
                    println!("ERROR: Direccion de RAM fuera de rango en STORE: {}", direccion);
                    break;
                }
                cpu.ram[direccion] = cpu.registro_a;
                cpu.pc += 2;
            }
            _ => {
                println!("ERROR: Opcode desconocido {:#04X} en la posicion {}. Abortando.", opcode, cpu.pc);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_cpu() {
        let cpu = CPU::reset();
        assert_eq!(cpu.registro_a, 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.ram, [0; TAMANO_RAM]);
    }

    #[test]
    fn test_load_instruction() {
        let mut cpu = CPU::reset();
        let programa = [0x01, 15, 0x00];
        ejecutar_programa(&mut cpu, &programa);
        assert_eq!(cpu.registro_a, 15);
    }

    #[test]
    fn test_store_and_add_instructions() {
        let mut cpu = CPU::reset();
        let programa = [
            0x01, 15, // LOAD 15
            0x03, 5,  // STORE RAM[5]
            0x01, 10, // LOAD 10
            0x02, 5,  // ADD RAM[5] (10 + 15 = 25)
            0x00,     // HALT
        ];
        ejecutar_programa(&mut cpu, &programa);
        assert_eq!(cpu.ram[5], 15);
        assert_eq!(cpu.registro_a, 25);
    }

    #[test]
    fn test_opcode_desconocido() {
        let mut cpu = CPU::reset();
        let programa = [0xFF];
        ejecutar_programa(&mut cpu, &programa);
        assert_eq!(cpu.pc, 0);
    }
}
