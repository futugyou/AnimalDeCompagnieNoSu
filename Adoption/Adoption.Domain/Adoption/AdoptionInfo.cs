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
         

        public virtual Animal Animal { get; set; }
        public virtual Adopter Adopter { get; set; }
        public string AdoptionReasons { get; set; }
    }
}
