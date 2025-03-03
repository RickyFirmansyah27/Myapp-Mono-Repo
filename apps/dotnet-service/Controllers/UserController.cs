using Microsoft.AspNetCore.Mvc;
using dotnet_service.Helpers;
using dotnet_service.Models;
using Microsoft.AspNetCore.Http.Extensions;

namespace dotnet_service.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class UserController : ControllerBase
    {
        private readonly ILogger<UserController> _logger;
        public UserController(ILogger<UserController> logger)
        {
            _logger = logger;
        }

        // GET: api/user
        [HttpGet]
        public ActionResult<BaseResponse<List<User>>> Get()
        {
            var response = new BaseResponse<List<User>>();
            // TODO: Implement logic to retrieve all users
            var data = new List<User> { new User("1", "Vincenza", "vincenza@email.com"), new User("2", "Lorine", "lorine@gmail.com") };
            response.Success = true;
            response.Message = "Get all users";
            response.Data = data;

            return Ok(response);
        }

        // GET: api/user/{id}
        [HttpGet("{id}")]
        public ActionResult<BaseResponse<User>> Get(int id)
        {
            var response = new BaseResponse<User>();
            response.Success = true;
            response.Message = $"Get user with id {id}";
            // TODO: Implement logic to retrieve user by id
            var data = new User(id.ToString(), "Skyes", "skyes@email.com");
            response.Data = data;

            return Ok(response);
        }

        // POST: api/user
        [HttpPost]
        public IActionResult Post([FromBody] User user)
        {
            // TODO: Implement logic to create a new user
            return Ok("Create user");
        }

        // PUT: api/user/{id}
        [HttpPut("{id}")]
        public IActionResult Put(int id, [FromBody] User user)
        {
            // TODO: Implement logic to update user by id
            return Ok($"Update user with id {id}");
        }

        // DELETE: api/user/{id}
        [HttpDelete("{id}")]
        public IActionResult Delete(int id)
        {
            // TODO: Implement logic to delete user by id
            return Ok($"Delete user with id {id}");
        }
    }
}
