//! Estimador espectral del ruido (sobre tramas sin voz) y ganancia suave por bin.
use super::vad::TRAMA;

pub const BINES: usize = TRAMA + 1;
const PROPORCION: f32 = 0.5; // sustraccion suave: nunca mas de 0.7
const PISO: f32 = 0.251; // -12 dB

pub struct EspectroRuido {
    ruido: Vec<f32>,
    ganancia: Vec<f32>,
    vistas: u32,
}

impl EspectroRuido {
    pub fn nuevo() -> Self {
        Self {
            ruido: vec![0.0; BINES],
            ganancia: vec![1.0; BINES],
            vistas: 0,
        }
    }

    pub fn vistas(&self) -> u32 {
        self.vistas
    }

    /// Actualiza el ruido con la potencia de una trama sin voz (baja rapido, sube lento).
    pub fn aprender(&mut self, pot: &[f32]) {
        for (r, &p) in self.ruido.iter_mut().zip(pot) {
            let a = match (self.vistas, p < *r) {
                (0, _) => 1.0,
                (_, true) => 0.3,
                _ => 0.05,
            };
            *r += a * (p - *r);
        }
        self.vistas += 1;
    }

    /// Ganancia por bin, suavizada en frecuencia y en tiempo (abre rapido, cierra suave).
    pub fn ganancias(&mut self, pot: &[f32]) -> &[f32] {
        let g: Vec<f32> = pot
            .iter()
            .zip(&self.ruido)
            .map(|(&p, &r)| (1.0 - PROPORCION * (r / p.max(1e-12)).sqrt()).clamp(PISO, 1.0))
            .collect();
        for k in 0..BINES {
            let (a, z) = (g[k.saturating_sub(1)], g[(k + 1).min(BINES - 1)]);
            let objetivo = 0.25 * a + 0.5 * g[k] + 0.25 * z;
            let paso = if objetivo > self.ganancia[k] {
                0.7
            } else {
                0.4
            };
            self.ganancia[k] += paso * (objetivo - self.ganancia[k]);
        }
        &self.ganancia
    }
}
