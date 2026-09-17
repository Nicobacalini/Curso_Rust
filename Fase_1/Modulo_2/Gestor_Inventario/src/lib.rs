//! # Gestor de Inventario
//!
//! Modulo para la gestion de productos, descuentos, control de stock y busqueda
//! en colecciones mediante conceptos de Ownership, Borrowing, referencias mutables y Option.

/// Crea un String descriptivo del producto tomando posesion (*ownership*) del nombre.
///
/// # Ejemplos
///
/// ```
/// use Gestor_Inventario::crear_producto;
///
/// let nombre = String::from("Teclado");
/// let prod = crear_producto(nombre, 50.0);
/// assert_eq!(prod, "Producto: Teclado, Precio: 50");
/// ```
pub fn crear_producto(nombre: String, precio: f64) -> String {
    format!("Producto: {}, Precio: {}", nombre, precio)
}

/// Aplica un porcentaje de descuento a un precio recibido por referencia inmutable.
///
/// # Ejemplos
///
/// ```
/// use Gestor_Inventario::aplicar_descuento;
///
/// let precio = 100.0;
/// let final_precio = aplicar_descuento(&precio, 0.20);
/// assert_eq!(final_precio, 80.0);
/// ```
pub fn aplicar_descuento(precio: &f64, porcentaje: f64) -> f64 {
    let precio_descuento = *precio - (*precio * porcentaje);
    println!("[APLICAR] Precio con descuento: {}", precio_descuento);
    precio_descuento
}

/// Modifica el valor del stock en memoria mediante una referencia mutable (`&mut`).
///
/// # Ejemplos
///
/// ```
/// use Gestor_Inventario::actualizar_stock;
///
/// let mut stock = 10;
/// actualizar_stock(&mut stock, -3);
/// assert_eq!(stock, 7);
/// ```
pub fn actualizar_stock(stock: &mut i32, cantidad: i32) {
    *stock += cantidad;
    println!("Stock actualizado internamente: {}", *stock);
}

/// Devuelve una referencia opcional al primer precio superior al limite dentro de un slice.
///
/// # Ejemplos
///
/// ```
/// use Gestor_Inventario::primer_producto_caro;
///
/// let precios = [10.0, 60.0, 30.0];
/// assert_eq!(primer_producto_caro(&precios, 50.0), Some(&60.0));
/// ```
pub fn primer_producto_caro(productos: &[f64], limite: f64) -> Option<&f64> {
    for precio in productos {
        if *precio > limite {
            return Some(precio);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_producto() {
        let res = crear_producto(String::from("Mouse"), 25.0);
        assert_eq!(res, "Producto: Mouse, Precio: 25");
    }

    #[test]
    fn test_aplicar_descuento() {
        let precio = 80.0;
        let resultado = aplicar_descuento(&precio, 0.10);
        assert_eq!(resultado, 72.0);
    }

    #[test]
    fn test_actualizar_stock() {
        let mut stock = 20;
        actualizar_stock(&mut stock, 5);
        assert_eq!(stock, 25);
        actualizar_stock(&mut stock, -10);
        assert_eq!(stock, 15);
    }

    #[test]
    fn test_primer_producto_caro_encontrado() {
        let lista = [25.5, 80.0, 120.0];
        assert_eq!(primer_producto_caro(&lista, 50.0), Some(&80.0));
    }

    #[test]
    fn test_primer_producto_caro_no_encontrado() {
        let lista = [10.0, 20.0, 30.0];
        assert_eq!(primer_producto_caro(&lista, 50.0), None);
    }
}
