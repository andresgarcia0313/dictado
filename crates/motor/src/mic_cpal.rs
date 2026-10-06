//! Flujo cpal del micrófono: configuración a 16 kHz, formato negociado y cola acotada de bloques.
use crate::audio::{Remuestreador, FS};
use crate::canal::Canales;
use cpal::traits::DeviceTrait;
use cpal::{
    BufferSize, Device, FromSample, SampleFormat, SizedSample, Stream, StreamConfig,
    SupportedBufferSize,
};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::mpsc::{SyncSender, TrySendError};
use std::sync::Arc;
/// Capacidad de la cola: pasado esto se descarta lo nuevo y el retraso no crece.
pub const COLA_BLOQUES: usize = 64;
/// Bloques en cola a partir de los cuales se avisa de un desborde.
const COLA_AVISO: usize = 20;
/// Se registra el descarte en el primero y luego cada tantos.
const CADA_DESCARTE: usize = 50;

pub fn config_16k(d: &Device) -> Result<(StreamConfig, SampleFormat), String> {
    let rangos = d.supported_input_configs().map_err(|e| e.to_string())?;
    let mejor = rangos
        .filter(|r| r.contains_rate(FS))
        .min_by_key(|r| r.channels())
        .and_then(|r| r.try_with_sample_rate(FS));
    let cfg = match mejor {
        Some(c) => c,
        None => d.default_input_config().map_err(|e| e.to_string())?,
    };
    let mut conf = cfg.config();
    // cpal #554: un buffer menor a 1024 da audio malo.
    if let SupportedBufferSize::Range { min, max } = cfg.buffer_size() {
        conf.buffer_size = BufferSize::Fixed(1024.clamp(*min, *max));
    }
    Ok((conf, cfg.sample_format()))
}

/// Pone un bloque en la cola; si está llena lo descarta y lo cuenta (registro escaso).
pub fn encolar(tx: &SyncSender<Vec<f32>>, b: Vec<f32>, cola: &AtomicUsize, descartes: &mut usize) {
    match tx.try_send(b) {
        Ok(()) => {
            if cola.fetch_add(1, Relaxed) + 1 == COLA_AVISO + 1 {
                registro::aviso!("desborde: mas de {COLA_AVISO} bloques sin consumir");
            }
        }
        Err(TrySendError::Full(_)) => {
            *descartes += 1;
            if *descartes == 1 || descartes.is_multiple_of(CADA_DESCARTE) {
                registro::aviso!("cola llena: bloques descartados={descartes}");
            }
        }
        Err(TrySendError::Disconnected(_)) => {}
    }
}

fn flujo<T: SizedSample + 'static>(
    d: &Device,
    conf: StreamConfig,
    tx: SyncSender<Vec<f32>>,
    cola: Arc<AtomicUsize>,
) -> Result<Stream, String>
where
    f32: FromSample<T>,
{
    let canales = conf.channels as usize;
    let mut canal = Canales::nuevo(canales, conf.sample_rate);
    let mut rem = Remuestreador::nuevo(conf.sample_rate);
    let mut descartes = 0;
    let f = move |datos: &[T], _: &cpal::InputCallbackInfo| {
        let f32s: Vec<f32> = datos.iter().map(|m| m.to_sample::<f32>()).collect();
        encolar(
            &tx,
            rem.procesar(&canal.procesar(&f32s)),
            &cola,
            &mut descartes,
        );
    };
    d.build_input_stream(conf, f, |e| registro::error!("cpal error={e}"), None)
        .map_err(|e| e.to_string())
}

/// Abre el flujo con el formato que cpal convierte por nosotros (lo no entero va como f32);
/// devuelve también el formato NEGOCIADO, que es el que llega al motor.
pub fn abrir(
    d: &Device,
    conf: StreamConfig,
    enumerado: SampleFormat,
    tx: SyncSender<Vec<f32>>,
    cola: Arc<AtomicUsize>,
) -> Result<(Stream, &'static str), String> {
    match enumerado {
        SampleFormat::I16 => flujo::<i16>(d, conf, tx, cola).map(|f| (f, "I16")),
        SampleFormat::U16 => flujo::<u16>(d, conf, tx, cola).map(|f| (f, "U16")),
        SampleFormat::I32 => flujo::<i32>(d, conf, tx, cola).map(|f| (f, "I32")),
        _ => flujo::<f32>(d, conf, tx, cola).map(|f| (f, "F32")),
    }
}

#[cfg(test)]
#[path = "mic_cpal_pruebas.rs"]
mod pruebas;
