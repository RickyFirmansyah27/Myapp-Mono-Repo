import HyperExpress from "hyper-express";
import cors from "cors";
import { routes } from "./route/index.js";
import { Logger } from "./helper/logger.js";

const app = new HyperExpress.Server();
const port = 8105;

app.use(cors({
  origin: '*',
}));

// Register all routes dynamically
routes.forEach(route => {
  app.use(route.path, route.handler);
});


app.listen(port, async () => {
  try {
      Logger.info(`[Hyper-Service] Server is running on port ${port}`);
  } catch (error) {
      if (error instanceof Error) {
          Logger.error(
              `Error starting server: Message: ${error.message} | Stack: ${error.stack}`
          );
      } else {
          Logger.error(`Error starting server: ${String(error)}`);
      }
  }
});
