//! Constancia de la salida del motor en el registro.
use crate::fuente::Fuente;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

static REGISTRADA: AtomicBool = AtomicBool::new(false);

/// Deja constancia de la salida UNA sola vez: motivo y codigo (error si el codigo no es 0).
/// El hilo de stdin la llama al recibir `salir`, antes de que el bloque en curso termine.
pub fn registrar(codigo: i32, motivo: &str) -> i32 {
    if !REGISTRADA.swap(true, SeqCst) {
        if codigo == 0 {
            registro::info!("fin codigo=0 motivo={motivo}");
        } else {
            registro::error!("fin codigo={codigo} motivo={motivo}");
        }
    }
    codigo
}

/// Salida por orden `salir`: no cierra el flujo del microfono (soltarlo tarda segundos con ALSA/PipeWire);
/// el sistema lo libera al terminar el proceso.
pub fn salir(fuente: Box<dyn Fuente>) -> i32 {
    std::mem::forget(fuente);
    registrar(0, "orden salir o tuberia cerrada")
}
