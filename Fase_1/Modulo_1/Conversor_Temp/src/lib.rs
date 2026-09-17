//! # Conversor de Temperatura
//!
//! Esta biblioteca provee funciones para la conversion entre unidades de temperatura
//! (Celsius y Fahrenheit) y la clasificacion de sensaciones termicas.

/// Convierte una temperatura de grados Celsius a Fahrenheit.
///
/// # Formula
/// `(celsius * 1.8) + 32.0`
///
/// # Ejemplos
///
/// ```
/// use conversor_temp::celsius_a_fahrenheit;
///
/// let f = celsius_a_fahrenheit(0.0);
/// assert_eq!(f, 32.0);
///
/// let f = celsius_a_fahrenheit(100.0);
/// assert_eq!(f, 212.0);
/// ```
pub fn celsius_a_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

/// Convierte una temperatura de grados Fahrenheit a Celsius.
///
/// # Formula
/// `(fahrenheit - 32.0) / 1.8`
///
/// # Ejemplos
///
/// ```
/// use conversor_temp::fahrenheit_a_celsius;
///
/// let c = fahrenheit_a_celsius(32.0);
/// assert_eq!(c, 0.0);
///
/// let c = fahrenheit_a_celsius(212.0);
/// assert_eq!(c, 100.0);
/// ```
pub fn fahrenheit_a_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

/// Clasifica la sensacion termica segun la temperatura en grados Celsius.
///
/// Categorias:
/// - "congelante": si celsius <= 0.0
/// - "templada": si 0.0 < celsius <= 25.0
/// - "calurosa": si celsius > 25.0
///
/// # Ejemplos
///
/// ```
/// use conversor_temp::clasificar_temperatura;
///
/// assert_eq!(clasificar_temperatura(-5.0), "congelante");
/// assert_eq!(clasificar_temperatura(20.0), "templada");
/// assert_eq!(clasificar_temperatura(30.0), "calurosa");
/// ```
pub fn clasificar_temperatura(celsius: f64) -> &'static str {
    if celsius <= 0.0 {
        "congelante"
    } else if celsius <= 25.0 {
        "templada"
    } else {
        "calurosa"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_a_fahrenheit() {
        assert_eq!(celsius_a_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_a_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_fahrenheit_a_celsius() {
        assert_eq!(fahrenheit_a_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_a_celsius(212.0), 100.0);
    }

    #[test]
    fn test_clasificar_temperatura() {
        assert_eq!(clasificar_temperatura(-10.0), "congelante");
        assert_eq!(clasificar_temperatura(0.0), "congelante");
        assert_eq!(clasificar_temperatura(15.0), "templada");
        assert_eq!(clasificar_temperatura(25.0), "templada");
        assert_eq!(clasificar_temperatura(26.0), "calurosa");
    }
}
