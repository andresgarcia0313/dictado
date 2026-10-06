import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-Interruptor.qml
Rectangle {
    width: 360; height: 200; color: Tema.fondo
    Column {
        anchors { fill: parent; margins: 24 }
        spacing: 16
        Interruptor {
            id: a; width: parent.width; texto: "Mejora de audio"; marcado: true
            onCambiado: v => marcado = v
        }
        Interruptor {
            id: b; width: parent.width; texto: "Reducir ruido de calle"; acento: Tema.magenta
            nota: "automático: actúa solo si hay mucho ruido"; onCambiado: v => marcado = v
        }
        Text { text: "mejora=" + a.marcado + " ruido=" + b.marcado; color: Tema.tenue }
    }
}
