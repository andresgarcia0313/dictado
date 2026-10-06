//! Etapa de reduccion de ruido: compuerta espectral opcional y su cola de salida (retraso de una trama).
use super::compuerta::Compuerta;
use super::vad::TRAMA;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Ruido {
    compuerta: Option<Compuerta>,
    salida: VecDeque<f32>,
}

impl Ruido {
    pub fn nuevo(on: bool) -> Self {
        let mut r = Self::default();
        r.fijar(on);
        r
    }

    pub fn fijar(&mut self, on: bool) {
        match (on, self.compuerta.is_some()) {
            (true, false) => {
                self.compuerta = Some(Compuerta::nueva());
                self.salida = VecDeque::from(vec![0.0; TRAMA]); // relleno = retraso de una trama
            }
            (false, true) => self.compuerta = None,
            _ => {}
        }
    }

    /// Olvida lo aprendido, conservando si estaba encendida.
    pub fn reiniciar(&mut self) {
        *self = Self::nuevo(self.compuerta.is_some());
    }

    pub fn trama(&mut self, t: &[f32], p: f32, snr: Option<f32>) {
        if let Some(c) = self.compuerta.as_mut() {
            self.salida.extend(c.trama(t, p, snr));
        }
    }

    /// Cambia el bloque por lo ya filtrado (misma longitud, retrasado una trama).
    pub fn aplicar(&mut self, m: &mut [f32]) {
        if self.compuerta.is_some() {
            for x in m.iter_mut() {
                *x = self.salida.pop_front().unwrap_or(0.0);
            }
        }
    }
}
