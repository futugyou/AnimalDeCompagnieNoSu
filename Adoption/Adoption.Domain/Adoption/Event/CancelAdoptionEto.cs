using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.EventBus;

namespace Adoption.Domain.Adoption.DomainEvent
{
    [EventName("adoption.domain.adoption.canceled")]
    public class CancelAdoptionEto
    {
        public Guid AdoptionId { get; }
        public string Reason { get; }

        public CancelAdoptionEto(Guid adoptionId, string reason)
        {
            AdoptionId = adoptionId;
            Reason = reason;
        }
    }
}
