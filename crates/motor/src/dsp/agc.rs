//! Nivelacion de volumen (AGC): lleva la voz a -20 dBFS usando el nivel de los ultimos 15 s,
//! medido SOLO en tramas de voz. En silencio la ganancia se congela y no se amplifica.
use super::limitador::rampa;
use crate::audio::{rms, FS};
use std::collections::VecDeque;

const OBJETIVO_DB: f32 = -20.0;
const PISO_DB: f32 = -50.0; // por debajo no es voz: no cuenta ni se amplifica
const MAX_DB: f32 = 30.0;
const MIN_DB: f32 = -12.0;
const VENTANA_S: f64 = 15.0;
const ATAQUE_S: f32 = 0.1;
const LIBERACION_S: f32 = 2.0;
const CAMBIO_DB_S: f32 = 3.0;
const ARRANQUE_S: f32 = 3.0; // con poca voz vista se converge rapido (24 dB/s, ataque)
const CAMBIO_ARRANQUE_DB_S: f32 = 24.0;

fn db(x: f32) -> f32 {
    20.0 * x.max(1e-9).log10()
}

pub struct Agc {
    historia: VecDeque<(f64, f64, f64)>, // (instante, duracion, energia)
    reloj: f64,
    voz_total: f32,
    objetivo_db: f32,
    aplicada_db: f32,
    previa: f32, // ganancia lineal al final del bloque anterior
}

impl Agc {
    pub fn nuevo() -> Self {
        Self {
            historia: VecDeque::new(),
            reloj: 0.0,
            voz_total: 0.0,
            objetivo_db: 0.0,
            aplicada_db: 0.0,
            previa: 1.0,
        }
    }

    pub fn ganancia_db(&self) -> f32 {
        self.aplicada_db
    }

    fn nivel_voz_db(&self) -> Option<f32> {
        let (d, e) = self
            .historia
            .iter()
            .fold((0.0, 0.0), |a, h| (a.0 + h.1, a.1 + h.1 * h.2));
        (d > 0.0).then(|| 10.0 * ((e / d) as f32).max(1e-18).log10())
    }

    /// `hay_voz`: el VAD vio voz en este bloque (si no, ni el ruido fuerte cuenta ni se amplifica).
    pub fn procesar(&mut self, m: &mut [f32], hay_voz: bool) {
        let dur = m.len() as f32 / FS as f32;
        self.reloj += dur as f64;
        let r = rms(m);
        let voz = hay_voz && db(r) > PISO_DB;
        if voz {
            self.historia
                .push_back((self.reloj, dur as f64, (r * r) as f64));
            self.voz_total += dur;
        }
        while self
            .historia
            .front()
            .is_some_and(|h| self.reloj - h.0 > VENTANA_S)
        {
            self.historia.pop_front();
        }
        let arranque = self.voz_total < ARRANQUE_S;
        if let Some(nivel) = self.nivel_voz_db() {
            let deseada = (OBJETIVO_DB - nivel).clamp(MIN_DB, MAX_DB);
            let paso = dur
                * if arranque {
                    CAMBIO_ARRANQUE_DB_S
                } else {
                    CAMBIO_DB_S
                };
            self.objetivo_db += (deseada - self.objetivo_db).clamp(-paso, paso);
        }
        let tau = if arranque || self.objetivo_db < self.aplicada_db {
            ATAQUE_S
        } else {
            LIBERACION_S
        };
        self.aplicada_db += (self.objetivo_db - self.aplicada_db) * (1.0 - (-dur / tau).exp());
        let efectiva = if voz {
            self.aplicada_db
        } else {
            self.aplicada_db.min(0.0)
        };
        let fin = 10f32.powf(efectiva / 20.0);
        rampa(m, self.previa, fin);
        self.previa = fin;
    }
}
