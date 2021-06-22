using Microsoft.Extensions.DependencyInjection;
using System;
using Volo.Abp.Modularity;

namespace Adoption.Infrastruct.Data.DbMigrations
{
    [DependsOn(
           typeof(AdoptionInfrastructDataModule)
           )]
    public class AdoptionInfrastructDataDbMigrationsModule: AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            context.Services.AddAbpDbContext<AdoptionMigrationsDbContext>();
        }
    }
}
