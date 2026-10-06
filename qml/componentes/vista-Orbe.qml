import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-Orbe.qml
Rectangle {
    width: 320; height: 240; color: Tema.fondo
    Orbe { id: o; anchors.centerIn: parent; width: 160 }
    Timer {
        interval: 60; running: true; repeat: true
        onTriggered: o.nivel = 0.5 + 0.5 * Math.sin(Date.now() / 300)
    }
}
