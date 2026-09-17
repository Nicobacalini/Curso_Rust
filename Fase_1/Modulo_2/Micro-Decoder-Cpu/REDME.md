# Micro-Decoder CPU en Rust — Emulador de Arquitectura de 8 Bits

Este proyecto consiste en el diseño e implementación de un emulador de hardware para una **CPU elemental de 8 bits** en Rust. Modela el ciclo fundamental de procesamiento **Fetch-Decode-Execute**, gestión de memoria estática en el *Stack* y ejecución de código máquina mediante decodificación por coincidencia de patrones (*Pattern Matching*).

---

## Objetivo del Proyecto

Comprender y modelar a bajo nivel los principios del diseño de procesadores y arquitectura de computadoras:
1. **Ciclo de Instrucción:** Implementación del ciclo *Fetch* (búsqueda), *Decode* (decodificación) y *Execute* (ejecución).
2. **Manipulación de Registros y Memoria:** Control explícito del **Program Counter (PC)**, el **Registro Acumulador (A)** y un banco de memoria **RAM estática de 16 bytes**.
3. **Seguridad y Robustez en Rust:** Prevención de desbordamientos aritméticos mediante `wrapping_add` y validación estricta de límites tanto en la memoria ROM como en la RAM para evitar accesos fuera de rango.

---

## Conjunto de Instrucciones (Instruction Set Architecture - ISA)

La CPU decodifica un conjunto de instrucciones binarias compactas:

| Opcode | Mnemónico | Bytes | Operando | Descripción |
| :---: | :--- | :---: | :--- | :--- |
| **`0x00`** | **HALT** | 1 | Ninguno | Detiene el ciclo de ejecución y apaga la CPU incrementando el PC en 1. |
| **`0x01`** | **LOAD** | 2 | `[Valor]` (8 bits) | Carga el valor inmediato en el acumulador (`registro_a = valor`). |
| **`0x02`** | **ADD** | 2 | `[Dir RAM]` (8 bits) | Suma el dato ubicado en `RAM[dir]` al acumulador usando suma modular (`wrapping_add`). |
| **`0x03`** | **STORE** | 2 | `[Dir RAM]` (8 bits) | Guarda el valor actual del acumulador en la posición indicada de la RAM. |
| **`_`** | **INV** | - | - | Cualquier código desconocido aborta la ejecución con mensaje de error de seguridad. |

---

## Glosario de Conceptos Técnicos

* **Fetch-Decode-Execute:** Ciclo infinito de hardware donde:
  - **Fetch:** Se lee el byte apuntado por el *Program Counter* (`programa[cpu.pc]`).
  - **Decode:** La estructura `match opcode` clasifica la operación a realizar.
  - **Execute:** Se ejecutan las mutaciones de registros o memoria y se avanza el PC.
* **Program Counter (PC):** Puntero de instrucción que guarda la dirección de memoria de la siguiente instrucción a procesar. En instrucciones de 2 bytes (Opcode + Operando), avanza de 2 en 2 (`cpu.pc += 2`).
* **Registro Acumulador:** Registro principal de la ALU (Unidad Aritmético-Lógica) donde se almacenan temporalmente los datos y los resultados de las operaciones aritméticas.
* **Wrapping Arithmetic (`wrapping_add`):** En aritmética de 8 bits (`u8`), si una suma supera 255 se produce un desbordamiento. En lugar de lanzar un *panic* del sistema, `wrapping_add` emula el comportamiento de hardware real aplicando módulo 256 (`(a + b) % 256`).
* **Zero-Heap Architecture:** Todos los componentes de la CPU (`ram: [u8; 16]`, registros y programa) residen íntegramente en el *Stack*, garantizando máxima velocidad y determinismo sin costo de recolector de basura ni alocador de memoria.

---

## Flujo del Programa de Prueba (ROM)

El programa binario de prueba ejecutado en `main.rs`:

```rust
let programa_rom: [u8; 9] = [
    0x01, 15, // LOAD 15     -> Registro A = 15
    0x03, 5,  // STORE en RAM[5] -> RAM[5] = 15
    0x01, 10, // LOAD 10     -> Registro A = 10
    0x02, 5,  // ADD RAM[5]  -> Registro A = 10 + 15 = 25
    0x00,     // HALT        -> Fin
];
```

**Resultado de depuración en consola:**
```text
=== COMENZANDO EJECUCION DEL PROGRAMA ===
Instruccion: HALT (0x00) detectada. Apagando CPU.
--- Estado de la CPU ---
PC: 9
Registro A: 0x19 (25)
RAM: [00: 0x00] [01: 0x00] [02: 0x00] [03: 0x00] [04: 0x00] [05: 0x0F] ...
```

---

## Comandos del Proyecto

* **Ejecutar el emulador:**
  ```bash
  cargo run
  ```

* **Ejecutar los Tests Unitarios y Doc-Tests:**
  ```bash
  cargo test
  ```

* **Generar y abrir la documentación técnica en HTML:**
  ```bash
  cargo doc --open
  ```
