//! Lógica del Controlador: aplica eventos del motor y atiende los invokables de QML.
use crate::ajustes;
use crate::controlador::qobject;
use crate::evento::Evento;
use crate::{motor, nativo};
use core::pin::Pin;
use cxx_qt_lib::QString;

impl qobject::Controlador {
    pub fn es(&self, estado: &str) -> bool {
        self.estado().to_string() == estado
    }

    pub fn aplicar(mut self: Pin<&mut Self>, ev: Evento) {
        match ev {
            Evento::Iniciando => {
                ajustes::nueva_vida();
                self.set_estado(QString::from("iniciando"));
            }
            Evento::Listo { modelo } => {
                self.as_mut().set_modelo(QString::from(&modelo));
                self.as_mut().set_mensaje(QString::default());
                self.as_mut().ajustes_de_entorno();
                self.reenviar_ajustes();
                self.set_estado(QString::from("escuchando"));
            }
            Evento::Parcial(t) => self.set_parcial(QString::from(&t)),
            Evento::Final(t) => {
                let previo = self.historial().to_string();
                let nuevo = if previo.is_empty() {
                    t
                } else {
                    format!("{previo}\n{t}")
                };
                self.as_mut().set_historial(QString::from(&nuevo));
                self.set_parcial(QString::default());
            }
            Evento::Nivel(n) => self.set_nivel(n.clamp(0.0, 1.0)),
            Evento::Espectro(csv) => self.set_espectro(QString::from(&csv)),
            Evento::Ganancia(db) => self.set_ganancia(db),
            Evento::Mics { lista, actual } => {
                self.as_mut().set_mics(QString::from(&lista));
                if !actual.is_empty() {
                    self.set_mic(QString::from(&actual));
                }
            }
            Evento::Entorno { estado, snr } => {
                self.as_mut().set_entorno(QString::from(&estado));
                self.set_snr(snr);
            }
            Evento::Error(m) => {
                self.as_mut().set_mensaje(QString::from(&m));
                self.set_estado(QString::from("error"));
            }
            Evento::Fin(limpio) => {
                self.as_mut().set_nivel(0.0);
                if !limpio {
                    self.as_mut().set_estado(QString::from("error"));
                }
                if std::env::var_os("DICTADO_AUTOSALIR").is_some() {
                    nativo::salir();
                }
            }
        }
    }

    pub fn alternar(mut self: Pin<&mut Self>) {
        if self.es("escuchando") {
            motor::enviar("pausa");
            self.as_mut().set_nivel(0.0);
            self.set_estado(QString::from("pausado"));
        } else if self.es("pausado") {
            motor::enviar("reanuda");
            self.set_estado(QString::from("escuchando"));
        }
    }

    pub fn limpiar(mut self: Pin<&mut Self>) {
        self.as_mut().set_historial(QString::default());
        self.set_parcial(QString::default());
    }

    pub fn copiar(&self) {
        nativo::copiar(&self.historial().to_string());
    }

    pub fn abrir_registros(&self) {
        let ruta = self.ruta_registros().to_string();
        registro::info!("se abre la carpeta de registros ruta=\"{ruta}\"");
        nativo::abrir_carpeta(&ruta);
    }
}
