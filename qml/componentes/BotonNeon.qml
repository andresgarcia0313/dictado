import QtQuick
import ".."

// Botón minimalista: borde de acento y relleno translúcido; `primario` lo rellena más.
Rectangle {
    id: raiz
    property string texto: "Botón"
    property color acento: Tema.cian
    property bool primario: false
    signal pulsado()

    implicitWidth: etiqueta.implicitWidth + Tema.u * 4
    implicitHeight: Tema.u * 5
    radius: Tema.radioChico
    color: Tema.translucido(acento, zona.pressed ? 0.30 : (zona.containsMouse ? 0.20 : (primario ? 0.16 : 0.05)))
    border { width: 1; color: Tema.translucido(acento, zona.containsMouse || primario ? 0.9 : 0.5) }
    scale: zona.pressed ? 0.97 : 1
    Behavior on color { ColorAnimation { duration: Tema.rapido } }
    Behavior on scale { NumberAnimation { duration: Tema.rapido; easing.type: Easing.OutCubic } }

    Text {
        id: etiqueta
        anchors.centerIn: parent
        text: raiz.texto
        color: raiz.acento
        font { family: Tema.fuente; pixelSize: Tema.tCuerpo - 1; weight: Font.DemiBold }
    }
    MouseArea {
        id: zona
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: raiz.pulsado()
    }
    Accessible.role: Accessible.Button
    Accessible.name: texto
    Accessible.onPressAction: pulsado()
}
