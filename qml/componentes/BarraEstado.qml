import QtQuick
import ".."

// Punto de color + estado + motor; si hay error, muestra el mensaje en rojo.
Item {
    id: raiz
    property string estado: "iniciando"
    property string modelo: ""
    property string mensaje: ""
    readonly property var _rotulos: ({ iniciando: "Iniciando", escuchando: "Escuchando", pausado: "En pausa", error: "Error" })

    implicitHeight: Tema.u * 3
    implicitWidth: Tema.u * 40

    Rectangle {
        id: punto
        width: Tema.u; height: width; radius: width / 2
        anchors.verticalCenter: parent.verticalCenter
        color: Tema.acentoDe(raiz.estado)
        Behavior on color { ColorAnimation { duration: Tema.suave } }
    }
    Text {
        anchors { left: punto.right; leftMargin: Tema.u; right: parent.right; verticalCenter: parent.verticalCenter }
        elide: Text.ElideRight
        textFormat: Text.PlainText
        text: raiz.estado === "error" && raiz.mensaje.length > 0
              ? raiz.mensaje
              : (raiz._rotulos[raiz.estado] || raiz.estado) + (raiz.modelo.length > 0 ? "  ·  " + raiz.modelo : "")
        color: raiz.estado === "error" ? Tema.rojo : Tema.tenue
        font { family: Tema.fuente; pixelSize: Tema.tPequeno + 1 }
    }
    Accessible.role: Accessible.StaticText
    Accessible.name: "Estado " + estado
}
