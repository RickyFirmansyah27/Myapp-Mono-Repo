package com.myapp.routes;

import com.myapp.controller.UserController;
import io.vertx.ext.web.Router;
import io.vertx.core.Vertx;

public class Routes {
    public static Router createRouter(Vertx vertx) {
        Router router = Router.router(vertx);
        var basePrefix = "/api";

        // Index
        router.get("/").handler(UserController::Index);

        // Definisikan rute dan hubungkan ke controller
        router.get(basePrefix + "/users").handler(UserController::getUsers);
        router.post(basePrefix + "/users").handler(UserController::createUser);
        router.get(basePrefix + "/users/:id").handler(UserController::getUserById);
        router.put(basePrefix + "/users/:id").handler(UserController::updateUser);
        router.delete(basePrefix + "/users/:id").handler(UserController::deleteUserById);

        return router;
    }
}
