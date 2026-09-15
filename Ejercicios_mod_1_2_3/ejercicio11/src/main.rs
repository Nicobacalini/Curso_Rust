// ==============================================================================
// EJERCICIO 11: "Canasta con Trait Drop y Rollback Automatico"
//
// Modela un sistema transaccional donde una reserva descuenta stock temporalmente.
// Si la reserva no se confirma explicitamente antes de morir (salir de scope),
// el trait Drop debe devolver el stock al inventario original de forma automatica.
//
// Requerimientos:
// 1. ReservaTemporal::crear(inventario, cantidad):
//    - Si inventario.stock_disponible < cantidad -> Err("Stock insuficiente")
//    - Si alcanza -> resta la cantidad de inventario.stock_disponible y retorna
//      Ok(Self { inventario, cantidad, confirmada: false })
//
// 2. ReservaTemporal::confirmar(mut self):
//    - Marca self.confirmada = true
//    - Suma +1 a self.inventario.operaciones_exitosas
//
// 3. Trait Drop para ReservaTemporal:
//    - Si confirmada == false -> devuelve la cantidad a self.inventario.stock_disponible
//    - Si confirmada == true -> no hace nada
// ==============================================================================

pub struct Inventario {
    pub stock_disponible: u32,
    pub operaciones_exitosas: u32,
}

pub struct ReservaTemporal<'a> {
    pub inventario: &'a mut Inventario,
    pub cantidad: u32,
    pub confirmada: bool,
}

impl<'a> ReservaTemporal<'a> {
    pub fn crear(inventario: &'a mut Inventario, cantidad: u32) -> Result<Self, &'static str> {
        if inventario.stock_disponible < cantidad{
            return Err("Stock insuficiente");
        }
        inventario.stock_disponible -= cantidad;
        Ok(Self {
            inventario,
            cantidad,
            confirmada: false,
        })
    }

    pub fn confirmar(mut self) {
        self.confirmada = true;
        self.inventario.operaciones_exitosas += 1;
    }
}

impl<'a> Drop for ReservaTemporal<'a> {
    fn drop(&mut self) {
        if !self.confirmada{
            self.inventario.stock_disponible += self.cantidad
        }
    }
}

fn main() {
    let mut inv = Inventario {
        stock_disponible: 50,
        operaciones_exitosas: 0,
    };

    println!("Stock inicial: {}", inv.stock_disponible);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaccion_rollback_automatico() {
        let mut inv = Inventario {
            stock_disponible: 50,
            operaciones_exitosas: 0,
        };

        // Escenario 1: Salir de scope sin confirmar provoca Rollback
        {
            let res = ReservaTemporal::crear(&mut inv, 20).unwrap();
            assert_eq!(res.inventario.stock_disponible, 30);
            // res sale de scope aca sin confirmar()
        }

        // El stock debio recuperarse a 50
        assert_eq!(inv.stock_disponible, 50);
        assert_eq!(inv.operaciones_exitosas, 0);

        // Escenario 2: Confirmar la reserva evita el Rollback
        {
            let res = ReservaTemporal::crear(&mut inv, 20).unwrap();
            res.confirmar();
        }

        assert_eq!(inv.stock_disponible, 30);
        assert_eq!(inv.operaciones_exitosas, 1);

        // Escenario 3: Stock insuficiente
        let fallo = ReservaTemporal::crear(&mut inv, 100);
        assert_eq!(fallo.is_err(), true);
        assert_eq!(inv.stock_disponible, 30);
    }
}