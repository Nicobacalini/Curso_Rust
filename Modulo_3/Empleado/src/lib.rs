//! # Gestion de Empleados
//!
//! Modulo para la representacion de empleados con nombre, salario y departamento opcional
//! utilizando `Option<String>`, `match` y `unwrap_or_else` para manejar valores ausentes.

/// Estructura que representa a un empleado con su nombre, salario y departamento opcional.
#[derive(Debug, Clone, PartialEq)]
pub struct Empleado {
    pub nombre: String,
    pub salario: f64,
    pub departamento: Option<String>,
}

impl Empleado {
    /// Crea un nuevo empleado.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Empleado::Empleado;
    ///
    /// let emp = Empleado::nuevo("Juan", 1500.0, Some("Ventas"));
    /// assert_eq!(emp.salario, 1500.0);
    /// assert_eq!(emp.departamento, Some("Ventas".to_string()));
    /// ```
    pub fn nuevo(nombre: &str, salario: f64, departamento: Option<&str>) -> Self {
        Empleado {
            nombre: nombre.to_string(),
            salario,
            departamento: departamento.map(|d| d.to_string()),
        }
    }

    /// Retorna el nombre del departamento o "Sin asignar" utilizando `unwrap_or_else`.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use Empleado::Empleado;
    ///
    /// let emp = Empleado::nuevo("Maria", 1800.0, None);
    /// assert_eq!(emp.obtener_departamento(), "Sin asignar");
    /// ```
    pub fn obtener_departamento(&self) -> String {
        self.departamento
            .clone()
            .unwrap_or_else(|| String::from("Sin asignar"))
    }

    /// Imprime el departamento del empleado usando `match`.
    pub fn imprimir_departamento(&self) {
        let depto = match &self.departamento {
            Some(dep) => dep.as_str(),
            None => "Sin asignar",
        };
        println!(
            "Empleado: {}, Salario: ${:.2}, Departamento: {}",
            self.nombre, self.salario, depto
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empleado_con_departamento() {
        let emp = Empleado::nuevo("Juan", 1500.0, Some("Ventas"));
        assert_eq!(emp.obtener_departamento(), "Ventas");
    }

    #[test]
    fn test_empleado_sin_departamento() {
        let emp = Empleado::nuevo("Maria", 1800.0, None);
        assert_eq!(emp.obtener_departamento(), "Sin asignar");
    }

    #[test]
    fn test_imprimir_departamento_match() {
        let emp = Empleado::nuevo("Carlos", 2000.0, Some("IT"));
        assert_eq!(emp.nombre, "Carlos");
        assert_eq!(emp.salario, 2000.0);
        assert_eq!(emp.obtener_departamento(), "IT");
    }
}
