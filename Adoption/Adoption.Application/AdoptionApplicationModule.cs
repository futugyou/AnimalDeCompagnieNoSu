using Adoption.Application.Contracts;
using Adoption.Infrastruct.Data;
using Rebus.Config;
using System;
using Volo.Abp.Application;
using Volo.Abp.AutoMapper;
using Volo.Abp.EventBus.Rebus;
using Volo.Abp.Modularity;

namespace Adoption.Application
{
    [DependsOn(
        typeof(AbpDddApplicationModule),
        typeof(AbpAutoMapperModule),
        typeof(AdoptionInfrastructDataModule),
        typeof(AdoptionApplicationContractsMdoule),
        typeof(AbpEventBusRebusModule))]
    public class AdoptionApplicationModule : AbpModule
    {

        public override void PreConfigureServices(ServiceConfigurationContext context)
        {
            PreConfigure<AbpRebusEventBusOptions>(options =>
            {
                options.InputQueueName = "animal.eventbus";
                options.Configurer = rebusConfigurer =>
                {
                    rebusConfigurer.Transport(t => t.UseRabbitMq("amqp://localhost", "animal.eventbus"));
                };
            });
        }
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            Configure<AbpAutoMapperOptions>(options =>
            {
                options.AddMaps<AdoptionApplicationModule>();
            });
        }
    }
}
