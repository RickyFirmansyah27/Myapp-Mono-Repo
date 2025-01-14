import os
import sys
import logging
sys.path.append(os.path.join(os.path.dirname(__file__), 'server'))

from django.core.management import execute_from_command_line
from connection import dbConnection

def runserver_with_connection_check():
    dbConnection()

    execute_from_command_line(sys.argv)



if __name__ == "__main__":
    os.environ.setdefault("DJANGO_SETTINGS_MODULE", "server.settings")

    if "runserver" in sys.argv:
        runserver_with_connection_check()
    else:
        execute_from_command_line(sys.argv)
