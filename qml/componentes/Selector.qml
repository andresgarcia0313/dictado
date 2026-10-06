import QtQuick
import ".."

// Selector de una lista (p. ej. micrófonos). `opciones` son textos; "auto" se muestra "Automático".
// Cerrado muestra `actual`; al pulsarlo se despliega la lista y emite `elegido(valor)`.
Rectangle {
    id: raiz
    property var opciones: []
    property string actual: ""
    property color acento: Tema.cian
    property bool abierto: false
    signal elegido(string valor)
    function _t(v) { return v === "auto" ? "Automático" : v }

    readonly property int _fila: Tema.u * 4
    implicitWidth: Tema.u * 30
    implicitHeight: _fila + (abierto ? lista.height + Tema.u / 2 : 0)
    clip: true
    radius: Tema.radioChico
    color: Tema.translucido(acento, abierto ? 0.08 : 0.04)
    border { width: 1; color: Tema.translucido(acento, abierto ? 0.9 : 0.4) }
    Behavior on implicitHeight { NumberAnimation { duration: Tema.rapido; easing.type: Easing.OutCubic } }

    Item {
        width: parent.width; height: raiz._fila
        Text {
            anchors { left: parent.left; right: flecha.left; margins: Tema.u * 1.5; verticalCenter: parent.verticalCenter }
            text: raiz._t(raiz.actual); elide: Text.ElideRight
            color: Tema.texto; font { family: Tema.fuente; pixelSize: Tema.tCuerpo - 2 }
        }
        Text {
            id: flecha
            anchors { right: parent.right; rightMargin: Tema.u * 1.5; verticalCenter: parent.verticalCenter }
            text: raiz.abierto ? "▴" : "▾"; color: raiz.acento
        }
        MouseArea { anchors.fill: parent; cursorShape: Qt.PointingHandCursor; onClicked: raiz.abierto = !raiz.abierto }
    }
    Column {
        id: lista
        y: raiz._fila; width: parent.width
        Repeater {
            model: raiz.opciones
            Rectangle {
                required property string modelData
                readonly property bool sel: modelData === raiz.actual
                width: lista.width; height: Tema.u * 4
                color: Tema.translucido(raiz.acento, zona.containsMouse ? 0.16 : (sel ? 0.08 : 0))
                Text {
                    anchors { fill: parent; leftMargin: Tema.u * 1.5; rightMargin: Tema.u * 1.5 }
                    verticalAlignment: Text.AlignVCenter; elide: Text.ElideRight
                    text: raiz._t(modelData); color: sel ? raiz.acento : Tema.tenue
                    font { family: Tema.fuente; pixelSize: Tema.tCuerpo - 2; weight: sel ? Font.DemiBold : Font.Normal }
                }
                MouseArea {
                    id: zona; anchors.fill: parent; hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                    onClicked: { raiz.abierto = false; raiz.elegido(modelData) }
                }
            }
        }
    }
    Accessible.role: Accessible.ComboBox
    Accessible.name: _t(actual)
}
