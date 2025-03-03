using dotnet_service.Helpers;
using dotnet_service.Middlewares;
var builder = WebApplication.CreateBuilder(args);

builder.Services.AddControllers();

// Add services to the container.
builder.Services.AddEndpointsApiExplorer();
builder.Logging.ClearProviders();
builder.Logging.AddConsole();  // Logs to console
builder.Logging.AddDebug();    // Logs to Debug output in IDE
var app = builder.Build();

app.UseMiddleware<RequestResponseLoggingMiddleware>();

app.MapControllers();

System.Console.WriteLine("api path is /api/user");

app.MapGet("/", () =>
{
    var response = new BaseResponse<string>();
    response.Success = true;
    response.Message = "Welcome to dotnet service";
    response.Data = "api path is /api/user";
    return response;
});

app.Run();