import { BusinessException } from './business-exception';
import { Context } from 'hono';

export const BaseResponse = (
  c: Context,
  resMessage: string,
  type: string,
  result: any = null,
) => {
  let responseData;
  let status = 200; // Default status code

  switch (type) {
    case 'created':
      responseData = { statusCode: 201, status: true, message: resMessage };
      break;
    case 'success':
      responseData = { statusCode: 200, status: true, message: resMessage, data: result };
      break;
    case 'unauthorized':
      responseData = { statusCode: 403, status: false, error: resMessage };
      break;
    case 'internalServerError':
      responseData = { statusCode: 500, status: false, error: resMessage };
      break;
    default:
      responseData = { statusCode: 200, status: true, message: resMessage, data: result };
      break;
  }

  return c.json(responseData, { status }); 
};
