import QtQuick
import ".."

// Finales unidos con "\n", seleccionables, con scroll que sigue al último.
Rectangle {
    id: raiz
    property string texto: ""
    property int tamano: 18
    readonly property bool haySeleccion: area.selectionStart !== area.selectionEnd

    color: Tema.superficie
    radius: Tema.radio
    border { width: 1; color: Tema.borde }
    clip: true

    Flickable {
        id: desliz
        anchors { fill: parent; margins: Tema.u * 2 }
        contentWidth: width
        contentHeight: area.height
        boundsBehavior: Flickable.StopAtBounds
        clip: true
        onContentHeightChanged: if (contentHeight > height) contentY = contentHeight - height
        TextEdit {
            id: area
            width: desliz.width
            text: raiz.texto
            readOnly: true
            selectByMouse: true
            wrapMode: TextEdit.Wrap
            color: Tema.texto
            selectionColor: Tema.translucido(Tema.cian, 0.35)
            font { family: Tema.fuente; pixelSize: raiz.tamano }
            Text {
                visible: raiz.texto.length === 0
                text: "Lo que digas quedará aquí."
                color: Tema.tenue
                font: area.font
            }
        }
    }
    Rectangle {
        anchors { right: parent.right; rightMargin: 4 }
        width: 3
        radius: 2
        visible: desliz.contentHeight > desliz.height
        color: Tema.translucido(Tema.cian, 0.4)
        height: Math.max(24, parent.height * desliz.height / desliz.contentHeight)
        y: 4 + (parent.height - 8 - height) * desliz.contentY / Math.max(1, desliz.contentHeight - desliz.height)
    }
    Accessible.role: Accessible.StaticText
    Accessible.name: "Historial de frases"
}
