using Adoption.Application.Contracts;
using Adoption.Infrastruct.Data;
using System;
using Volo.Abp.Application;
using Volo.Abp.Modularity;

namespace Adoption.Application
{
    [DependsOn(
        typeof(AbpDddApplicationModule),
        typeof(AdoptionInfrastructDataModule),
        typeof(AdoptionApplicationContractsMdoule))]
    public class AdoptionApplicationModule : AbpModule
    {
    }
}
