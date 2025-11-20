import os
from PyQt5.QtWidgets import QComboBox, QFileDialog
from PyQt5.QtGui import QIcon

class SignAppComboBox(QComboBox):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.refresh()
        self.setSizeAdjustPolicy(QComboBox.AdjustToContents)
        self.setMinimumContentsLength(30)

    def refresh(self):
        self.clear()
        app_dir = os.path.join('sign', 'app')
        exe_icon = QIcon(os.path.join('gui', 'icons', 'exe.ico')) if os.path.exists(os.path.join('gui', 'icons', 'exe.ico')) else QIcon()
        if os.path.isdir(app_dir):
            for f in os.listdir(app_dir):
                if f.lower().endswith('.exe'):
                    abs_path = os.path.abspath(os.path.join(app_dir, f))
                    self.addItem(exe_icon, f, abs_path)
        if self.count() > 0 and self.currentIndex() == -1:
            self.setCurrentIndex(0)

    def choose_file(self, parent=None):
        path, _ = QFileDialog.getOpenFileName(parent, '选择被伪造的应用', '.', 'EXE Files (*.exe);;All Files (*)')
        if path:
            display_name = os.path.basename(path)
            exe_icon = QIcon(os.path.join('gui', 'icons', 'exe.ico')) if os.path.exists(os.path.join('gui', 'icons', 'exe.ico')) else QIcon()
            for i in range(self.count()):
                if self.itemData(i) == path:
                    self.setCurrentIndex(i)
                    return
            self.addItem(exe_icon, display_name, path)
            self.setCurrentIndex(self.count() - 1)
