# Gestor de Memoria Virtual en Rust — Modulo 3

Este proyecto corresponde al desarrollo practico del **Modulo 3** del curso de Rust. Su proposito es implementar un simulador de gestion de memoria RAM utilizando enumeraciones compuestas (`EstadoBloque`), manejo de errores personalizados con `Result<T, ErrorMemoria>` y busqueda de bloques mediante la opcion de retorno `Option<usize>`.

---

## Objetivo del Proyecto

Demostrar la representacion de estados fisicos de memoria (Kernel, Libre, Asignado a PID), validacion de rangos e interrupcion de intentos de escritura invalida mediante coincidencias de patrones (*pattern matching*) y construccion de errores especificos.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`EstadoBloque`** | `enum { Libre, Asignado { pid, solo_lectura }, ReservadoKernel }` | **Enum con datos embebidos** | Representa la condicion de cada bloque de la RAM. |
| **`ErrorMemoria`** | `enum { BloqueProtegidoKernel, BloqueOcupado, IndiceFueraDeRango }` | **Errores Personalizados** | Define los tipos de fallo de hardware o limites. |
| **`MemoriaVirtual`** | `struct { bloques: [EstadoBloque; 8] }` | **Arreglo en RAM** | Contiene el mapa completo de memoria fisica. |
| **`buscar_bloque_libre`** | `fn(&self) -> Option<usize>` | **Uso de `Option`** | Devuelve el primer indice libre o `None` si la RAM esta llena. |
| **`asignar_bloque`** | `fn(&mut self, usize, u32, bool) -> Result<(), ErrorMemoria>` | **Retorno con `Result`** | Modifica el estado del bloque o retorna el error correspondiente. |

---

## Glosario de Conceptos Tecnicos

* **Pattern Matching Exhaustivo:** Garantiza evaluar cada uno de los estados de un bloque de memoria sin dejar casos sin cubrir.
* **Manejo de Errores con `Result<T, E>`:** Patron de Rust para retornar resultados exitosos (`Ok(())`) o fallos especificos (`Err(ErrorMemoria)`).
* **Validacion de Limites:** Previene accesos fuera de rango (*Out of Bounds*) antes de manipular arreglos de tamaño fijo.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
--- Empezando asignacion de memoria ---
Bloque libre encontrado en la posicion fisica: 1
Error esperado al escribir en bloque 0: BloqueProtegidoKernel
Bloque 1 asignado con exito al proceso 99
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
