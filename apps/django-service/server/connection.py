import os
import django
from django.db import connections
from django.db.utils import OperationalError

# Tentukan DJANGO_SETTINGS_MODULE
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'server.settings')

def dbConnection():
    django.setup()  # Setup Django environment
    
    # Cetak versi Django
    print(f"Django Version: {django.get_version()}")
    
    # Koneksi ke database PostgreSQL
    db_conn = connections['default']
    try:
        # Mendapatkan versi PostgreSQL
        with db_conn.cursor() as cursor:
            cursor.execute("SELECT version();")
            db_version = cursor.fetchone()
            print(f"PostgreSQL Version: {db_version[0]}")
        
        print("Koneksi ke database berhasil!")
    except OperationalError as e:
        print(f"Koneksi gagal: {e}")

if __name__ == '__main__':
    dbConnection()
