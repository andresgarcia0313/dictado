//! QObject `Controlador` (módulo QML `dictado.app`): une el motor y la medición de RAM con QML.
use crate::datos::ControladorRust;
use crate::motor;
use crate::{nativo, ram};
use core::pin::Pin;
use cxx_qt::Threading;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, estado)]
        #[qproperty(QString, parcial)]
        #[qproperty(QString, historial)]
        #[qproperty(f64, nivel)]
        #[qproperty(f64, ram_app, cxx_name = "ramApp")]
        #[qproperty(f64, ram_motor, cxx_name = "ramMotor")]
        #[qproperty(QString, modelo)]
        #[qproperty(QString, mensaje)]
        #[qproperty(bool, mejora)]
        #[qproperty(bool, ruido)]
        #[qproperty(QString, espectro)]
        #[qproperty(f64, ganancia)]
        #[qproperty(QString, mics)]
        #[qproperty(QString, mic)]
        #[qproperty(QString, entorno)]
        #[qproperty(f64, snr)]
        #[qproperty(QString, ruta_registros, cxx_name = "rutaRegistros")]
        #[qproperty(bool, saltar_filtros, cxx_name = "saltarFiltros")]
        #[qproperty(QString, ruta_vocabulario, cxx_name = "rutaVocabulario")]
        type Controlador = super::ControladorRust;

        #[qinvokable]
        fn alternar(self: Pin<&mut Controlador>);
        #[qinvokable]
        fn limpiar(self: Pin<&mut Controlador>);
        #[qinvokable]
        fn copiar(self: &Controlador);
        #[qinvokable]
        #[cxx_name = "fijarMejora"]
        fn fijar_mejora(self: Pin<&mut Controlador>, activo: bool);
        #[qinvokable]
        #[cxx_name = "fijarRuido"]
        fn fijar_ruido(self: Pin<&mut Controlador>, activo: bool);
        #[qinvokable]
        #[cxx_name = "elegirMic"]
        fn elegir_mic(self: Pin<&mut Controlador>, nombre: &QString);
        #[qinvokable]
        #[cxx_name = "abrirRegistros"]
        fn abrir_registros(self: &Controlador);
        #[qinvokable]
        #[cxx_name = "abrirVocabulario"]
        fn abrir_vocabulario(self: &Controlador);
        #[qinvokable]
        #[cxx_name = "fijarSaltarFiltros"]
        fn fijar_saltar_filtros(self: Pin<&mut Controlador>, activo: bool);
    }

    impl cxx_qt::Threading for Controlador {}
    impl cxx_qt::Initialize for Controlador {}
}

impl cxx_qt::Initialize for qobject::Controlador {
    fn initialize(self: Pin<&mut Self>) {
        let hilo = self.qt_thread();
        let para_motor = self.qt_thread();
        motor::iniciar(move |ev| {
            nativo::traza(&ev);
            let _ = para_motor.queue(move |o| o.aplicar(ev));
        });
        ram::vigilar(hilo);
    }
}
