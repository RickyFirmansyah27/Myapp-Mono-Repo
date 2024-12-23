from django.contrib import admin
from django.urls import path, include

urlpatterns = [
    path('', include('api.routes.index')), 
    path('admin/', admin.site.urls),
]
