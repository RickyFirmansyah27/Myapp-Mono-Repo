import logging
from rest_framework.decorators import api_view
from api.response.helper import BaseResponse

# Configure logging
logger = logging.getLogger(__name__)


@api_view(['GET'])
def getUsers(request):
    try:
        # Data sampel
        users = [
            {"id": 1, "name": "John Doe", "email": "john.doe@example.com"},
            {"id": 2, "name": "Jane Smith", "email": "jane.smith@example.com"},
            {"id": 3, "name": "Alice Johnson", "email": "alice.johnson@example.com"},
            {"id": 4, "name": "Bob Brown", "email": "bob.brown@example.com"},
            {"id": 5, "name": "Charlie White", "email": "charlie.white@example.com"},
        ]

        logger.info(f'[UserController] - Fetched all user successfully. {users}')

        # Mengembalikan respons dengan data sampel
        return BaseResponse('success', 'Successfully fetched sample users', users)
    except Exception as e:
        logger.error('[UserController] - Failed to fetch sample users.', exc_info=True)
        return BaseResponse('error', 'Failed to fetch sample users', str(e))
