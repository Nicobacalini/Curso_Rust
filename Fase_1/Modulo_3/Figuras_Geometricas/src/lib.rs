//! # Figuras Geometricas
//!
//! Modulo para el calculo de perimetros y comparacion de figuras geometricas
//! mediante enumeraciones (`enum`), coincidencia de patrones (`match`) y referencias prestadas (`&[T]`).

/// Representa las distintas formas geometricas soportadas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Figura {
    Circulo(f64),
    Rectangulo(f64, f64),
    Cuadrado(f64),
}

impl Figura {
    /// Calcula el perimetro de la figura segun su variante.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Figuras_Geometricas::Figura;
    ///
    /// let cuadrado = Figura::Cuadrado(4.0);
    /// assert_eq!(cuadrado.perimetro(), 16.0);
    /// ```
    pub fn perimetro(&self) -> f64 {
        match self {
            Figura::Circulo(radio) => 2.0 * std::f64::consts::PI * radio,
            Figura::Rectangulo(base, altura) => 2.0 * (base + altura),
            Figura::Cuadrado(lado) => 4.0 * lado,
        }
    }

    /// Devuelve una referencia opcional a la figura con mayor perimetro dentro de un slice.
    /// Retorna `None` si la lista de figuras esta vacia.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Figuras_Geometricas::Figura;
    ///
    /// let c = Figura::Circulo(1.0);
    /// let r = Figura::Rectangulo(10.0, 10.0);
    /// let figuras = [c, r];
    ///
    /// assert_eq!(Figura::figura_mas_grande(&figuras), Some(&r));
    /// ```
    pub fn figura_mas_grande(figuras: &[Figura]) -> Option<&Figura> {
        if figuras.is_empty() {
            return None;
        }
        let mut mas_grande = &figuras[0];
        for figura in figuras {
            if figura.perimetro() > mas_grande.perimetro() {
                mas_grande = figura;
            }
        }
        Some(mas_grande)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perimetro_cuadrado() {
        let c = Figura::Cuadrado(3.0);
        assert_eq!(c.perimetro(), 12.0);
    }

    #[test]
    fn test_perimetro_rectangulo() {
        let r = Figura::Rectangulo(4.0, 6.0);
        assert_eq!(r.perimetro(), 20.0);
    }

    #[test]
    fn test_perimetro_circulo() {
        let circulo = Figura::Circulo(5.0);
        let esperado = 2.0 * std::f64::consts::PI * 5.0;
        assert!((circulo.perimetro() - esperado).abs() < 1e-6);
    }

    #[test]
    fn test_figura_mas_grande() {
        let circulo = Figura::Circulo(5.0); // perimetro ~ 31.415
        let rectangulo = Figura::Rectangulo(4.0, 6.0); // perimetro 20.0
        let cuadrado = Figura::Cuadrado(3.0); // perimetro 12.0

        let figuras = [circulo, rectangulo, cuadrado];
        let mas_grande = Figura::figura_mas_grande(&figuras);

        assert_eq!(mas_grande, Some(&circulo));
    }

    #[test]
    fn test_figura_mas_grande_vacio() {
        let figuras: [Figura; 0] = [];
        assert_eq!(Figura::figura_mas_grande(&figuras), None);
    }
}
