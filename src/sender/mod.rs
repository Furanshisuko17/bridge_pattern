pub mod excel;
pub mod pdf;

pub trait Sender {
    fn enviar_reporte(&mut self);
    fn convertir(&self);
    fn is_enviado(&self) -> Option<String>;
}
