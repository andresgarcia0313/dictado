import QtQuick
import dictado.app

// Entrada de la app real: Main conectado al Controlador de Rust.
Main {
    ctl: Controlador {}
}
