import os
import sys

# Menambahkan path folder 'server' ke sys.path
sys.path.append(os.path.join(os.path.dirname(__file__), 'server'))

from django.core.management import execute_from_command_line

# Import dbConnection dari connection.py yang sekarang ada di folder server
from connection import dbConnection

def runserver_with_connection_check():
    # Menjalankan connection.py untuk memeriksa koneksi database
    dbConnection()

    # Jika koneksi berhasil, lanjutkan menjalankan server Django
    execute_from_command_line(sys.argv)

if __name__ == "__main__":
    os.environ.setdefault("DJANGO_SETTINGS_MODULE", "server.settings")

    # Cek apakah perintah yang dijalankan adalah 'runserver'
    if "runserver" in sys.argv:
        runserver_with_connection_check()
    else:
        execute_from_command_line(sys.argv)