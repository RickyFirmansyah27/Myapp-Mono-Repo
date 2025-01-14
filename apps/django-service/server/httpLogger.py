import time
import logging

# Inisialisasi logger
logger = logging.getLogger('middleware')

class HTTPRequestLoggerMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request):
        # Catat waktu mulai request
        start_time = time.time()

        # Log Request info
        logger.info(f"Request | Method: {request.method} | Headers: {dict(request.headers)} | URL: {request.get_full_path()}")

        # Proses request dan dapatkan response
        response = self.get_response(request)

        # Catat waktu selesai dan hitung durasi dalam milidetik
        duration = (time.time() - start_time) * 1000  # Durasi dalam milidetik

        # Log Response info
        logger.info(f"Response | Method: {request.method} | URL: {request.get_full_path()} | Status: {response.status_code} | Duration: {duration:.2f} ms")

        return response
