using Adoption.Domain.AnimalInfo;
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


        public virtual Animal Animal { get; private set; }
        public virtual Adopter Adopter { get; private set; }
        public string AdoptionReasons { get; private set; }
        public string AdoptionStatus { get; private set; }
    }
}
