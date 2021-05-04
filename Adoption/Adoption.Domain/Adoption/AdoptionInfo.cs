using Adoption.Domain.Adoption.DomainEvent;
using Adoption.Domain.Shared.Adoption;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.Adoption
{
    public class AdoptionInfo : AggregateRoot<Guid>
    {
        protected AdoptionInfo() { }

        public AdoptionInfo(Animal animal, Adopter adopter, string adoptionReasons)
        {
            Animal = animal;
            Adopter = adopter;
            AdoptionReasons = adoptionReasons;
            AdoptionStatus = AdoptionStatus.Auditing;
        }

        public virtual Animal Animal { get; private set; }
        public virtual Adopter Adopter { get; private set; }
        public virtual string AdoptionReasons { get; private set; }
        public virtual AdoptionStatus AdoptionStatus { get; private set; }
        public virtual string AdoptionResult { get; private set; }
        public void CancelAdoption(string cancelReason)
        {
            AdoptionStatus = AdoptionStatus.Cancel;
            AdoptionResult = cancelReason;
            AddDistributedEvent(new CancelAdoptionEto());
        }
        public void RejectAdoption(string rejectReason)
        {
            AdoptionStatus = AdoptionStatus.Reject;
            AdoptionResult = rejectReason;
            AddDistributedEvent(new RejectAdoptionEto());
        }
        public void AuditedAdoption(string auditedReason)
        {
            AdoptionStatus = AdoptionStatus.Audited;
            AdoptionResult = auditedReason;
            AddDistributedEvent(new AuditedAdoptionEto());
        }
        public void CompleteAdoption()
        {
            if (AdoptionStatus != AdoptionStatus.Audited)
            {
                //TODO: ADD CustomException
                throw new Exception("");
            }
            AdoptionStatus = AdoptionStatus.Complete;
            AddDistributedEvent(new CompleteAdoptionEto());
        }
    }
}
