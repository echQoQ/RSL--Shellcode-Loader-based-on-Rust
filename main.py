import sys
from PyQt5.QtWidgets import QApplication
from gui.main_window import LoaderGUI

if __name__ == '__main__':
    app = QApplication(sys.argv)
    gui = LoaderGUI()
    gui.show()
    sys.exit(app.exec_())
