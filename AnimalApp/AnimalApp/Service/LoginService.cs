namespace AnimalApp.Service;

public interface ILoginService
{
    Task<bool> Login(string username, string password);
}
public class LoginService : ILoginService
{
    public Task<bool> Login(string username, string password)
    {
        return Task.FromResult(true);
    }
}
