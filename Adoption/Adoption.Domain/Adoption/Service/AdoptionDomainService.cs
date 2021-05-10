using Adoption.Domain.Adoption.Aggregate;
using Adoption.Domain.Adoption.DomainEvent;
using Adoption.Domain.Adoption.Respository;
using Adoption.Domain.Shared.Adoption;
using System;
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

        public async Task<bool> ChangeAdoptionStatus(Guid id, AdoptionStatus targetStatus, string remarks = "")
        {
            var adoptioninfo = await _adoptionRespository.FindAsync(id);
            if (adoptioninfo == null)
            {
                throw new BusinessException(AdoptionDomainErrorCodes.AdoptionNotFound).WithData("id", id);
            }
            switch (targetStatus)
            {
                case AdoptionStatus.Audited:
                    adoptioninfo.AuditedAdoption(remarks);
                    break;
                case AdoptionStatus.Rejected:
                    adoptioninfo.RejectAdoption(remarks);
                    break;
                case AdoptionStatus.Canceled:
                    adoptioninfo.CancelAdoption(remarks);
                    break;
                case AdoptionStatus.Completed:
                    adoptioninfo.CompleteAdoption();
                    break;
                default:
                    throw new BusinessException(AdoptionDomainErrorCodes.TargetStatusNotSupport).WithData("status", targetStatus);
            }
            await _adoptionRespository.UpdateAsync(adoptioninfo);
            return true;
        }

        public async Task<bool> CreateAdoption(AdoptionInfo info)
        {
            var adoptionList = await _adoptionRespository.FindByAnimalCardIdAsync(info.Animal.CardId);
            if (adoptionList.Any(p => p.AdoptionStatus != AdoptionStatus.Rejected && p.AdoptionStatus != AdoptionStatus.Canceled))
            {
                throw new BusinessException(AdoptionDomainErrorCodes.AnimalHaveBeenAdoptioned).WithData("CardId", info.Animal.CardId);
            }
            info.SetId(GuidGenerator.Create());
            await _adoptionRespository.InsertAsync(info);
            await _distributedEventBus.PublishAsync(new AdoptionCreated());
            return true;
        }
    }
}