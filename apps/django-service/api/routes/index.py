# api/routes/index.py
from django.urls import path
from api.controller import userController

urlpatterns = [
    path('api/users/', userController.getUsers, name='get user data'),
]
