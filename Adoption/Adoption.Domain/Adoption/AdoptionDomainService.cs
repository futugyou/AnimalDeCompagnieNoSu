using Adoption.Domain.Adoption.DomainEvent;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Services;

namespace Adoption.Domain.Adoption
{
    public class AdoptionDomainService : IDomainService 
    {
        private readonly IAdoptionRespository adoptionRespository;
        private readonly IDistributedEventBus distributedEventBus;
        public AdoptionDomainService(IAdoptionRespository  adoptionRespository,
            IDistributedEventBus distributedEventBus)
        {
            adoptionRespository = adoptionRespository;
            distributedEventBus = distributedEventBus;
        }

        public async Task<bool> CreateAdoption(AdoptionInfo info) 
        {
            var adoptionList = await adoptionRespository.FindByAnimalCardIdAsync(info.Animal.CardId);
            if (adoptionList.Count(p=>p.AdoptionStatus != AdoptionStatus.Rejected && p.AdoptionStatus != AdoptionStatus.Canceled) > 0)
            {
                //TODO: ADD CustomException
                throw new Exception("this pet have been adoption by other person");
            }
            await adoptionRespository.InsertAsync(info);
            await distributedEventBus.PublishAsync(new AdoptionCreated());
            return true;
        }
    }
}