package com.myapp;

import io.vertx.core.Vertx;
import io.vertx.ext.web.Router;
import io.vertx.ext.web.handler.BodyHandler;
import routes.Routes;

public class Server {
    public static void main(String[] args) {
        Vertx vertx = Vertx.vertx();
        Router router = Routes.createRouter(vertx);

        // Menambahkan BodyHandler untuk menerima data POST jika diperlukan
        router.route().handler(BodyHandler.create());

        // Menjalankan server di port 8004
        vertx.createHttpServer()
            .requestHandler(router)
            .listen(8004, res -> {
                if (res.succeeded()) {
                    System.out.println("Server running at http://localhost:8004");
                } else {
                    System.err.println("Failed to start server: " + res.cause().getMessage());
                }
            });
    }
}
