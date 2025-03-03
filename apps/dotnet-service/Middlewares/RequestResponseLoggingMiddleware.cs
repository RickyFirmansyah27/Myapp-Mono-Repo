namespace dotnet_service.Middlewares
{
    using System.Diagnostics;
    using System.IO;
    using System.Text;
    using System.Threading.Tasks;
    using Microsoft.AspNetCore.Http;
    using Microsoft.AspNetCore.Http.Extensions;
    using Microsoft.Extensions.Logging;

    public class RequestResponseLoggingMiddleware
    {
        private readonly RequestDelegate _next;
        private readonly ILogger<RequestResponseLoggingMiddleware> _logger;

        public RequestResponseLoggingMiddleware(RequestDelegate next, ILogger<RequestResponseLoggingMiddleware> logger)
        {
            _next = next;
            _logger = logger;
        }

        public async Task Invoke(HttpContext context)
        {
            var stopwatch = Stopwatch.StartNew();
            // Log Request
            _logger.LogInformation($"Request | Method: {context.Request.Method} | Headers: {FormatHeaders(context.Request.Headers)} | URL: {context.Request.GetDisplayUrl()}");

            if (context.Request.ContentLength > 0 && context.Request.Body.CanRead)
            {
                context.Request.EnableBuffering(); // Allows re-reading the request body
                using var reader = new StreamReader(context.Request.Body, Encoding.UTF8, leaveOpen: true);
                var requestBody = await reader.ReadToEndAsync();
                _logger.LogInformation($"📝 Request Body: {requestBody}");
                context.Request.Body.Position = 0; // Reset body stream position
            }

            // Capture Response Body
            var originalResponseBody = context.Response.Body;
            using var responseBodyStream = new MemoryStream();
            context.Response.Body = responseBodyStream;

            await _next(context); // Process request

            stopwatch.Stop();
            var elapsedMilliseconds = stopwatch.ElapsedMilliseconds;

            // Read Response Body
            context.Response.Body.Seek(0, SeekOrigin.Begin);
            var responseBody = await new StreamReader(context.Response.Body).ReadToEndAsync();
            _logger.LogInformation($"Response | Method: {context.Request.Method} | URL: {context.Request.GetDisplayUrl()} | Status: {context.Response.StatusCode} | Duration: {elapsedMilliseconds} ms");
            context.Response.Body.Seek(0, SeekOrigin.Begin);

            await responseBodyStream.CopyToAsync(originalResponseBody); // Write back response
        }

        private string FormatHeaders(IHeaderDictionary headers)
        {
            return string.Join("; ", headers.Select(h => $"{h.Key}: {h.Value}"));
        }
    }

}
