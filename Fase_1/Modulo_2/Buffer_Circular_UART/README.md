# Buffer Circular UART en Rust — Modulo 2

Este proyecto corresponde al desarrollo practico del **Modulo 2** del curso de Rust. Su proposito es implementar un buffer circular (*Ring Buffer*) FIFO de tamano fijo en hardware emulado para la recepcion de bytes mediante una interfaz serie UART.

---

## Objetivo del Proyecto

Demostrar la gestion de arreglos estaticos en el stack, aritmetica modular para punteros de lectura y escritura (`% CAPACIDAD`), estrategias de sobrescritura ante desbordamientos y limpieza mediante el trait **`Drop`** (RAII).

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`UartRingBuffer`** | `struct { memoria: [u8; 4], escritura: usize, lectura: usize, pendientes: usize }` | **Stack Buffer FIFO** | Almacena bytes de telemetria en un arreglo circular. |
| **`nuevo`** | `fn() -> UartRingBuffer` | **Constructor por defecto** | Inicializa los punteros e indices a cero. |
| **`escribir`** | `fn(&mut self, u8)` | **Aritmetica modular y Overflow** | Inserta un byte y avanza el puntero de escritura. Sobrescribe si el buffer esta lleno. |
| **`leer`** | `fn(&mut self) -> Option<u8>` | **Desencolado FIFO** | Remueve y devuelve el byte mas antiguo disponible en el buffer. |
| **`Drop`** | `impl Drop for UartRingBuffer` | **Gestion RAII** | Detecta y advierte si el buffer se destruye con datos sin procesar. |

---

## Glosario de Conceptos Tecnicos

* **Ring Buffer / Buffer Circular:** Estructura donde el ultimo elemento conecta con el primero usando el operador modulo (`(i + 1) % CAPACIDAD`).
* **Politica FIFO (First In, First Out):** Los primeros datos en ingresar son los primeros en procesarse.
* **Sobrescritura por Overflow:** Ante desbordamiento de capacidad, el indice de lectura avanza perdiendo el dato mas antiguo para dar lugar al nuevo.
* **Trait `Drop` / RAII:** Garantiza la ejecucion automatica de codigo de limpieza cuando la variable sale de su ambito.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
=== INICIANDO CONTROLADOR UART DE HARDWARE ===

--- Recibiendo 3 bytes de telemetria ---

--- Procesando datos en CPU ---
Dato procesado con exito: 100
Dato procesado con exito: 101

--- Llegada de rafaga de alta velocidad ---
WARNING: OVERFLOW! Se ha sobrescrito el dato mas antiguo.

--- Apagado repentino del modulo ---
Saliendo del ambito del controlador temporal...
CRITICAL: El buffer se ha destruido con 2 bytes pendientes.

--- Fin del programa principal ---
CRITICAL: El buffer se ha destruido con 3 bytes pendientes.
```

---

## Comandos del Proyecto

* **Ejecutar el programa principal:**
  ```bash
  cargo run
  ```

* **Ejecutar los Tests Unitarios y Doc-Tests:**
  ```bash
  cargo test
  ```

* **Generar y abrir la documentacion tecnica en HTML:**
  ```bash
  cargo doc --open
  ```
