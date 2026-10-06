//! Una frase: la p de voz de `Mejora` con histeresis abre y cierra el segmento; 100 ms de relleno previo.
use crate::audio::nivel;
use crate::dsp::{Cambio, Histeresis};
use crate::evento::Evento;
use crate::reconocedor::Reconocedor;
use std::time::{Duration, Instant};

const PREVIO: usize = 1600; // 100 ms
const CADA_PARCIAL: Duration = Duration::from_millis(66);
const CADA_NIVEL: Duration = Duration::from_millis(50);

pub struct Frase {
    hist: Histeresis,
    previo: Vec<f32>,
    ultimo: String,
    t_parcial: Instant,
    t_nivel: Instant,
}

pub fn capitalizar(t: &str) -> String {
    let mut c = t.chars();
    c.next()
        .map(|p| p.to_uppercase().chain(c).collect())
        .unwrap_or_default()
}

impl Frase {
    pub fn nueva() -> Self {
        let ahora = Instant::now();
        Self {
            hist: Histeresis::default(),
            previo: vec![],
            ultimo: String::new(),
            t_parcial: ahora,
            t_nivel: ahora,
        }
    }

    /// `ps` = p de voz por trama de este bloque (de `Mejora`); el bloque se reparte entre ellas.
    pub fn procesar(&mut self, rec: &mut dyn Reconocedor, bloque: &[f32], ps: &[f32]) {
        if self.t_nivel.elapsed() >= CADA_NIVEL {
            Evento::Nivel(nivel(bloque)).emitir();
            self.t_nivel = Instant::now();
        }
        let n = ps.len().max(1);
        let mut audio: Vec<f32> = vec![];
        for i in 0..n {
            let t = &bloque[i * bloque.len() / n..(i + 1) * bloque.len() / n];
            let p = ps.get(i).copied();
            match p.map_or(Cambio::Nada, |p| self.hist.paso(p)) {
                Cambio::Entra => {
                    registro::info!(
                        "voz inicio p={:.2} previo_ms={}",
                        p.unwrap_or(0.0),
                        self.previo.len() / 16
                    );
                    audio.append(&mut self.previo);
                    audio.extend_from_slice(t);
                }
                Cambio::Sale => {
                    audio.extend_from_slice(t);
                    self.alimentar(rec, &mut audio);
                    self.finalizar(rec);
                }
                Cambio::Nada if self.hist.dentro() => audio.extend_from_slice(t),
                Cambio::Nada => self.guardar_previo(t),
            }
        }
        self.alimentar(rec, &mut audio);
        if self.hist.dentro() && rec.fin_de_frase() {
            self.cerrar(rec);
        }
    }

    fn guardar_previo(&mut self, t: &[f32]) {
        self.previo.extend_from_slice(t);
        let sobra = self.previo.len().saturating_sub(PREVIO);
        self.previo.drain(..sobra);
    }

    fn alimentar(&mut self, rec: &mut dyn Reconocedor, audio: &mut Vec<f32>) {
        if audio.is_empty() {
            return;
        }
        rec.alimentar(audio);
        audio.clear();
        let t = crate::vocabulario::aplicar_parcial(&cierre::limpiar(&rec.texto()));
        if t != self.ultimo && self.t_parcial.elapsed() >= CADA_PARCIAL {
            Evento::Parcial(&t).emitir();
            (self.ultimo, self.t_parcial) = (t, Instant::now());
        }
    }
}

#[path = "frase_cierre.rs"]
mod cierre;

#[cfg(test)]
#[path = "frase_pruebas.rs"]
mod pruebas;
