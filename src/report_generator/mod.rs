use crate::sender::{Sender};

pub mod advanced;
pub mod basic;

pub trait HasMutableSender<D: Sender> {
    fn sender(&mut self) -> &mut D;
}

pub trait ReportGenerator<D: Sender>: HasMutableSender<D> {
    fn generar_reporte(&mut self);
    fn enviar_reporte(&mut self);
    fn is_enviado(&mut self);
}
