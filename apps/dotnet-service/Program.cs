using dotnet_service.Helpers;
var builder = WebApplication.CreateBuilder(args);

builder.Services.AddControllers();

// Add services to the container.
builder.Services.AddEndpointsApiExplorer();

var app = builder.Build();

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