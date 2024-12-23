package helper;

import io.vertx.core.json.JsonObject;
import io.vertx.ext.web.RoutingContext;

public class BaseResponse {

    public static void sendResponse(RoutingContext context, String resMessage, String responseType, Object result) {
        JsonObject response;
        int status;

        switch (responseType) {
            case "created":
                response = CreatedResponse(resMessage);
                status = 201;
                break;
            case "success":
                response = SuccessResponse(result, resMessage);
                status = 200;
                break;
            case "unauthorized":
            case "forbidden":
                response = UnauthorizedResponse(resMessage);
                status = 403;
                break;
            case "internalServerError":
                response = InternalServerErrorResponse();
                status = 500;
                break;
            default:
                response = SuccessResponse(result, resMessage);
                status = 200;
        }

        // Set status and send response
        context.response()
            .setStatusCode(status)
            .putHeader("content-type", "application/json")
            .end(response.encode());
    }

    private static JsonObject CreatedResponse(String message) {
        return new JsonObject()
            .put("status", "created")
            .put("message", message);
    }

    private static JsonObject SuccessResponse(Object result, String message) {
        return new JsonObject()
            .put("status", "success")
            .put("message", message)
            .put("data", result);
    }

    private static JsonObject UnauthorizedResponse(String message) {
        return new JsonObject()
            .put("status", "unauthorized")
            .put("message", message);
    }

    private static JsonObject InternalServerErrorResponse() {
        return new JsonObject()
            .put("status", "error")
            .put("message", "Internal Server Error");
    }
}
