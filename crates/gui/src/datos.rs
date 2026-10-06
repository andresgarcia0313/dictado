//! Estado interno del `Controlador` (los campos que QML ve como propiedades).
use cxx_qt_lib::QString;

pub struct ControladorRust {
    pub estado: QString,
    pub parcial: QString,
    pub historial: QString,
    pub nivel: f64,
    pub ram_app: f64,
    pub ram_motor: f64,
    pub modelo: QString,
    pub mensaje: QString,
    pub mejora: bool,
    pub ruido: bool,
    pub espectro: QString,
    pub ganancia: f64,
    pub mics: QString,
    pub mic: QString,
    pub entorno: QString,
    pub snr: f64,
    pub ruta_registros: QString,
    pub saltar_filtros: bool,
    pub ruta_vocabulario: QString,
}

impl Default for ControladorRust {
    fn default() -> Self {
        Self {
            estado: QString::from("iniciando"),
            parcial: QString::default(),
            historial: QString::default(),
            nivel: 0.0,
            ram_app: 0.0,
            ram_motor: 0.0,
            modelo: QString::default(),
            mensaje: QString::default(),
            mejora: true,
            ruido: false,
            espectro: QString::default(),
            ganancia: 0.0,
            mics: QString::default(),
            mic: QString::from("auto"),
            entorno: QString::from("silencio"),
            snr: 0.0,
            ruta_registros: QString::from(&registro::carpeta_registros().display().to_string()),
            saltar_filtros: true,
            ruta_vocabulario: QString::from(
                &crate::filtros::ruta_vocabulario().display().to_string(),
            ),
        }
    }
}
