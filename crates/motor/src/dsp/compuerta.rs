//! Compuerta espectral automatica: sustraccion suave del ruido estimado en tramas sin voz (p < 0.5).
//! Solo actua si el SNR medido es bajo (enciende bajo 6 dB, apaga sobre 10 dB). Sobre audio
//! limpio la ganancia es 1 y la salida es identica a la entrada (retrasada una trama).
use super::espectro_ruido::{EspectroRuido, BINES};
use super::histeresis::ENTRA;
use super::vad::TRAMA;
use realfft::num_complex::Complex;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};
use std::sync::Arc;

const N: usize = 2 * TRAMA;
pub const SNR_ENCIENDE: f32 = 6.0;
pub const SNR_APAGA: f32 = 10.0;

pub struct Compuerta {
    directa: Arc<dyn RealToComplex<f32>>,
    inversa: Arc<dyn ComplexToReal<f32>>,
    ventana: Vec<f32>,
    previa: Vec<f32>,
    solape: Vec<f32>,
    ruido: EspectroRuido,
    peso: f32,
    activa: bool,
}

impl Compuerta {
    pub fn nueva() -> Self {
        let mut plan = RealFftPlanner::<f32>::new();
        let hann = |i: usize| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / N as f32).cos();
        Self {
            directa: plan.plan_fft_forward(N),
            inversa: plan.plan_fft_inverse(N),
            ventana: (0..N).map(|i| hann(i).sqrt()).collect(),
            previa: vec![0.0; TRAMA],
            solape: vec![0.0; TRAMA],
            ruido: EspectroRuido::nuevo(),
            peso: 0.0,
            activa: false,
        }
    }

    #[cfg(test)]
    pub fn activa(&self) -> bool {
        self.activa
    }

    fn decidir(&mut self, snr: Option<f32>) {
        let Some(s) = snr else { return };
        if !self.activa && s < SNR_ENCIENDE {
            self.activa = true;
            registro::info!("compuerta espectral activada snr_db={s:.1}");
        } else if self.activa && s > SNR_APAGA {
            self.activa = false;
            registro::info!("compuerta espectral desactivada snr_db={s:.1}");
        }
    }

    /// Procesa una trama de `TRAMA` muestras; devuelve `TRAMA` muestras (retraso de una trama).
    pub fn trama(&mut self, x: &[f32], p: f32, snr: Option<f32>) -> Vec<f32> {
        self.decidir(snr);
        let mut b: Vec<f32> = self.previa.iter().chain(x).copied().collect();
        self.previa.copy_from_slice(x);
        for (m, w) in b.iter_mut().zip(&self.ventana) {
            *m *= w;
        }
        let mut espectro = vec![Complex::new(0.0, 0.0); BINES];
        let _ = self.directa.process(&mut b, &mut espectro);
        let pot: Vec<f32> = espectro.iter().map(|c| c.norm_sqr()).collect();
        if p < ENTRA {
            self.ruido.aprender(&pot);
        }
        let ganancia = if self.activa || self.peso > 0.0 {
            self.ruido.ganancias(&pot).to_vec()
        } else {
            vec![1.0; BINES]
        };
        let objetivo = (self.activa && self.ruido.vistas() > 10) as u8 as f32;
        self.peso += (objetivo - self.peso).clamp(-0.15, 0.15); // rampa de ~100 ms
        for (c, g) in espectro.iter_mut().zip(&ganancia) {
            *c *= 1.0 + self.peso * (g - 1.0);
        }
        (espectro[0].im, espectro[BINES - 1].im) = (0.0, 0.0);
        let mut y = vec![0.0; N];
        let _ = self.inversa.process(&mut espectro, &mut y);
        let salida = (0..TRAMA)
            .map(|i| self.solape[i] + y[i] * self.ventana[i] / N as f32)
            .collect();
        for i in 0..TRAMA {
            self.solape[i] = y[TRAMA + i] * self.ventana[TRAMA + i] / N as f32;
        }
        salida
    }
}

#[cfg(test)]
#[path = "compuerta_pruebas.rs"]
mod pruebas;
