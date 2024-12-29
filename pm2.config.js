//PM2 Config
module.exports = {
  apps: [
    {
      name: "vertx-service",
      script: "make",
      args: "run",
      cwd: "apps/vertx-service",
    },
    {
      name: "django-service",
      script: "make",
      args: "run",
      cwd: "apps/django-service",
    },
    {
      name: "actix-service",
      script: "make",
      args: "run",
      cwd: "apps/actix-service",
    },
  ],
};
