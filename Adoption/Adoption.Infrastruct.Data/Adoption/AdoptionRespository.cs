using Adoption.Domain.Adoption;
using Microsoft.EntityFrameworkCore;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Repositories.EntityFrameworkCore;
using Volo.Abp.EntityFrameworkCore;

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
    }
}
