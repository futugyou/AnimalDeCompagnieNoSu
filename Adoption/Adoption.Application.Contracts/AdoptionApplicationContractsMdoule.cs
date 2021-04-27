using Adoption.Application.Contracts.Localization;
using System;
using Volo.Abp.Localization;
using Volo.Abp.Localization.ExceptionHandling;
using Volo.Abp.Modularity;
using Volo.Abp.VirtualFileSystem;

namespace Adoption.Application.Contracts
{
    [DependsOn(typeof(AbpLocalizationModule))]
    public class AdoptionApplicationContractsMdoule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {

            Configure<AbpVirtualFileSystemOptions>(options =>
            {
                options.FileSets.AddEmbedded<AdoptionApplicationContractsMdoule>();
            });

            Configure<AbpLocalizationOptions>(options =>
            {
                options.DefaultResourceType = typeof(AnimalInfoResource);
                options.Resources.Add<AnimalInfoResource>("en").AddVirtualJson("/Localization/AnimalInfo");
            });

            Configure<AbpExceptionLocalizationOptions>(options =>
            {
                options.MapCodeNamespace("animalInfo", typeof(AnimalInfoResource));
            });
        }
    }
}
