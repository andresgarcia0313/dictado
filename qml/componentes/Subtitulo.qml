import QtQuick
import ".."

// Texto parcial en vivo: protagonista visual. Cian con halo suave y cursor que parpadea.
// Crece hacia arriba: siempre se ve el final de lo que se dice.
Item {
    id: raiz
    property string parcial: ""
    property string vacio: "Habla, te escucho..."
    property bool activo: true
    property int tamano: 40
    property real _cx: 0
    property real _cy: 0

    clip: true
    implicitHeight: Tema.u * 20

    Text {
        id: texto
        anchors { left: parent.left; right: parent.right; bottom: parent.bottom }
        text: raiz.parcial.length > 0 ? raiz.parcial : raiz.vacio
        wrapMode: Text.Wrap
        font.family: Tema.fuente
        font.pixelSize: raiz.tamano
        font.weight: Font.Medium
        lineHeight: 1.15
        color: raiz.parcial.length > 0 ? Tema.cian : Tema.tenue
        style: raiz.parcial.length > 0 ? Text.Outline : Text.Normal
        styleColor: Tema.translucido(Tema.cian, 0.22)
        Behavior on color { ColorAnimation { duration: Tema.suave } }
        onLineLaidOut: line => {
            raiz._cx = line.x + line.implicitWidth
            raiz._cy = line.y
        }
        Rectangle {
            id: cursor
            visible: raiz.activo
            x: raiz._cx + 4
            y: raiz._cy + raiz.tamano * 0.12
            width: 3
            height: raiz.tamano * 0.95
            radius: 2
            color: Tema.cian
            SequentialAnimation on opacity {
                loops: Animation.Infinite
                running: raiz.activo
                NumberAnimation { to: 0.15; duration: 600; easing.type: Easing.InOutSine }
                NumberAnimation { to: 1; duration: 600; easing.type: Easing.InOutSine }
            }
        }
    }
    Accessible.role: Accessible.StaticText
    Accessible.name: raiz.parcial
}
