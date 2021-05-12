using Adoption.Domain.Adoption.Aggregate;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.EventBus;

namespace Adoption.Domain.Adoption.DomainEvent
{
    [EventName("adoption.domain.adoption.rejected")]
    public class RejectAdoptionEto
    {
        public RejectAdoptionEto(AdoptionInfo adoptionInfo)
        {
            AdoptionInfo = adoptionInfo;
        }

        public AdoptionInfo AdoptionInfo { get; }
    }
}
