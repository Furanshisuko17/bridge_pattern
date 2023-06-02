use std::collections::HashMap;

use report_generator::{basic::ReporteBasico, ReportGenerator};
use sender::excel::ExcelSender;

use crate::{
    report_generator::advanced::ReporteAvanzado,
    sender::pdf::{PdfSender},
};

mod report_generator;
mod sender;

fn main() {
    let datos_avanzado = HashMap::from([
        (4, "Mercury".to_owned()),
        (7, "Venus".to_owned()),
        (1, "Earth".to_owned()),
        (5, "Mars".to_owned()),
    ]);

    let datos_basico = Vec::from([1, 30, 400, 22, 3, 4]);

    let pdf_sender = PdfSender::new("pdfemail@utp.com");
    let excel_sender = ExcelSender::new("excelemail@utp.com");

    let mut reporte_avanzado = ReporteAvanzado::new(
        excel_sender.clone(),
        "Mi reporte avanzado",
        datos_avanzado.clone(),
    );

    let mut reporte_avanzado_2 =
        ReporteAvanzado::new(pdf_sender.clone(), "Mi reporte avanzado", datos_avanzado);

    let mut reporte_basico = ReporteBasico::new(
        excel_sender.clone(),
        "Mi reporte básico",
        datos_basico.clone(),
    );

    let mut reporte_basico_2 =
        ReporteBasico::new(pdf_sender.clone(), "Mi reporte básico", datos_basico);

    reporte_avanzado.enviar_reporte();
    println!("\n");
    reporte_avanzado_2.enviar_reporte();

    println!("\n----------------------------------------------\n");

    reporte_basico.enviar_reporte();
    println!("\n");
    reporte_basico_2.enviar_reporte();
}
