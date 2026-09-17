# Router de Eventos en Rust — Modulo 3

Este proyecto corresponde al desarrollo practico del **Modulo 3** del curso de Rust. Su proposito es implementar un procesador de transacciones de dominio utilizando Newtypes (`TransaccionId`, `ClienteId`), metadatos por defecto con el trait **`Default`**, enumeraciones complejas (`Transaccion`) y coincidencia de patrones (*pattern matching*) avanzada con guarda de patrones (*match guards*).

---

## Objetivo del Proyecto

Demostrar la organizacion de una libreria modular con tipos de dominio seguros, propagacion de errores con el operador **`?`** y evaluacion exhaustiva de casos de pago, reembolso y bloqueo de cuentas.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`TransaccionId` / `ClienteId`** | `struct TransaccionId(pub u64)` | **Newtype Pattern** | Tipado fuerte que evita confundir IDs numericos. |
| **`MetadataTransaccion`** | `struct { prioridad: u8, timestamp: u64, reintentos: u8 }` | **Trait `Default`** | Proporciona valores iniciales predeterminados para metadatos. |
| **`Transaccion`** | `enum { Pago, Reembolso, BloqueoCuenta }` | **Variantes de Dominio** | Enumeracion compuesta con datos embebidos y campos nombrados. |
| **`procesar_transaccion`** | `fn(Transaccion) -> Result<String, String>` | **Pattern Matching Avanzado** | Evalua condiciones de monto, auditoria y estado retornando un `Result`. |

---

## Glosario de Conceptos Tecnicos

* **Newtype Pattern:** Encapsulamiento de tipos primitivos en tuplas de un solo elemento para garantizar seguridad de tipos en tiempo de compilacion.
* **Trait `Default`:** Rasgo estandar de Rust para la construccion de instancias con valores por defecto.
* **Operador `?`:** Propagacion concisa de errores `Result::Err` retornando inmediatamente si ocurre un fallo.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
=== Procesando Lote Valido con Operador ? ===
OK: Pago de 250.00 [ID: 101] procesado exitosamente para el cliente 1
OK: Pago de alto valor (15000.00) [ID: 102] requiere auditoria obligatoria para el cliente 2
OK: Reembolso [ID: 103] procesado con motivo: Devolucion por defecto de fabrica
OK: Cuenta del cliente 2 bloqueada exitosamente
--> Todo el lote fue procesado con exito.

=== Demostracion de Interrupcion Anticipada con ? ===
OK Transaccion previa exitosa: Pago de 100.00 [ID: 201] procesado exitosamente para el cliente 5
Se capturo un error encadenado correctamente: 'Monto invalido'
```

---

## Comandos del Proyecto

* **Ejecutar el programa principal:**
  ```bash
  cargo run
  ```

* **Ejecutar los Tests Unitarios:**
  ```bash
  cargo test
  ```

* **Generar y abrir la documentacion tecnica en HTML:**
  ```bash
  cargo doc --open
  ```
