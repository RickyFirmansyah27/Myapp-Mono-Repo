export const BusinessException = {
    successResponse: (data, message) => {
      return {
        statusCode: 200,
        status: true,
        data,
        message,
      };
    },
  
    createdResponse: (message) => {
      return {
        statusCode: 201,
        status: true,
        message,
      };
    },
  
    badRequestResponse: (message, errors) => {
      return {
        statusCode: 400,
        status: false,
        error: message,
        errors,
      };
    },
  
    internalServerErrorResponse: () => {
      return {
        statusCode: 500,
        status: false,
        error: 'Internal server error',
      };
    },
  
    unauthorizedResponse: (message) => {
      return {
        statusCode: 401,
        status: false,
        error: message,
      };
    },
  };
  