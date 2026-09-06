# Analizador de Tramas Ethernet II "Zero-Copy" en Rust

Este repositorio contiene un ejercicio práctico diseñado para demostrar el uso de **Zero-Copy memory management** y el control estricto de tiempos de vida (**lifetimes**) en el lenguaje de programación Rust. El proyecto consiste en el diseño conceptual y lógico de un parser de red de alto rendimiento capaz de procesar de forma segura buffers binarios estructurados bajo el estándar Ethernet II.

## Objetivo del Ejercicio

El propósito principal es modelar un analizador que trocee y extraiga metadatos de un flujo de bytes crudos (`&[u8]`) sin realizar alocaciones dinámicas en el *heap* ni duplicar información en memoria. Toda la estructura se apoya en referencias directas sobre el *stack*, garantizando la seguridad en tiempo de compilación a través del *Borrow Checker*.

## Mapa de Memoria de la Trama

El parser procesa el buffer binario mapeando los siguientes rangos de índices fijos y dinámicos:

| Campo | Tamaño | Rango de Índices | Descripción |
| :--- | :---: | :---: | :--- |
| **Preámbulo** | 7 bytes | `0..7` | Sincronización del reloj del receptor (típicamente `0xAA`). |
| **SFD (Comienzo de trama)** | 1 byte | `7` | Indica el inicio inmediato de la trama (`0xAB`). |
| **Dirección Destino (MAC)** | 6 bytes | `8..14` | Dirección MAC del dispositivo receptor. |
| **Dirección Origen (MAC)** | 6 bytes | `14..20` | Dirección MAC del dispositivo transmisor. |
| **Tipo (EtherType)** | 2 bytes | `20..22` | Protocolo de red encapsulado (ej: `0x0800` para IPv4). |
| **Datos (Payload)** | Variable | `22..(N - 4)` | Carga útil transportada por la trama de red. |
| **CRC** | 4 bytes | `(N - 4)..N` | Código de redundancia cíclica para validación de errores. |

## Glosario de Conceptos Técnicos

*   **Zero-Copy:** Técnica de diseño de software que evita la copia de datos de un área de memoria a otra. En este analizador, los métodos de extracción devuelven *slices* (`&[u8]`) que apuntan directamente a las posiciones físicas del buffer original, eliminando la sobrecarga por clonación de datos.
*   **Lifetimes (Tiempos de Vida):** Mecanismo de Rust expresado con la sintaxis `'a` que utiliza el compilador para validar la vigencia de las referencias. Asegura que la estructura del parser no pueda sobrevivir al buffer de bytes real recibido por la interfaz de red, destruyendo los punteros colgados (*dangling pointers*) en tiempo de compilación.
*   **Borrow Checker:** El componente del compilador de Rust que hace cumplir las reglas de pertenencia y préstamo de memoria. Impide mutaciones accidentales, carreras de datos y accesos a memoria liberada.
*   **Network Byte Order (Big-Endian):** Convención estándar en redes de datos que establece que los bytes más significativos de un número se transmiten primero. El campo **Tipo** debe ser leído bajo esta alineación de bytes para convertirse correctamente a un entero nativo.

## Requisitos Conceptuales de la Consigna

1.  **Garantías de Lifetime:** Vincular estructuralmente la vista del analizador al ciclo de vida del arreglo de bytes de entrada.
2.  **Alineación de Endianness:** Convertir el campo **Tipo** mediante la lectura e interpretación Big-Endian de los dos bytes correspondientes.
3.  **Aritmética de Slices Dinámica:** Calcular de forma automática los límites del campo de datos (payload) y el CRC al final del buffer, soportando tamaños de trama variables.
4.  **Formateo Hexadecimal:** Transformar arreglos de bytes crudos en cadenas legibles con formato estandarizado de direcciones MAC (`XX:XX:XX:XX:XX:XX`).
