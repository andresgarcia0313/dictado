import QtQuick
import ".."

// Interruptor con texto y nota opcional. Emite `cambiado(valor)` al pulsarlo.
Item {
    id: raiz
    property string texto: "Opción"
    property string nota: ""
    property bool marcado: false
    property color acento: Tema.cian
    signal cambiado(bool valor)

    implicitHeight: Math.max(Tema.u * 4, textos.implicitHeight)
    implicitWidth: Tema.u * 30

    Rectangle {
        id: pista
        width: Tema.u * 5; height: Tema.u * 3; radius: height / 2
        anchors { right: parent.right; top: parent.top; topMargin: 2 }
        color: Tema.translucido(raiz.acento, raiz.marcado ? 0.35 : 0.06)
        border { width: 1; color: raiz.marcado ? raiz.acento : Tema.tenue }
        Behavior on color { ColorAnimation { duration: Tema.rapido } }
        Rectangle {
            width: parent.height - 8; height: width; radius: width / 2
            y: 4; x: raiz.marcado ? parent.width - width - 4 : 4
            color: raiz.marcado ? raiz.acento : Tema.tenue
            Behavior on x { NumberAnimation { duration: Tema.rapido; easing.type: Easing.OutCubic } }
        }
    }
    Column {
        id: textos
        anchors { left: parent.left; right: pista.left; rightMargin: Tema.u; top: parent.top }
        Text {
            width: parent.width; text: raiz.texto; elide: Text.ElideRight
            color: Tema.texto; font { family: Tema.fuente; pixelSize: Tema.tCuerpo - 2 }
        }
        Text {
            width: parent.width; visible: raiz.nota !== ""; text: raiz.nota; wrapMode: Text.WordWrap
            color: Tema.tenue; font { family: Tema.fuente; pixelSize: Tema.tPequeno - 1 }
        }
    }
    MouseArea {
        anchors.fill: parent; cursorShape: Qt.PointingHandCursor
        onClicked: raiz.cambiado(!raiz.marcado)
    }
    Accessible.role: Accessible.CheckBox
    Accessible.name: texto
    Accessible.checked: marcado
    Accessible.onToggleAction: cambiado(!marcado)
}
