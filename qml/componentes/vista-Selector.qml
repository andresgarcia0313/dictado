import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-Selector.qml
Rectangle {
    width: 360; height: 240; color: Tema.fondo
    Selector {
        id: s
        anchors { left: parent.left; right: parent.right; top: parent.top; margins: 24 }
        opciones: ["auto", "Micrófono integrado", "Auricular USB", "Webcam HD"]
        actual: "auto"; abierto: true
        onElegido: n => actual = n
    }
}
