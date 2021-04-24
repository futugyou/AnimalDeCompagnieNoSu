using System;
using Volo.Abp.Domain;
using Volo.Abp.Modularity;

namespace Adoption.Domain
{
    [DependsOn(typeof(AbpDddDomainModule))]
    public class AdoptionDomainModule : AbpModule
    {
    }
}
