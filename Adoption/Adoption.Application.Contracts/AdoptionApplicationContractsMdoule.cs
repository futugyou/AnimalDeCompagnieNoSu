using Adoption.Application.Contracts.Localization;
using Adoption.Application.Contracts.Localization.AnimalInfo;
using System;
using Volo.Abp.FluentValidation;
using Volo.Abp.Localization;
using Volo.Abp.Localization.ExceptionHandling;
using Volo.Abp.Modularity;
using Volo.Abp.VirtualFileSystem;

namespace Adoption.Application.Contracts
{
    [DependsOn(
        typeof(AbpLocalizationModule),
        typeof(AbpFluentValidationModule))]
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
                options.Resources
                    .Add<AnimalInfoResource>("en")
                    //.AddBaseTypes(typeof(AbpValidationResource))
                    .AddVirtualJson("/Localization/AnimalInfo");

                options.DefaultResourceType = typeof(AnimalInfoResource);
            });

            Configure<AbpExceptionLocalizationOptions>(options =>
            {
                options.MapCodeNamespace("animalInfo", typeof(AnimalInfoResource));
            });
        }
    }
}
