using Adoption.Domain.Adoption.DomainEvent;
using Adoption.Domain.Shared.Adoption;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Services;
using Volo.Abp.EventBus.Distributed;

namespace Adoption.Domain.Adoption
{
    public class AdoptionDomainService : IDomainService
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
            if (adoptionList.Count(p => p.AdoptionStatus != AdoptionStatus.Rejected && p.AdoptionStatus != AdoptionStatus.Canceled) > 0)
            {
                //TODO: ADD CustomException
                throw new Exception("this pet have been adoption by other person");
            }
            await _adoptionRespository.InsertAsync(info);
            await _distributedEventBus.PublishAsync(new AdoptionCreated());
            return true;
        }
    }
}