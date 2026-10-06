import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-Subtitulo.qml
Rectangle {
    width: 640; height: 260; color: Tema.fondo
    Subtitulo {
        id: s
        anchors { fill: parent; margins: 24 }
        parcial: "hola como estas hoy quiero revisar el informe"
    }
}
