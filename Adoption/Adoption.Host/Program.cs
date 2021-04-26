using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;

namespace Adoption.Host
{
    public class Program
    {
        public static void Main(string[] args)
        {
            CreateHostBuilder(args)
            .Build()
            //.InitializeDbContexts()
            .Run();
        }

        public static IHostBuilder CreateHostBuilder(string[] args)
        {
            var builder = Microsoft.Extensions.Hosting.Host
                .CreateDefaultBuilder(args)
                .ConfigureWebHostDefaults(webBuilder =>
                {
                    webBuilder.UseStartup<Startup>();
                })
                .UseAutofac();
            return builder;
        }
    }
}
