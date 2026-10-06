import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-Espectrograma.qml
Rectangle {
    width: 420; height: 220; color: Tema.fondo
    property real fase: 0
    Espectrograma {
        id: e
        anchors { fill: parent; margins: 24 }
    }
    Timer {
        interval: 40; repeat: true; running: true
        onTriggered: {
            parent.fase += 0.12
            e.bandas = Array.from({ length: 32 }, (_, i) =>
                Math.max(0, Math.min(255, Math.round(150 + 100 * Math.sin(parent.fase + i / 4) * Math.exp(-i / 24) + 40 * Math.random() - 60))))
        }
    }
}
