using Volo.Abp.FluentValidation;
using Volo.Abp.Modularity;

namespace Adoption.Application.Contracts
{
    [DependsOn( 
        typeof(AbpFluentValidationModule))]
    public class AdoptionApplicationContractsMdoule : AbpModule
    {
    }
}
