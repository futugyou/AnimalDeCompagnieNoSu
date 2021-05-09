using Adoption.Domain.Shared.Adoption;
using Microsoft.AspNetCore.Routing.Constraints;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.Adoption.Aggregate
{
    public class Animal : Entity<int>
    {
        protected Animal() { }

        internal Animal(string name, AnimalType type, string cardid, DateTime birthday)
        {
            CardId = cardid;
            Birthday = birthday;
            AnimalType = type;
            Name = name;
            Age = (DateTime.UtcNow - birthday).Days / 365;
        }
        public virtual string CardId { get; private set; }
        public virtual DateTime Birthday { get; private set; }
        public virtual string Name { get; private set; }
        public virtual AnimalType AnimalType { get; private set; }
        public int Age { get; private set; }
        public void AnimalCreateed()
        {
            // use in AuditedAggregateRoot<T>  not Entity<T>
            //AddDistributedEvent(new AnimalCreatedEto { CardId = CardId, Name = Name });
        }
    }
}
