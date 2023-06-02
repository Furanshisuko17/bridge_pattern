use std::collections::HashMap;

use crate::sender::{HasMutableSender, Sender};

use super::ReportGenerator;

pub struct ReporteAvanzado<D: Sender> {
    sender: D,
    nombre_documento: String,
    data: HashMap<i32, String>,
    generado: bool,
}

impl<D: Sender> ReporteAvanzado<D> {
    pub fn new(sender: D, name: &str, data: HashMap<i32, String>) -> Self {
        Self {
            sender,
            nombre_documento: name.to_string(),
            data,
            generado: false,
        }
    }
}

impl<D: Sender> HasMutableSender<D> for ReporteAvanzado<D> {
    fn sender(&mut self) -> &mut D {
        &mut self.sender
    }
}

impl<D: Sender> ReportGenerator<D> for ReporteAvanzado<D> {
    fn generar_reporte(&mut self) {
        if self.generado {
            println!("Reporte avanzado regenerado: {}", self.nombre_documento);
        } else {
            println!("Reporte avanzado generado: {}", self.nombre_documento);
        }
        self.data.iter().for_each(move |x| {
            println!("{}: {}", x.0, x.1);
        });
        self.generado = true;
    }
    fn enviar_reporte(&mut self) {
        if !self.generado {
            self.generar_reporte();
        }
        self.sender.enviar_reporte();
    }

    fn is_enviado(&self) {
        if let Some(email) = self.sender.is_enviado() {
            println!("Reporte enviado a: {email}");
        } else {
            println!("El reporte no fue enviado aún.")
        }
    }
}
