using Adoption.Infrastruct.Data.DbMigrations;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System;
using System.Threading;
using System.Threading.Tasks;
using Volo.Abp;
using Volo.Abp.Data;

namespace Adoption.DbMigrator
{
    public class DbMigratorHostedService : IHostedService
    {
        private readonly IAbpApplicationWithExternalServiceProvider _application;
        private readonly IServiceProvider _serviceProvider;

        public DbMigratorHostedService(
            IAbpApplicationWithExternalServiceProvider application,
            IServiceProvider serviceProvider)
        {
            _application = application;
            _serviceProvider = serviceProvider;
        }

        public async Task StartAsync(CancellationToken cancellationToken)
        {
            _application.Initialize(_serviceProvider);

            await _application.ServiceProvider.GetRequiredService<AdoptionDbSchemaMigrator>().MigrateAsync();
            await _application.ServiceProvider.GetRequiredService<IDataSeeder>().SeedAsync();

            IHostApplicationLifetime lifetime = _application.ServiceProvider.GetRequiredService<IHostApplicationLifetime>();
            lifetime.StopApplication();
        }

        public Task StopAsync(CancellationToken cancellationToken)
        {
            _application.Shutdown();
            return Task.CompletedTask;
        }
    }
}
