using Microsoft.Extensions.DependencyInjection;
using System;
using Volo.Abp.AspNetCore;
using Volo.Abp.EntityFrameworkCore;
using Volo.Abp.Modularity;

namespace Adoption.Infrastruct.Data
{
    [DependsOn(typeof(AbpEntityFrameworkCoreModule))]
    public class AdoptionInfrastructDataModule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            var servers = context.Services;
            servers.AddAbpDbContext<AdoptionDbContext>(builder => builder.AddDefaultRepositories(true));
            Configure<AbpDbContextOptions>(opt =>
            {
                opt.UseMySQL();
            });
        }

    }
}
