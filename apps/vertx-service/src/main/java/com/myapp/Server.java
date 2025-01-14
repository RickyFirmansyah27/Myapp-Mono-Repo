package com.myapp;

import io.vertx.core.Vertx;
import io.vertx.ext.web.Router;
import io.vertx.ext.web.handler.BodyHandler;
import io.vertx.ext.web.handler.LoggerHandler;
import com.myapp.routes.Routes;

public class Server {
    public static void main(String[] args) {
        Vertx vertx = Vertx.vertx();
        Router router = Routes.createRouter(vertx);
        final int port = 8106;

        // Menambahkan BodyHandler untuk menerima data POST jika diperlukan
        router.route().handler(BodyHandler.create());

        // Menambahkan LoggerHandler untuk mencatat log request secara otomatis
        router.route().handler(LoggerHandler.create());

        // Menjalankan server di port 8106
        vertx.createHttpServer()
            .requestHandler(router)  // Menetapkan router sebagai request handler
            .listen(port, res -> {
                if (res.succeeded()) {
                    System.out.println("[Vertx-Service] Server is running on port " + port);
                } else {
                    System.err.println("Failed to start server: " + res.cause().getMessage());
                }
            });
    }
}
