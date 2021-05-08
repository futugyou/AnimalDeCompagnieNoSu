using Adoption.Domain.Adoption.Aggregate;
using Adoption.Domain.Shared.Adoption;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Specifications;

namespace Adoption.Domain.Adoption.Specification
{
    public class CatAdoptionSpecification : Specification<AdoptionInfo>
    {
        public override System.Linq.Expressions.Expression<Func<AdoptionInfo, bool>> ToExpression()
        {
            return info => info.Animal.AnimalType == AnimalType.Cat;
        }
    }
}
