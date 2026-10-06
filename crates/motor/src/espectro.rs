//! Espectrograma: tramas de BANDAS valores 0-255 de lo que recibe el reconocedor (16 kHz).
//! FFT de 512 con ventana Hann, salto de 640 muestras (~40 ms), bandas log 80 Hz - 7.6 kHz.
use crate::audio::FS;
use realfft::num_complex::Complex;
use realfft::{RealFftPlanner, RealToComplex};
use std::sync::Arc;

pub const BANDAS: usize = 32;
const N: usize = 512;
const SALTO: usize = 640;
const F_MIN: f32 = 80.0;
const F_MAX: f32 = 7600.0;
const DB_MIN: f32 = -85.0;
const DB_MAX: f32 = -25.0;
const ALFA: f32 = 0.4;

pub struct Espectro {
    fft: Arc<dyn RealToComplex<f32>>,
    ventana: Vec<f32>,
    limites: [usize; BANDAS + 1],
    pendientes: Vec<f32>,
    entrada: Vec<f32>,
    salida: Vec<Complex<f32>>,
    previo: [f32; BANDAS],
}

impl Espectro {
    pub fn nuevo() -> Self {
        let fft = RealFftPlanner::<f32>::new().plan_fft_forward(N);
        let ventana = (0..N)
            .map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / N as f32).cos())
            .collect();
        let mut limites = [0usize; BANDAS + 1];
        let hz_bin = FS as f32 / N as f32;
        for (i, l) in limites.iter_mut().enumerate() {
            let hz = F_MIN * (F_MAX / F_MIN).powf(i as f32 / BANDAS as f32);
            *l = (hz / hz_bin).round() as usize;
        }
        Self {
            salida: fft.make_output_vec(),
            entrada: fft.make_input_vec(),
            fft,
            ventana,
            limites,
            pendientes: Vec::with_capacity(SALTO + N),
            previo: [0.0; BANDAS],
        }
    }

    /// Acepta muestras a 16 kHz; devuelve una trama cada ~40 ms o `None` si aun no hay.
    /// Si llegan varias tramas en un bloque, devuelve la última.
    pub fn procesar(&mut self, muestras: &[f32]) -> Option<[u8; BANDAS]> {
        self.pendientes.extend_from_slice(muestras);
        let mut ultima = None;
        while self.pendientes.len() >= SALTO {
            ultima = Some(self.trama());
            self.pendientes.drain(..SALTO);
        }
        ultima
    }

    fn trama(&mut self) -> [u8; BANDAS] {
        for (i, e) in self.entrada.iter_mut().enumerate() {
            *e = self.pendientes[i] * self.ventana[i];
        }
        let _ = self.fft.process(&mut self.entrada, &mut self.salida);
        let mut trama = [0u8; BANDAS];
        for (b, t) in trama.iter_mut().enumerate() {
            let (a, z) = (
                self.limites[b],
                self.limites[b + 1].max(self.limites[b] + 1),
            );
            let pot: f32 =
                self.salida[a..z].iter().map(|c| c.norm_sqr()).sum::<f32>() / (z - a) as f32;
            // Amplitud de una senoide de pico 1 = 1 en el bin: |X| * 2 / suma(ventana) = |X| / (N/4).
            let db = 10.0 * (pot.max(1e-20)).log10() - 20.0 * (N as f32 / 4.0).log10();
            let v = ((db - DB_MIN) / (DB_MAX - DB_MIN)).clamp(0.0, 1.0) * 255.0;
            self.previo[b] = ALFA * self.previo[b] + (1.0 - ALFA) * v;
            *t = self.previo[b].round() as u8;
        }
        trama
    }
}

#[cfg(test)]
#[path = "espectro_pruebas.rs"]
mod pruebas;
