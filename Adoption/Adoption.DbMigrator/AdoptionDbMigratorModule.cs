using Adoption.Infrastruct.Data.DbMigrations;
using Microsoft.Extensions.DependencyInjection;
using Volo.Abp.Autofac;
using Volo.Abp.Modularity;

namespace Adoption.DbMigrator
{
    [DependsOn(
        typeof(AbpAutofacModule),
        typeof(AdoptionInfrastructDataDbMigrationsModule))]
    public class AdoptionDbMigratorModule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            context.Services.AddHostedService<DbMigratorHostedService>();
        }
    }
}
