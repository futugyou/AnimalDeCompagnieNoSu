using Adoption.Domain.Shared;
using System;
using Volo.Abp.Domain;
using Volo.Abp.Modularity;

namespace Adoption.Domain
{
    [DependsOn(typeof(AbpDddDomainModule),
        typeof(AdoptionDomainSharedModule))]
    public class AdoptionDomainModule : AbpModule
    {
    }
}
