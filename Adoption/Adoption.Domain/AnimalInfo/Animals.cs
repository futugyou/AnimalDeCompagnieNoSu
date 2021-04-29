using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.AnimalInfo
{
    public class Animals : Entity<int>
    {
        public string CardId { get; set; }
        public string Name { get; set; }
        public void AnimalCreateed()
        {
            // use in AuditedAggregateRoot<T>  not Entity<T>
            //AddDistributedEvent(new AnimalCreatedEto { CardId = CardId, Name = Name });
        }
    }
}
