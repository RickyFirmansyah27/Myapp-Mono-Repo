package com.myapp.routes;

import com.myapp.controller.UserController;
import io.vertx.ext.web.Router;
import io.vertx.core.Vertx;

public class Routes {
    public static Router createRouter(Vertx vertx) {
        Router router = Router.router(vertx);

        // Definisikan rute dan hubungkan ke controller
        router.get("/users").handler(UserController::getUsers);
        router.get("/users/:id").handler(UserController::getUserById);
        router.delete("/users/:id").handler(UserController::deleteUserById);

        return router;
    }
}
