using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Services;

namespace Adoption.Domain.Adoption.Service
{
    public interface IAdoptionDomainService : IDomainService
    {
        Task<bool> CreateAdoption(AdoptionInfo info);
    }
}
