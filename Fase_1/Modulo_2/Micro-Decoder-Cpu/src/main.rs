use Micro_Decoder_Cpu::{CPU, ejecutar_programa};

fn main() {
    let mut mi_cpu = CPU::reset();

    // Programa de prueba codificado en binario (ROM):
    // Cargar 15 -> Guardar en RAM[5] -> Cargar 10 -> Sumar RAM[5] (10+15=25) -> HALT
    let programa_rom: [u8; 9] = [
        0x01, 15, // LOAD 15
        0x03, 5,  // STORE en RAM[5]
        0x01, 10, // LOAD 10
        0x02, 5,  // ADD valor en RAM[5]
        0x00,     // HALT
    ];

    // Estado inicial de la CPU
    mi_cpu.volcar_estado();

    // Ejecucion del ciclo de instrucciones
    ejecutar_programa(&mut mi_cpu, &programa_rom);

    // Estado final esperado: Registro A con 25, RAM[5] con 15, PC en 9
    mi_cpu.volcar_estado();
}
