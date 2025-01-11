// src/middleware/error-handler.ts
import { Context } from 'hono';
import boom from '@hapi/boom';

export const ErrorHandler = (err: Error, c: Context) => {
  const boomError = boom.boomify(err);
  return c.json(
    {
      message: boomError.message,
      statusCode: boomError.output.statusCode,
    },
    boomError.output.statusCode
  );
};