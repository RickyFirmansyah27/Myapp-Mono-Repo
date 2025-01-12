// PM2 Config
module.exports = {
  apps: [
    {
      name: "actix-service",
      script: "make",
      args: "run",
      cwd: "apps/actix-service",
      log_date_format: "YYYY-MM-DD HH:mm:ss",
      output: "logs/actix-service-out.log",  // Log output
      error: "logs/actix-service-error.log", // Log error
    },
    {
      name: "django-service",
      script: "make",
      args: "run",
      cwd: "apps/django-service",
      log_date_format: "YYYY-MM-DD HH:mm:ss",
      output: "logs/django-service-out.log",  // Log output
      error: "logs/django-service-error.log", // Log error
    },
    {
      name: "dotnet-service",
      script: "make",
      args: "run",
      cwd: "apps/dotnet-service",
      log_date_format: "YYYY-MM-DD HH:mm:ss",
      output: "logs/dotnet-service-out.log",  // Log output
      error: "logs/dotnet-service-error.log", // Log error
    },
    {
      name: "fiber-service",
      script: "make",
      args: "run",
      cwd: "apps/fiber-service",
      log_date_format: "YYYY-MM-DD HH:mm:ss",
      output: "logs/fiber-service-out.log",  // Log output
      error: "logs/fiber-service-error.log", // Log error
    },
    {
      name: "hono-service",
      script: "make",
      args: "start",
      cwd: "apps/hono-service",
      log_date_format: "YYYY-MM-DD HH:mm:ss",
      output: "logs/hono-service-out.log",  // Log output
      error: "logs/hono-service-error.log", // Log error
    },
    {
      name: "vertx-service",
      script: "make",
      args: "run",
      cwd: "apps/vertx-service",
      log_date_format: "YYYY-MM-DD HH:mm:ss",
      output: "logs/vertx-service-out.log",  // Log output
      error: "logs/vertx-service-error.log", // Log error
    },
  ],
};
