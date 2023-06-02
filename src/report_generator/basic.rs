use crate::sender::{HasMutableSender, Sender};

use super::ReportGenerator;

pub struct ReporteBasico<D: Sender> {
    sender: D,
    nombre_documento: String,
    data: Vec<i32>,
    generado: bool,
}

impl<D: Sender> ReporteBasico<D> {
    pub fn new(sender: D, name: &str, data: Vec<i32>) -> Self {
        Self {
            sender,
            nombre_documento: name.to_string(),
            data,
            generado: false,
        }
    }
}

impl<D: Sender> HasMutableSender<D> for ReporteBasico<D> {
    fn sender(&mut self) -> &mut D {
        &mut self.sender
    }
}

impl<D: Sender> ReportGenerator<D> for ReporteBasico<D> {
    fn generar_reporte(&mut self) {
        if self.generado {
            println!("Reporte básico regenerado: {}", self.nombre_documento);
        } else {
            println!("Reporte básico generado: {}", self.nombre_documento);
        }
        self.data.iter().for_each(move |x| {
            println!("{x} ");
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
