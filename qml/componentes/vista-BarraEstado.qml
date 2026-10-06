import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-BarraEstado.qml
Rectangle {
    width: 420; height: 200; color: Tema.fondo
    Column {
        anchors.centerIn: parent
        spacing: 12
        BarraEstado { width: 360; estado: "escuchando"; modelo: "es-kroko" }
        BarraEstado { width: 360; estado: "pausado"; modelo: "es-kroko" }
        BarraEstado { width: 360; estado: "iniciando" }
        BarraEstado { width: 360; estado: "error"; mensaje: "El motor se cerró; reintentando (1/3)" }
    }
}
