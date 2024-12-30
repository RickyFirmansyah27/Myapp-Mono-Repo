//PM2 Config
module.exports = {
  apps: [
    {
      name: "actix-service",
      script: "make",
      args: "run",
      cwd: "apps/actix-service",
    },
    {
      name: "django-service",
      script: "make",
      args: "run",
      cwd: "apps/django-service",
    },
    {
      name: "dotnet-service",
      script: "make",
      args: "run",
      cwd: "apps/dotnet-service",
    },
    {
      name: "fiber-service",
      script: "make",
      args: "run",
      cwd: "apps/fiber-service",
    },
    {
      name: "hyper-service",
      script: "make",
      args: "run",
      cwd: "apps/hyper-service",
    },
    {
      name: "vertx-service",
      script: "make",
      args: "run",
      cwd: "apps/vertx-service",
    },
  ],
};
