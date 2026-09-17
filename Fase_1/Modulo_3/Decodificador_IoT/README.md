# Decodificador IoT en Rust — Modulo 3

Este proyecto corresponde al desarrollo practico del **Modulo 3** del curso de Rust. Su proposito es implementar un broker y decodificador de tramas de red IoT procesando enumeraciones compuestas (`MensajeIot`, `Prioridad`) mediante coincidencias de patrones (*pattern matching*) avanzadas, guardas de patron (*match guards*) y vinculacion de valores (*@ binding*).

---

## Objetivo del Proyecto

Demostrar la decodificacion segura de tramas de telemetria, comandos y pings de red, aplicando guardas condicionales `if lectura > 80.0`, vinculaciones `@` para capturar rangos o variantes especificas y filtrado veloz con `if let`.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`Prioridad`** | `enum { Baja, Media, Alta }` | **Enum Simple** | Representa la prioridad de ejecucion de comandos. |
| **`MensajeIot`** | `enum { Ping, Telemetria(u16, f32), Comando(u8, Prioridad) }` | **Enum Compuesto** | Representa los tipos de mensajes de la red IoT. |
| **`BrokerIot`** | `struct { mensajes_procesados: u32 }` | **Gestor de Mensajes** | Cuenta y procesa el flujo entrante de datos. |
| **`procesar_mensaje`** | `fn(&mut self, MensajeIot) -> String` | **Match Guards & `@` Binding** | Evalua lecturas criticas e instruye alertas de ejecucion inmediata. |
| **`registrar_latencia`** | `fn(&self, &MensajeIot) -> bool` | **Control con `if let`** | Filtra exclusivamente mensajes de tipo `Ping`. |

---

## Glosario de Conceptos Tecnicos

* **Match Guards (`if lectura > 80.0`):** Condicion logica adicional agregada a una rama del `match` para filtrar por rango o valor.
* **Pattern Binding (`prio @ Prioridad::Alta`):** Vincula el valor coincidente a una variable para su uso posterior en la misma rama.
* **Filtrado Conciso `if let`:** Sintaxis reducida para evaluar una sola variante de un `enum` sin necesidad de escribir un `match` completo.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
Registrando latencia de red...
Ping de latencia recibido
Telemetria normal: sensor 101, lectura 24.5
Alerta critica de sobrecalentamiento: sensor 102
Alerta de ejecucion inmediata (prioridad Alta): comando 4
Comando en cola: comando 5, prioridad Media

Total de mensajes procesados: 5
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
