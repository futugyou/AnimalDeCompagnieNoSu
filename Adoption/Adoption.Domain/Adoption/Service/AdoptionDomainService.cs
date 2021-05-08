using Adoption.Domain.Adoption.Aggregate;
using Adoption.Domain.Adoption.DomainEvent;
using Adoption.Domain.Adoption.Respository;
using Adoption.Domain.Shared.Adoption;
using System.Linq;
using System.Threading.Tasks;
using Volo.Abp;
using Volo.Abp.Domain.Services;
using Volo.Abp.EventBus.Distributed;

namespace Adoption.Domain.Adoption.Service
{
    public class AdoptionDomainService : DomainService, IAdoptionDomainService
    {
        private readonly IAdoptionRespository _adoptionRespository;
        private readonly IDistributedEventBus _distributedEventBus;
        public AdoptionDomainService(IAdoptionRespository adoptionRespository,
            IDistributedEventBus distributedEventBus)
        {
            _adoptionRespository = adoptionRespository;
            _distributedEventBus = distributedEventBus;
        }

        public async Task<bool> CreateAdoption(AdoptionInfo info)
        {
            var adoptionList = await _adoptionRespository.FindByAnimalCardIdAsync(info.Animal.CardId);
            if (adoptionList.Any(p => p.AdoptionStatus != AdoptionStatus.Rejected && p.AdoptionStatus != AdoptionStatus.Canceled))
            {
                throw new BusinessException(AdoptionDomainErrorCodes.AnimalHaveBeenAdoptioned).WithData("CardId", info.Animal.CardId);
            }
            await _adoptionRespository.InsertAsync(info);
            await _distributedEventBus.PublishAsync(new AdoptionCreated());
            return true;
        }
    }
}