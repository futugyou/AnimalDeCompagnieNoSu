using Adoption.Domain.Adoption.Aggregate;
using Adoption.Domain.Adoption.Respository;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Volo.Abp.Domain.Repositories.EntityFrameworkCore;
using Volo.Abp.EntityFrameworkCore;
using Volo.Abp.Specifications;

namespace Adoption.Infrastruct.Data.Adoption
{
    public class AdoptionRespository : EfCoreRepository<AdoptionDbContext, AdoptionInfo, Guid>, IAdoptionRespository
    {
        public AdoptionRespository(IDbContextProvider<AdoptionDbContext> dbContextProvider)
            : base(dbContextProvider)
        {

        }

        public async Task<List<AdoptionInfo>> FindByAdopterNameAsync(string name)
        {
            return await GetListAsync(p => p.Adopter.Name == name, true);
        }

        public async Task<List<AdoptionInfo>> GetAdoptionInfoAsync(ISpecification<AdoptionInfo> spec)
        {
            return await GetListAsync(spec.ToExpression(), true);
        }
        public async Task<List<AdoptionInfo>> FindByAnimalCardIdAsync(string cardId)
        {
            return await GetListAsync(p => p.Animal.CardId == cardId, true);
        }
    }
}
