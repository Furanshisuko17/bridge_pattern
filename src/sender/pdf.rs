use super::Sender;

#[derive(Clone)]
pub struct PdfSender {
    email: String,
    enviado: bool,
}

impl PdfSender {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
            enviado: false,
        }
    }
}

impl Sender for PdfSender {
    fn convertir(&self) {
        println!("Reporte convertido a PDF")
    }

    fn enviar_reporte(&mut self) {
        self.convertir();
        println!("Reporte PDF enviado a: {}.", self.email);
        self.enviado = true;
    }

    fn is_enviado(&self) -> Option<String> {
        match self.enviado {
            true => Some(self.email.clone()),
            false => None,
        }
    }
}
