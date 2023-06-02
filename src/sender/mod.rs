pub mod excel;
pub mod pdf;

pub trait HasMutableSender<D: Sender> {
    fn sender(&mut self) -> &mut D;
}

pub trait Sender {
    fn enviar_reporte(&mut self);
    fn convert(&self);
    fn is_enviado(&self) -> Option<String>;
}
