namespace dotnet_service.Helpers
{
    public class BaseResponse<T>
    {
        public bool Success { get; set; }
        public string Message { get; set; }
        public T Data { get; set; }
        public List<string> Errors { get; set; }

        public BaseResponse()
        {
            Errors = new List<string>();
        }
    }
}
