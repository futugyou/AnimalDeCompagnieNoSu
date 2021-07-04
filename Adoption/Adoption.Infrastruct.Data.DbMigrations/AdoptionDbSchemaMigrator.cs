using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.DependencyInjection;

namespace Adoption.Infrastruct.Data.DbMigrations
{
    public class AdoptionDbSchemaMigrator : ITransientDependency
    {
        private readonly IServiceProvider _serviceProvider;

        public AdoptionDbSchemaMigrator(
            IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        public async Task MigrateAsync()
        {
            /* We intentionally resolving the AdoptionMigrationsDbContext
             * from IServiceProvider (instead of directly injecting it)
             * to properly get the connection string of the current tenant in the
             * current scope.
             */
            //var context = _serviceProvider.GetRequiredService<AdoptionMigrationsDbContext>();
            //var migration = context.GetInfrastructure().GetService<IMigrator>();
            //await migration.MigrateAsync();
            await _serviceProvider
                .GetRequiredService<AdoptionMigrationsDbContext>()
                .Database
                .MigrateAsync();
        }
    }
}
