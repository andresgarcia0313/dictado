//! Dictado - GUI Qt Quick sobre cxx-qt. Lanza el motor STT como proceso hijo.
mod ajustes;
mod aplicar;
mod cierre;
mod controlador;
mod datos;
mod diario;
mod evento;
mod filtros;
mod motor;
mod nativo;
mod ram;
mod vida;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

fn main() {
    registro::iniciar("gui");
    // Estilo ligero y uniforme en las tres plataformas.
    if std::env::var_os("QT_QUICK_CONTROLS_STYLE").is_none() {
        std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Basic");
    }
    registro::info!(
        "arranque version={} pid={} motor=\"{}\"",
        env!("CARGO_PKG_VERSION"),
        std::process::id(),
        evento::ruta_motor().display()
    );
    let mut app = QGuiApplication::new();
    nativo::fijar_icono();
    let mut motor_qml = QQmlApplicationEngine::new();
    if let Some(mut app) = app.as_mut() {
        // Claves de `Settings` en Main.qml: organización y aplicación fijas.
        app.as_mut()
            .set_organization_name(&QString::from("ingeniumcodex"));
        app.set_application_name(&QString::from("dictado"));
    }
    registro::info!("geometria guardada {}", nativo::geometria());
    if let Some(motor_qml) = motor_qml.as_mut() {
        motor_qml.load(&QUrl::from("qrc:/qt/qml/dictado/app/Raiz.qml"));
    }
    if let Some(app) = app.as_mut() {
        app.exec();
    }
    drop(motor_qml);
    registro::info!("cierre geometria {}", nativo::geometria());
    motor::detener();
    registro::info!("cierre completo");
}
