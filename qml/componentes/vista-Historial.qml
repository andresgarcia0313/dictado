import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-Historial.qml
Rectangle {
    width: 560; height: 260; color: Tema.fondo
    Historial {
        id: h
        anchors { fill: parent; margins: 16 }
        texto: "Hola, como estas.\nQuiero revisar el informe del mes.\nPor favor agenda la reunion del martes."
    }
    Timer {
        interval: 700; running: true; repeat: true
        onTriggered: h.texto += "\nFrase numero " + (h.texto.split("\n").length + 1) + " de la prueba."
    }
}
