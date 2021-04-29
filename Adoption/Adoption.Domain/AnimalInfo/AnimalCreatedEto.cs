using Microsoft.AspNetCore.Identity;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.EventBus;

namespace Adoption.Domain.AnimalInfo
{
    [EventName("animal.adoption.animal.created")]
    public class AnimalCreatedEto
    {
        public string Name { get; set; }
        public string CardId { get; set; }
    }
}
