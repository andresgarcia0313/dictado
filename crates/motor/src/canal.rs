//! Reducción a mono por SNR: si un canal está muerto o muy bajo se usa solo el fuerte, en vez de promediar
//! (un promedio con un canal mudo baja la voz 6 dB). Decide cada 2 s con histéresis para no saltar.

use crate::canal_regla::{decidir, Modo, PRIMERA_S, VENTANA_S};

pub struct Canales {
    n: usize,
    modo: Modo,
    cuadros: usize,
    limite: usize,
    cov: Vec<f32>,
    hz: usize,
}

impl Canales {
    pub fn nuevo(n: usize, hz: u32) -> Self {
        let hz = hz as usize;
        Self {
            n,
            modo: Modo::Promedio,
            cuadros: 0,
            limite: (PRIMERA_S * hz as f32) as usize,
            cov: vec![0.0; n * n],
            hz,
        }
    }

    /// Convierte bloques intercalados a mono según el modo vigente.
    pub fn procesar(&mut self, m: &[f32]) -> Vec<f32> {
        if self.n <= 1 {
            return m.to_vec();
        }
        let n = self.n;
        let mut salida = Vec::with_capacity(m.len() / n);
        for c in m.chunks_exact(n) {
            for i in 0..n {
                for j in i..n {
                    self.cov[i * n + j] += c[i] * c[j];
                }
            }
            salida.push(match self.modo {
                Modo::Promedio => c.iter().sum::<f32>() / n as f32,
                Modo::Solo(i) => c[i],
            });
        }
        self.cuadros += salida.len();
        if self.cuadros >= self.limite {
            self.decidir();
        }
        salida
    }

    fn decidir(&mut self) {
        let n = self.n;
        let c = |i: usize, j: usize| self.cov[i.min(j) * n + i.max(j)];
        let rms: Vec<f32> = (0..n)
            .map(|i| (c(i, i) / self.cuadros as f32).sqrt())
            .collect();
        let fuerte = (0..n).fold(0, |m, i| if rms[i] > rms[m] { i } else { m });
        let corr: Vec<f32> = (0..n)
            .filter(|&i| i != fuerte)
            .map(|i| c(i, fuerte) / (c(i, i) * c(fuerte, fuerte)).sqrt().max(1e-12))
            .collect();
        let nuevo = decidir(self.modo, &rms, &corr);
        if nuevo != self.modo {
            registro::info!("canal modo={nuevo:?} rms={rms:?} corr={corr:?}");
            self.modo = nuevo;
        }
        self.cov.fill(0.0);
        (self.cuadros, self.limite) = (0, (VENTANA_S * self.hz as f32) as usize);
    }
}

#[cfg(test)]
#[path = "canal_pruebas.rs"]
mod pruebas;
