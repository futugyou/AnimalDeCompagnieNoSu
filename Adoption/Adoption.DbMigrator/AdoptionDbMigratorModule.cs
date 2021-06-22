using Adoption.Infrastruct.Data.DbMigrations;
using Volo.Abp.Autofac;
using Volo.Abp.BackgroundJobs;
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
            Configure<AbpBackgroundJobOptions>(options => options.IsJobExecutionEnabled = false);
        }
    }
}
