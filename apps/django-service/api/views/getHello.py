# api/views.py
from django.http import HttpResponse

def getHello(request):
    return HttpResponse("Hello, World!")
