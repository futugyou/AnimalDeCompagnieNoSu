using Adoption.Domain.Shared.Localization.Adoption;
using System;
using Volo.Abp.Localization;
using Volo.Abp.Localization.ExceptionHandling;
using Volo.Abp.Modularity;
using Volo.Abp.VirtualFileSystem;

namespace Adoption.Domain.Shared
{
    [DependsOn(
           typeof(AbpLocalizationModule))]
    public class AdoptionDomainSharedModule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            Configure<AbpVirtualFileSystemOptions>(options =>
            {
                options.FileSets.AddEmbedded<AdoptionDomainSharedModule>();
            });

            Configure<AbpLocalizationOptions>(options =>
            {
                options.Resources
                    .Add<AdoptionInfoResource>("en")
                    .AddVirtualJson("/Localization/Adoption");

                options.DefaultResourceType = typeof(AdoptionInfoResource);
            });

            Configure<AbpExceptionLocalizationOptions>(options =>
            {
                options.MapCodeNamespace("Adoption", typeof(AdoptionInfoResource));
            });
        }
    }
}
