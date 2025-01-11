export const BusinessException = {
  successResponse: (data: string | object | Array<object>, message: string) => {
    return {
      statusCode: 200,
      status: true,
      data,
      message
    };
  },

  createdResponse: (message: string) => {
    return {
      statusCode: 201,
      status: true,
      message
    };
  },

  badRequestResponse: (message: string, errors: any) => {
    return {
      statusCode: 400,
      status: false,
      error: message,
      errors
    };
  },

  internalServerErrorResponse: () => {
    return {
      statusCode: 500,
      status: false,
      error: 'Internal server error'
    };
  },

  unauthorizedResponse: (message: string) => {
    return {
      statusCode: 401,
      status: false,
      error: message
    };
  }
};
