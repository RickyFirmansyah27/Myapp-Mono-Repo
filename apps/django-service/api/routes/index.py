# api/routes/index.py
from django.urls import path
from api.views.getHello import getHello

urlpatterns = [
    path('api/hello/', getHello, name='hello'),
]
