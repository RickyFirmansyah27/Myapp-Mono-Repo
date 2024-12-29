namespace dotnet_service.Models
{
    public class User
    {
        public string Id { get; set; }
        public string Name { get; set; }
        public string Email { get; set; }
        public User(string id, string name, string email)
        {
            Id = id;
            Name = name;
            Email = email;
        }
        // Add more properties as needed
    }
}
