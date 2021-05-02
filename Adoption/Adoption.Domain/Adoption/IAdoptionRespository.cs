using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Repositories;

namespace Adoption.Domain.Adoption
{
    public interface IAdoptionRespository : IRepository<AdoptionInfo, Guid>
    {
        Task<List<AdoptionInfo>> FindByAdopterNameAsync(string name);
    }
}
