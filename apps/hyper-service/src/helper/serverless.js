// serverlessHandler.js
import { routes } from "../route/index.js";
import { Logger } from "./logger.js";

export default async function serverlessHandler(req, res) {
  try {
    const route = routes.find((route) => route.path === req.url);

    if (!route || !route.handler) {
      res.status(404).json({
        error: 'Not Found',
        message: 'Route not found',
      });
      return;
    }

    await route.handler(req, res);
  } catch (error) {
    Logger.error(`Error handling request: ${error.message}`);
    res.status(500).json({
      error: 'Internal Server Error',
      message: error.message,
    });
  }
}
