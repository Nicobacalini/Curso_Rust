/// ==============================================================================
/// EJERCICIO 8: "Generador de Mazo Aleatorio sin Repeticiones"
///
/// Objetivo:
/// Simular la creación de una baraja española completa (48 cartas) mediante la
/// generación de cartas al azar, garantizando que no existan duplicados.
///
/// Consigna:
/// 1. Definir los tipos de datos para representar una carta:
///    - Palo: Espada, Basto, Oro, Copa.
///    - Carta: número (1 al 12) y palo.
/// 2. Implementar la función `registrar_carta`:
///    - Recibe una referencia mutable al mazo (`Vec<Carta>`) y una nueva carta.
///    - Si la carta NO existe en el mazo, la agrega y devuelve `true`.
///    - Si ya existe, no la agrega y devuelve `false`.
/// 3. Generar cartas aleatorias dentro de un bucle hasta completar el mazo (48 cartas).
/// 4. Contabilizar cuántos intentos/extracciones tomó completar la baraja.
/// 5. Escribir tests unitarios que verifiquen:
///    - El registro exitoso de una carta nueva.
///    - El rechazo de una carta repetida sin alterar el tamaño del mazo.
///    - Que el mazo generado al azar tenga exactamente 48 cartas únicas.
/// ==============================================================================
use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Palo {
    Espada,
    Basto,
    Oro,
    Copa,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Carta {
    pub numero: u8,
    pub palo: Palo,
}

pub fn registrar_carta(mazo: &mut Vec<Carta>, nueva: Carta) -> bool {
    if mazo.contains(&nueva) {
        false
    } else {
        mazo.push(nueva);
        true
    }
}

fn carta_aleatoria<R: Rng>(rng: &mut R) -> Carta {
    let palos = [Palo::Espada, Palo::Basto, Palo::Oro, Palo::Copa];
    Carta {
        numero: rng.gen_range(1..=12),
        palo: palos[rng.gen_range(0..4)],
    }
}

fn main() {
    let mut mazo: Vec<Carta> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut intentos = 0;

    while mazo.len() < 48 {
        intentos += 1;
        let nueva = carta_aleatoria(&mut rng);

        if registrar_carta(&mut mazo, nueva) {
            println!("+ Carta agregada: {:?} de {:?}", nueva.numero, nueva.palo);
        } else {
            println!("x Repetida: {:?} de {:?}", nueva.numero, nueva.palo);
        }
    }

    println!("\nMazo completado en {} intentos", intentos);
    println!("Total cartas unicas: {}", mazo.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agrega_carta_nueva() {
        let mut mazo = Vec::new();
        let carta = Carta { numero: 1, palo: Palo::Espada };

        assert!(registrar_carta(&mut mazo, carta));
        assert_eq!(mazo.len(), 1);
        assert_eq!(mazo[0], carta);
    }

    #[test]
    fn no_agrega_repetidas() {
        let mut mazo = Vec::new();
        let carta = Carta { numero: 7, palo: Palo::Oro };

        assert!(registrar_carta(&mut mazo, carta));
        assert!(!registrar_carta(&mut mazo, carta));
        assert_eq!(mazo.len(), 1);
    }

    #[test]
    fn mazo_aleatorio_no_tiene_duplicados() {
        let mut mazo = Vec::new();
        let mut rng = rand::thread_rng();

        while mazo.len() < 48 {
            let carta = carta_aleatoria(&mut rng);
            registrar_carta(&mut mazo, carta);
        }

        // Verificamos que las 48 sean distintas
        for i in 0..mazo.len() {
            for j in (i + 1)..mazo.len() {
                assert_ne!(mazo[i], mazo[j], "Se encontro una carta");
            }
        }
    }
}