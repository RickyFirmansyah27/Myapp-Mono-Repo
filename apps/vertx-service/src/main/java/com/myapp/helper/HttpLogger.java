package com.myapp.helper;

import io.vertx.ext.web.RoutingContext;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class HttpLogger {

    private static final Logger logger = LoggerFactory.getLogger(HttpLogger.class);

    public static void logRequest(RoutingContext context) {
        logger.info("HttpLogger invoked for request: " + context.request().uri());
        long startTime = System.nanoTime();
    
        context.addEndHandler(v -> {
            logger.info("addEndHandler invoked for request: " + context.request().uri());
            long duration = (System.nanoTime() - startTime) / 1_000_000;
    
            String requestDetails = String.format("Request | Method: %s | Headers: %s | URL: %s", 
                context.request().method(), 
                context.request().headers(), 
                context.request().uri());
            logger.info(requestDetails);
    
            String responseDetails = String.format("Response | Method: %s | URL: %s | Status: %d | Duration: %.4f seconds", 
                context.request().method(), 
                context.request().uri(), 
                context.response().getStatusCode(), 
                duration / 1000.0);
            logger.info(responseDetails);
        });
    }    
}
