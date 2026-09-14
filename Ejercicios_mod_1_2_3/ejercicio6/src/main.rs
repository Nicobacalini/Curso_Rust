// ==============================================================================
// EJERCICIO 6: "Borrow Splitting: Servidor de Batalla por Turnos"
//
// Tienes una estructura que modela una sala de batalla entre dos jugadores.
// Debes implementar el metodo `aplicar_ataque` que:
// 1. Recibe una bandera booleana `de_a_hacia_b`:
//    - Si es true: Jugador A ataca a Jugador B.
//      Se descuenta `danio` de `jugador_b_hp` (sin bajar de 0).
//      Se guarda en el historial: "A ataca a B por X" (donde X es el danio).
//    - Si es false: Jugador B ataca a Jugador A.
//      Se descuenta `danio` de `jugador_a_hp` (sin bajar de 0).
//      Se guarda en el historial: "B ataca a A por X".
//
// Restriccion de Borrow Checker:
// Debes acceder a la vida del jugador afectado mediante una referencia mutable
// y a `self.historial` mediante otra referencia mutable, aprovechando Borrow Splitting.
// ==============================================================================

pub struct SalaBatalla {
    pub jugador_a_hp: i32,
    pub jugador_b_hp: i32,
    pub historial: Vec<String>,
}

impl SalaBatalla {
    pub fn aplicar_ataque(&mut self, de_a_hacia_b: bool, danio: i32) {
        let hp_objetivo = if de_a_hacia_b {
            &mut self.jugador_b_hp
        }
        else{
            &mut self.jugador_a_hp
        };

        let historial_ref = &mut self.historial;
        *hp_objetivo = (*hp_objetivo - danio).max(0);

        // Guardar el evento en el historial
        let mensaje = if de_a_hacia_b{
            format!("A ataca a B por {}", danio)
        }else{
            format!("B ataca a A por {}", danio)
        };
        historial_ref.push(mensaje);
    }

}

fn main() {
    let mut sala = SalaBatalla {
        jugador_a_hp: 100,
        jugador_b_hp: 80,
        historial: Vec::new(),
    };
    println!("HP B: {}", sala.jugador_b_hp);
    sala.aplicar_ataque(true, 25);
    println!("HP B: {}", sala.jugador_b_hp);
    println!("Historial: {:?}", sala.historial);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sala_batalla() {
        let mut sala = SalaBatalla {
            jugador_a_hp: 100,
            jugador_b_hp: 80,
            historial: Vec::new(),
        };

        // A ataca a B
        sala.aplicar_ataque(true, 25);
        assert_eq!(sala.jugador_b_hp, 55);
        assert_eq!(sala.jugador_a_hp, 100);
        assert_eq!(sala.historial.len(), 1);
        assert_eq!(sala.historial[0], "A ataca a B por 25");

        // B ataca a A con danio letal
        sala.aplicar_ataque(false, 150);
        assert_eq!(sala.jugador_a_hp, 0);
        assert_eq!(sala.jugador_b_hp, 55);
        assert_eq!(sala.historial.len(), 2);
        assert_eq!(sala.historial[1], "B ataca a A por 150");
    }
}