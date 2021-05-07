using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Repositories;
using Volo.Abp.Specifications;

namespace Adoption.Domain.Adoption.Respository
{
    public interface IAdoptionRespository : IRepository<AdoptionInfo, Guid>
    {
        Task<List<AdoptionInfo>> FindByAdopterNameAsync(string name);
        Task<List<AdoptionInfo>> GetAdoptionInfoAsync(ISpecification<AdoptionInfo> spec);
        Task<List<AdoptionInfo>> FindByAnimalCardIdAsync(string cardId);
    }
}
