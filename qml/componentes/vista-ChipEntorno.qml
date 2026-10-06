import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-ChipEntorno.qml
Rectangle {
    width: 360; height: 160; color: Tema.fondo
    Column {
        anchors { fill: parent; margins: 24 }
        spacing: 12
        ChipEntorno { entorno: "voz"; snr: 14.2 }
        ChipEntorno { entorno: "ruido"; snr: 3.8 }
        ChipEntorno { entorno: "silencio" }
    }
}
