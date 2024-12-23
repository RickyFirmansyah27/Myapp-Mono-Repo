from django.http.response import JsonResponse

def BaseResponse(response_type, message, data=None):
    if response_type == 'success':
        response_data = successResponse(data, message)
        status_code = 200
    elif response_type == 'error':
        response_data = badRequestResponse(message)
        status_code = 400
    elif response_type == 'notfound':
        response_data = notFoundResponse(message)
        status_code = 404
    else:
        response_data = internalServerErrorResponse(message)
        status_code = 500

    return JsonResponse(response_data, status=status_code, safe=False)

def successResponse(data, message):
    return {
        "statusCode": 200,
        "status": True,
        "data": data,
        "message": message
    }

def invalidResponse(message):
    return {
        "statusCode": 400,
        "status": False,
        "data": None,
        "message": message
    }

def badRequestResponse(message):
    return {
        "statusCode": 400,
        "status": False,
        "data": None,
        "message": message
    }

def internalServerErrorResponse(message):
    return {
        "statusCode": 500,
        "status": False,
        "data": None,
        "message": message
    }

def notFoundResponse(message):
    return {
        "statusCode": 404,
        "status": False,
        "data": None,
        "message": message
    }
