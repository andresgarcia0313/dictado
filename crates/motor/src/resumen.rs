//! Resumen periodico del audio para el registro: cada 5 s (nivel depura).
use std::time::{Duration, Instant};

const CADA: Duration = Duration::from_secs(5);
pub struct Resumen {
    desde: Instant,
    bloques: u32,
    muestras: usize,
    suma_cuadrados: f64,
    pico: f32,
}

impl Resumen {
    pub fn nuevo() -> Self {
        Self {
            desde: Instant::now(),
            bloques: 0,
            muestras: 0,
            suma_cuadrados: 0.0,
            pico: 0.0,
        }
    }

    /// Anota un bloque ya mejorado y la ganancia vigente del AGC (los saltos de ganancia los registra el AGC).
    pub fn anotar(&mut self, mejorado: &[f32], ganancia_db: f32) {
        self.bloques += 1;
        self.muestras += mejorado.len();
        for m in mejorado {
            self.suma_cuadrados += f64::from(m * m);
            self.pico = self.pico.max(m.abs());
        }
        let t = self.desde.elapsed();
        if t >= CADA {
            let rms = (self.suma_cuadrados / self.muestras.max(1) as f64).sqrt();
            registro::depura!(
                "resumen rms={rms:.4} pico={:.3} ganancia_db={ganancia_db:.1} bloques_s={:.1}",
                self.pico,
                f64::from(self.bloques) / t.as_secs_f64()
            );
            *self = Self::nuevo();
        }
    }
}
