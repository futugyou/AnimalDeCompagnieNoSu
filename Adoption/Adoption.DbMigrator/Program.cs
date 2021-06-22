using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Serilog;
using Serilog.Events;
using Serilog.Sinks.Grafana.Loki;
using System;
using System.IO;
using System.Threading.Tasks;

namespace Adoption.DbMigrator
{
    class Program
    {
        static async Task<int> Main(string[] args)
        {
            try
            {
                await CreateHostBuilder(args).RunConsoleAsync();
                return 0;
            }
            catch (Exception ex)
            {
                Log.Fatal(ex, "Host terminated unexpectedly!");
                return 1;
            }
            finally
            {
                Log.Information("Host stopped!");
                Log.CloseAndFlush();
            }
        }

        private static readonly Action<HostBuilderContext, LoggerConfiguration> InitSerilog = (context, config) =>
        {
            var configuration = new ConfigurationBuilder()
                .SetBasePath(Directory.GetCurrentDirectory())
                .AddJsonFile("appsettings.json")
                .AddEnvironmentVariables()
                .Build();

            config
            .Enrich.FromLogContext()
#if DEBUG
            .MinimumLevel.Debug()
#else
            .MinimumLevel.Information()
#endif
            .MinimumLevel.Override("Microsoft", LogEventLevel.Information)
            .MinimumLevel.Override("Microsoft.EntityFrameworkCore", LogEventLevel.Warning)
            .Enrich.WithProperty("app", context.HostingEnvironment.ApplicationName)
            .WriteTo.GrafanaLoki(configuration["GrafanaLoki:Uri"])
            .WriteTo.Console();
        };

        public static IHostBuilder CreateHostBuilder(string[] args) =>
            Host.CreateDefaultBuilder(args)
            .UseAutofac()
            .UseSerilog(InitSerilog)
            .ConfigureServices((hostContext, services) =>
            {
                services.AddApplication<AdoptionDbMigratorModule>();
            });
    }
}
