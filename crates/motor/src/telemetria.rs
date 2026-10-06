//! Eventos de monitoreo del audio ya mejorado: espectro (<=25/s) y ganancia (2/s).
use crate::dsp::Cupo;
use crate::espectro::Espectro;
use crate::evento::Evento;
use std::time::{Duration, Instant};

const SALTO: usize = 640; // 40 ms a 16 kHz = una trama por llamada
const CADA_GANANCIA: Duration = Duration::from_millis(500);

pub struct Telemetria {
    espectro: Espectro,
    pendiente: Vec<f32>,
    cupo: Cupo,
    t_ganancia: Instant,
}

impl Telemetria {
    pub fn nueva() -> Self {
        Self {
            espectro: Espectro::nuevo(),
            pendiente: Vec::new(),
            cupo: Cupo::nuevo(25),
            t_ganancia: Instant::now(),
        }
    }

    pub fn alimentar(&mut self, mejorado: &[f32], ganancia_db: f32) {
        self.pendiente.extend_from_slice(mejorado);
        while self.pendiente.len() >= SALTO {
            let trozo: Vec<f32> = self.pendiente.drain(..SALTO).collect();
            if let Some(b) = self.espectro.procesar(&trozo) {
                if self.cupo.permite() {
                    Evento::Espectro(&b).emitir();
                }
            }
        }
        if self.t_ganancia.elapsed() >= CADA_GANANCIA {
            Evento::Ganancia(ganancia_db).emitir();
            self.t_ganancia = Instant::now();
        }
    }
}
