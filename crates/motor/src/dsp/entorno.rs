//! Analisis del entorno por tramas de 16 ms: VAD, SNR y estado voz/ruido/silencio.
//! El evento `entorno` sale 4 veces por segundo de AUDIO (no de reloj: igual con micro y con WAV).
use super::medida::Medida;
use super::vad::{self, Vad, TRAMA};
use crate::evento::Evento;

const CADA: usize = 4_000; // 250 ms a 16 kHz
const RESUMEN: usize = 80_000; // 5 s

pub struct Entorno {
    vad: Box<dyn Vad>,
    pend: Vec<f32>,
    medida: Medida,
    p_suma: f32,
    p_n: u32,
    pico: f32,
    muestras: usize,
}

impl Entorno {
    pub fn nuevo() -> Self {
        Self::con_vad(vad::crear())
    }

    pub fn con_vad(vad: Box<dyn Vad>) -> Self {
        Self {
            vad,
            pend: vec![],
            medida: Medida::nueva(),
            p_suma: 0.0,
            p_n: 0,
            pico: 0.0,
            muestras: 0,
        }
    }

    /// Analiza las tramas completas del bloque y llama `f(trama, p, snr)` por cada una.
    pub fn analizar(&mut self, m: &[f32], mut f: impl FnMut(&[f32], f32, Option<f32>)) {
        self.pend.extend_from_slice(m);
        let n = self.pend.len() / TRAMA * TRAMA;
        let tramas: Vec<f32> = self.pend.drain(..n).collect();
        for t in tramas.chunks_exact(TRAMA) {
            let p = self.vad.p(t);
            self.medida.trama(t, p);
            self.pico = t.iter().fold(self.pico, |a, x| a.max(x.abs()));
            (self.p_suma, self.p_n) = (self.p_suma + p, self.p_n + 1);
            f(t, p, self.medida.snr());
            self.reloj(TRAMA);
        }
    }

    fn reloj(&mut self, n: usize) {
        let antes = self.muestras;
        self.muestras += n;
        if antes / CADA != self.muestras / CADA {
            let p = self.p_suma / self.p_n.max(1) as f32; // media de la ventana de 250 ms
            Evento::Entorno {
                estado: self.medida.estado(p),
                p,
                snr: self.medida.snr().unwrap_or(0.0),
            }
            .emitir();
            (self.p_suma, self.p_n) = (0.0, 0);
        }
        if antes / RESUMEN != self.muestras / RESUMEN {
            let (voz, ruido) = self.medida.conteos();
            registro::depura!(
                "resumen entorno nivel_db={:.1} piso_db={:.1} pico={:.2} snr_db={:.1} tramas_voz={voz} tramas_ruido={ruido}",
                self.medida.nivel_db(),
                self.medida.piso_db(),
                self.pico,
                self.medida.snr().unwrap_or(0.0)
            );
            self.pico = 0.0;
        }
    }

    pub fn reiniciar(&mut self) {
        *self = Self::nuevo();
    }
}
