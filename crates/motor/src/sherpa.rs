//! Reconocedor sherpa-onnx: zipformer transducer streaming, busqueda greedy, 2 hilos.
use crate::audio::FS;
use crate::reconocedor::Reconocedor;
use crate::sherpa_config::{configuracion, HILOS};
use sherpa_onnx::{OnlineRecognizer, OnlineStream};
use std::path::Path;
use std::time::{Duration, Instant};

/// 1.5 s de ceros: sin ella el zipformer corta la ultima palabra de la frase.
const COLA: usize = 24_000;
const CADA_RESUMEN: Duration = Duration::from_secs(5);

pub struct Sherpa {
    rec: OnlineRecognizer,
    flujo: OnlineStream,
    decodificado: Duration,
    llamadas: u32,
    desde: Instant,
}

impl Sherpa {
    pub fn cargar(dir: &Path) -> Result<Self, String> {
        let c = configuracion(dir);
        let t = Instant::now();
        let rec = OnlineRecognizer::create(&c).ok_or_else(|| {
            registro::error!("fallo la carga del modelo ruta=\"{}\"", dir.display());
            format!("no se pudo cargar el modelo en {}", dir.display())
        })?;
        let flujo = rec.create_stream();
        registro::info!(
            "modelo cargado ruta=\"{}\" carga_ms={} hilos={HILOS}",
            dir.display(),
            t.elapsed().as_millis()
        );
        Ok(Self {
            rec,
            flujo,
            decodificado: Duration::ZERO,
            llamadas: 0,
            desde: Instant::now(),
        })
    }

    fn decodificar(&mut self) {
        let t = Instant::now();
        while self.rec.is_ready(&self.flujo) {
            self.rec.decode(&self.flujo);
        }
        self.decodificado += t.elapsed();
        self.llamadas += 1;
        if self.desde.elapsed() >= CADA_RESUMEN {
            let media = self.decodificado.as_secs_f64() * 1000.0 / f64::from(self.llamadas);
            registro::depura!(
                "decodificacion media_ms={media:.2} llamadas={}",
                self.llamadas
            );
            (self.decodificado, self.llamadas, self.desde) = (Duration::ZERO, 0, Instant::now());
        }
    }
}

impl Reconocedor for Sherpa {
    fn alimentar(&mut self, muestras: &[f32]) {
        self.flujo.accept_waveform(FS as i32, muestras);
        self.decodificar();
    }

    fn texto(&self) -> String {
        self.rec
            .get_result(&self.flujo)
            .map(|r| r.text.trim().to_string())
            .unwrap_or_default()
    }

    fn fin_de_frase(&self) -> bool {
        self.rec.is_endpoint(&self.flujo)
    }

    fn cerrar(&mut self) -> String {
        let t = Instant::now();
        self.flujo.accept_waveform(FS as i32, &[0.0; COLA]);
        self.decodificar();
        registro::depura!(
            "cierre de frase decodificacion_ms={}",
            t.elapsed().as_millis()
        );
        let t = self.texto();
        self.rec.reset(&self.flujo);
        t
    }
}
