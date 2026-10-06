//! Modo lote: `--wav a.wav b.wav` con un solo arranque del modelo.
use crate::conexion::{Abrir, Ajustes};
use crate::evento::Evento;
use crate::fuente::Fuente;
use crate::reconocedor::Reconocedor;
use crate::{bucle, wav};
use std::path::PathBuf;
use std::sync::mpsc::channel;

/// Modo lote: un solo arranque del modelo para todos los archivos; cada uno anuncia `archivo` y
/// la salida es el primer código distinto de cero.
pub fn correr(wavs: &[PathBuf], rec: &mut dyn Reconocedor, ajustes: Ajustes) -> i32 {
    let mut codigo = 0;
    for ruta in wavs {
        Evento::Archivo(&ruta.display().to_string()).emitir();
        let ruta = ruta.clone();
        let mut abrir: Box<Abrir> =
            Box::new(move |_| Ok(Box::new(wav::WavFuente::abrir(&ruta)?) as Box<dyn Fuente>));
        let (_tx, rx) = channel(); // sin ordenes: el emisor vivo evita el cierre
        let c = bucle::correr(&mut *abrir, rec, rx, ajustes);
        if codigo == 0 {
            codigo = c;
        }
    }
    codigo
}
