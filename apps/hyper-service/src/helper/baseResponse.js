import { BusinessException } from "./bussinesException.js";

export const BaseResponse = (res, resMessage, type, result = null) => {
  let response;
  let status = 200;

  switch (type) {
    case 'created':
      response = BusinessException.createdResponse(resMessage);
      status = 201;
      break;
    case 'success':
      response = BusinessException.successResponse(result, resMessage);
      status = 200;
      break;
    case 'unauthorized':
      response = BusinessException.unauthorizedResponse(resMessage);
      status = 403;
      break;
    case 'forbidden':
      response = BusinessException.unauthorizedResponse(resMessage); // Assuming the same method is reused
      status = 403;
      break;
    case 'internalServerError':
      response = BusinessException.internalServerErrorResponse();
      status = 500;
      break;
    default:
      response = BusinessException.successResponse(result, resMessage);
  }

  res.status(status).json(response);
};
