
///### Ejercicio 1: "Simulador de Submarino" (Módulo 1 & 2)
///*Habilidades: Arrays, tuplas, mutabilidad, referencias mutables, casting.*

///#### Descripción
///Un submarino autónomo recibe una lista de comandos de navegación en un array fijo de pares `(&str, i32)`. Las direcciones posibles son:
///- `"forward"`: aumenta la posición horizontal en $X$.
///- `"down"`: aumenta la profundidad en $X$.
///- `"up"`: disminuye la profundidad en $X$. (La profundidad nunca puede ser menor que 0; si una orden la hiciera negativa, se satura en 0).

///Implementa la función:
///```rust
///pub fn simular_submarino(comandos: &[(&str, i32)]) -> (i32, i32);



fn simular_submarino(comandos: &[(&str, i32)]) -> (i32, i32){
    let mut horizontal = 0;
    let mut profundidad = 0;

    for &(direccion, valor) in comandos{
        match direccion {
            "forward" => horizontal += valor,
            "down" => profundidad += valor,
            "up" => profundidad = (profundidad - valor).max(0),
            _ => (),
        }
        
    }
    (horizontal, profundidad)
}

fn main() {
    let ruta = [("forward", 5), ("down", 10), ("forward", 3), ("up", 4), ("up", 10)];
    let (h, p) = simular_submarino(&ruta);
    println!("Posicion: horizontal={}, profundidad={}", h, p);
}

#[test]
fn test_submarino() {
    let ruta = [("forward", 5), ("down", 10), ("forward", 3), ("up", 4), ("up", 10)];
    assert_eq!(simular_submarino(&ruta), (8, 0));

    
}