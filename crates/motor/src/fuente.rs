//! Fuente de audio: entrega bloques mono a 16 kHz, ya sea del microfono o de un WAV.
pub enum Lectura {
    Muestras(Vec<f32>),
    /// Sin datos aun (el microfono no ha entregado nada en el plazo).
    Vacio,
    /// Fin del flujo (solo WAV).
    Fin,
}

pub trait Fuente {
    fn leer(&mut self) -> Lectura;
    fn dispositivo(&self) -> String;
    /// Pausa entre bloques al leer de archivo (el microfono ya marca su ritmo).
    fn ritmo_ms(&self) -> u64 {
        0
    }
}
