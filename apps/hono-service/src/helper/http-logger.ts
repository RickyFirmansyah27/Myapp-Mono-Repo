import { Context } from 'hono';
import { Logger } from './logger';

export const httpLogger = async (c: Context, next: () => Promise<void>) => {
  const headersObj: Record<string, string | string[]> = Object.entries(c.req.headers).reduce(
    (acc, [name, value]) => {
      if (value !== undefined) {
        acc[name] = value;
      }
      return acc;
    },
    {} as Record<string, string | string[]>
  );

  const start = performance.now();
  Logger.http({
    message: `Request | Method: ${c.req.method} | Headers: ${JSON.stringify(headersObj)} | URL: ${c.req.url}`,
  });

  try {
    await next();
  } catch (err) {
    Logger.error(`Error occurred: ${err}`);
    throw err;
  }

  const durationInMs = performance.now() - start;
  Logger.http({
    message: `Response | Method: ${c.req.method} | URL: ${c.req.url} | Status: ${c.res.status} | Duration: ${durationInMs.toFixed(2)} ms`,
  });
};
