//! Limita cuantos eventos se emiten por segundo (reloj de pared).
use std::time::{Duration, Instant};

pub struct Cupo {
    max: u32,
    usados: u32,
    desde: Instant,
}

impl Cupo {
    pub fn nuevo(max_por_segundo: u32) -> Self {
        Self {
            max: max_por_segundo,
            usados: 0,
            desde: Instant::now(),
        }
    }

    pub fn permite(&mut self) -> bool {
        if self.desde.elapsed() >= Duration::from_secs(1) {
            self.desde = Instant::now();
            self.usados = 0;
        }
        let ok = self.usados < self.max;
        self.usados += ok as u32;
        ok
    }
}
