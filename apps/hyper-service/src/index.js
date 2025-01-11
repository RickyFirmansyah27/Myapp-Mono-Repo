import uWS from "uWebSockets.js";
import { routes } from "./route/index.js";
import { Logger } from "./helper/logger.js";

const port = 8105;

// Create the uWebSockets.js app
const app = uWS.App();

// Helper function to handle CORS
const corsHandler = (res) => {
  res.writeHeader("Access-Control-Allow-Origin", "*");
  res.writeHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
  res.writeHeader("Access-Control-Allow-Headers", "Content-Type");
};

// Handle preflight requests
app.options("/*", (res, req) => {
  corsHandler(res);
  res.writeStatus("204 No Content").end();
});

// Register routes dynamically
routes.forEach((route) => {
  const { method, path, handler } = route;

  // Add the route to the app
  app[method.toLowerCase()](path, (res, req) => {
    corsHandler(res); // Add CORS headers to every response

    // Use a helper to collect body data if needed
    if (["post", "put"].includes(method.toLowerCase())) {
      let buffer = "";
      res.onData((chunk, isLast) => {
        buffer += new TextDecoder("utf-8").decode(chunk);
        if (isLast) {
          try {
            const body = JSON.parse(buffer);
            handler(res, req, body); // Pass the body to the handler
          } catch (err) {
            res.writeStatus("400 Bad Request").end("Invalid JSON");
          }
        }
      });
    } else {
      handler(res, req); // Call the handler for GET, DELETE, etc.
    }
  });
});

// Start the server
app.listen(port, (listenSocket) => {
  if (listenSocket) {
    Logger.info(`[Hyper-Service] Server is running on port ${port}`);
  } else {
    Logger.error(`[Hyper-Service] Failed to start server on port ${port}`);
  }
});
