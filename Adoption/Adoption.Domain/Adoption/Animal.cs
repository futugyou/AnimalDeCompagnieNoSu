using Microsoft.AspNetCore.Routing.Constraints;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.Adoption
{
    public class Animal : Entity<int>
    {
        protected Animal()
        {

        }

        public Animal(string name, string type, string cardid, DateTime birthday)
        {
            CardId = cardid;
            Birthday = birthday;
            AnimalType = type;
            Name = name;
        }
        public string CardId { get; private set; }
        public DateTime Birthday { get; private set; }
        public string Name { get; private set; }
        public string AnimalType { get; private set; }
        public void AnimalCreateed()
        {
            // use in AuditedAggregateRoot<T>  not Entity<T>
            //AddDistributedEvent(new AnimalCreatedEto { CardId = CardId, Name = Name });
        }
    }
}
