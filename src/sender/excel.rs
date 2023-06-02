use super::Sender;

#[derive(Clone)]
pub struct ExcelSender {
    email: String,
    enviado: bool,
}

impl ExcelSender {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
            enviado: false,
        }
    }
}

impl Sender for ExcelSender {
    fn convertir(&self) {
        println!("Reporte convertido a Excel")
    }

    fn enviar_reporte(&mut self) {
        self.convertir();
        println!("Reporte Excel enviado a: {}.", self.email);
        self.enviado = true;
    }

    fn is_enviado(&self) -> Option<String> {
        match self.enviado {
            true => Some(self.email.clone()),
            false => None,
        }
    }
}
