import QtQuick
import QtQuick.Controls
import Quickshell.Io
import qs.Commons
import qs.Ui

Panel {
  id: root
  moduleName: "dev.keyarchy.launcher"
  ipcTarget: "dev.keyarchy.launcher"

  property bool installed: false
  property bool checked: false

  function checkInstalled() {
    if (!probe.running) probe.running = true
  }

  function launch() {
    if (!installed || !bar) return
    bar.run("keyarchy")
    close()
  }

  implicitWidth: button.implicitWidth
  implicitHeight: button.implicitHeight

  Component.onCompleted: checkInstalled()
  onOpenedChanged: if (opened) checkInstalled()

  Process {
    id: probe
    command: ["sh", "-lc", "command -v keyarchy >/dev/null 2>&1"]
    onExited: function(exitCode) {
      root.installed = exitCode === 0
      root.checked = true
    }
  }

  BarIconButton {
    id: button
    anchors.fill: parent
    bar: root.bar
    text: "⌨"
    tooltipText: root.installed ? "Open Keyarchy" : "Keyarchy is not installed"
    onPressed: root.toggle()
  }

  KeyboardPanel {
    id: panel
    anchorItem: button
    owner: root
    bar: root.bar
    open: root.opened
    focusTarget: keyCatcher
    contentWidth: panel.fittedContentWidth(Style.space(320))
    contentHeight: panel.fittedContentHeight(content.implicitHeight, Style.space(220))

    PanelKeyCatcher {
      id: keyCatcher
      anchors.fill: parent
      onActivateRequested: root.launch()
      onCloseRequested: root.close()
      onTabRequested: function(direction) { root.switchPanel(direction) }

      Column {
        id: content
        width: parent.width
        spacing: Style.space(12)

        Text {
          width: parent.width
          text: "Keyarchy"
          color: root.bar ? root.bar.foreground : Color.foreground
          font.family: root.bar ? root.bar.fontFamily : Style.font.family
          font.pixelSize: Style.font.title
          font.bold: true
        }

        Text {
          width: parent.width
          text: !root.checked
            ? "Checking for Keyarchy…"
            : root.installed
              ? "Practice and learn your Omarchy hotkeys."
              : "Keyarchy is not installed or is not on PATH."
          color: root.bar ? root.bar.foreground : Color.foreground
          opacity: 0.75
          font.family: root.bar ? root.bar.fontFamily : Style.font.family
          font.pixelSize: Style.font.body
          wrapMode: Text.WordWrap
        }

        Button {
          id: launchButton
          text: "Open Keyarchy"
          enabled: root.installed
          Accessible.name: text
          onClicked: root.launch()
        }

        Text {
          visible: root.checked && !root.installed
          width: parent.width
          text: "Install the keyarchy package, then reopen this panel."
          color: root.bar ? root.bar.foreground : Color.foreground
          opacity: 0.6
          font.family: root.bar ? root.bar.fontFamily : Style.font.family
          font.pixelSize: Style.font.bodySmall
          wrapMode: Text.WordWrap
        }
      }
    }
  }
}
