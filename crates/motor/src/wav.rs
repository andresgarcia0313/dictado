//! Fuente que lee un WAV de 16 kHz, mono, 16 bits, en bloques de 100 ms.
use crate::fuente::{Fuente, Lectura};
use std::path::Path;

pub struct WavFuente {
    muestras: Vec<f32>,
    pos: usize,
    nombre: String,
}

const BLOQUE: usize = 1600;

impl WavFuente {
    pub fn abrir(ruta: &Path) -> Result<Self, String> {
        let mut r = hound::WavReader::open(ruta).map_err(|e| format!("WAV ilegible: {e}"))?;
        let s = r.spec();
        if s.sample_rate != 16_000 || s.channels != 1 || s.bits_per_sample != 16 {
            return Err("el WAV debe ser 16 kHz, mono, 16 bits".into());
        }
        let muestras = r
            .samples::<i16>()
            .map(|m| m.map(|v| v as f32 / 32768.0))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("WAV corrupto: {e}"))?;
        registro::info!(
            "wav abierto ruta=\"{}\" segundos={:.1}",
            ruta.display(),
            muestras.len() as f32 / 16_000.0
        );
        Ok(Self {
            muestras,
            pos: 0,
            nombre: ruta.display().to_string(),
        })
    }
}

impl Fuente for WavFuente {
    fn leer(&mut self) -> Lectura {
        if self.pos >= self.muestras.len() {
            return Lectura::Fin;
        }
        let fin = (self.pos + BLOQUE).min(self.muestras.len());
        let bloque = self.muestras[self.pos..fin].to_vec();
        self.pos = fin;
        Lectura::Muestras(bloque)
    }

    fn dispositivo(&self) -> String {
        self.nombre.clone()
    }

    fn ritmo_ms(&self) -> u64 {
        30
    }
}
