using Adoption.Application.Contracts;
using Adoption.Infrastruct.Data;
using System;
using Volo.Abp.Application;
using Volo.Abp.AutoMapper;
using Volo.Abp.Modularity;

namespace Adoption.Application
{
    [DependsOn(
        typeof(AbpDddApplicationModule),
        typeof(AbpAutoMapperModule),
        typeof(AdoptionInfrastructDataModule),
        typeof(AdoptionApplicationContractsMdoule))]
    public class AdoptionApplicationModule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            Configure<AbpAutoMapperOptions>(options =>
            {
                options.AddMaps<AdoptionApplicationModule>();
            });
        }
    }
}
