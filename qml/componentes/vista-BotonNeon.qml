import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-BotonNeon.qml
Rectangle {
    width: 420; height: 160; color: Tema.fondo
    property string ultimo: "(sin pulsar)"
    Row {
        anchors.centerIn: parent
        spacing: 12
        BotonNeon { texto: "Pausar"; primario: true; onPulsado: parent.parent.ultimo = "Pausar" }
        BotonNeon { texto: "Limpiar"; acento: Tema.magenta; onPulsado: parent.parent.ultimo = "Limpiar" }
        BotonNeon { texto: "Copiar"; acento: Tema.verde; onPulsado: parent.parent.ultimo = "Copiar" }
    }
    Text {
        anchors { bottom: parent.bottom; horizontalCenter: parent.horizontalCenter; margins: 16 }
        text: parent.ultimo; color: Tema.tenue
    }
}
