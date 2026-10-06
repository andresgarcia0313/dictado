//! Audio previo al reconocedor: pasa-altos 80 Hz -> analisis de entorno (VAD) -> compuerta espectral -> AGC.
mod agc;
mod compuerta;
mod cupo;
mod entorno;
mod espectro_ruido;
mod histeresis;
mod limitador;
mod medida;
mod mejora;
mod pasaaltos;
#[cfg(test)]
mod pruebas;
#[cfg(test)]
mod pruebas_mejora;
mod ruido;
mod vad;

pub use cupo::Cupo;
pub use histeresis::{Cambio, Histeresis};
pub use mejora::Mejora;
#[cfg(test)]
pub use vad::{Vad, TRAMA};
