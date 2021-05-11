using Adoption.Application.Contracts.Adoption;
using Adoption.Domain.Adoption.Aggregate;
using Adoption.Domain.Adoption.Service;
using Adoption.Domain.Shared.Adoption;
using Adoption.Domain.Shared.Localization.Adoption;
using System;
using System.Threading.Tasks;
using Volo.Abp.Application.Services;

namespace Adoption.Application.Adoption
{
    public class AdoptioninfoAppService : ApplicationService, IAdoptioninfoAppService
    {
        private readonly IAdoptionDomainService _adoptionDomainService;

        public AdoptioninfoAppService(IAdoptionDomainService adoptionDomainService)
        {
            _adoptionDomainService = adoptionDomainService;
            LocalizationResource = typeof(AdoptionInfoResource);
        }

        public async Task<bool> Cancel(Guid id, string cancelReason)
        {
            return await _adoptionDomainService.ChangeAdoptionStatus(id, AdoptionStatus.Canceled, cancelReason);
        }

        public async Task<bool> Complete(Guid id)
        {
            return await _adoptionDomainService.ChangeAdoptionStatus(id, AdoptionStatus.Completed);
        }

        public async Task<bool> Create(CreateAdoptioninfoDto adoptioninfoDto)
        {
            var adoptioninfo = ObjectMapper.Map<CreateAdoptioninfoDto, AdoptionInfo>(adoptioninfoDto);
            await _adoptionDomainService.CreateAdoption(adoptioninfo);
            return true;
        }
    }
}
