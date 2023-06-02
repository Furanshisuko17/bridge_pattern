use crate::sender::{HasMutableSender, Sender};

pub mod advanced;
pub mod basic;

pub trait ReportGenerator<D: Sender>: HasMutableSender<D> {
    fn generar_reporte(&mut self);
    fn enviar_reporte(&mut self);
    fn is_enviado(&self);
}
