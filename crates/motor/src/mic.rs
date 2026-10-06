//! Fuente de microfono con cpal: pide 16 kHz mono y, si no se puede, remuestrea.
use crate::audio::FS;
use crate::fuente::{Fuente, Lectura};
use crate::mic_cpal::{abrir, config_16k, COLA_BLOQUES};
use cpal::traits::StreamTrait;
use cpal::Stream;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::mpsc::{sync_channel, Receiver, RecvTimeoutError};
use std::sync::Arc;
use std::time::Duration;

pub struct Mic {
    _flujo: Stream,
    rx: Receiver<Vec<f32>>,
    nombre: String,
    /// Bloques enviados y aun no leidos: si crece, el motor no da abasto.
    cola: Arc<AtomicUsize>,
}

impl Mic {
    /// `nombre` = None usa el micrófono por defecto.
    pub fn abrir(nombre: Option<&str>) -> Result<Self, String> {
        Self::dentro(nombre).map_err(|e| format!("No se pudo abrir el micrófono: {e}"))
    }

    fn dentro(pedido: Option<&str>) -> Result<Self, String> {
        let (nombre, d) = crate::dispositivos::buscar(pedido)?;
        let (conf, enumerado) = config_16k(&d)?;
        let (tasa, canales, buffer) = (conf.sample_rate, conf.channels, conf.buffer_size);
        let (tx, rx) = sync_channel(COLA_BLOQUES);
        let cola = Arc::new(AtomicUsize::new(0));
        let (f, negociado) = abrir(&d, conf, enumerado, tx, cola.clone())?;
        registro::info!(
            "dispositivo nombre=\"{nombre}\" tasa={tasa} canales={canales} formato={negociado} buffer={buffer:?} remuestreo={}",
            tasa != FS
        );
        f.play().map_err(|e| e.to_string())?;
        Ok(Self {
            _flujo: f,
            rx,
            nombre,
            cola,
        })
    }
}

impl Fuente for Mic {
    fn leer(&mut self) -> Lectura {
        match self.rx.recv_timeout(Duration::from_millis(100)) {
            Ok(m) => {
                self.cola.fetch_sub(1, Relaxed);
                Lectura::Muestras(m)
            }
            Err(RecvTimeoutError::Timeout) => Lectura::Vacio,
            Err(RecvTimeoutError::Disconnected) => Lectura::Fin,
        }
    }

    fn dispositivo(&self) -> String {
        self.nombre.clone()
    }
}
