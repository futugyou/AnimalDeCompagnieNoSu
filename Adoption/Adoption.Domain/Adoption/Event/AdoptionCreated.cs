using Adoption.Domain.Adoption.Aggregate;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.EventBus;

namespace Adoption.Domain.Adoption.DomainEvent
{
    [EventName("adoption.domain.adoption.created")]
    public class AdoptionCreated
    {
        public AdoptionCreated(AdoptionInfo info)
        {
            Info = info;
        }

        public AdoptionInfo Info { get; }
    }
}
