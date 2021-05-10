using Adoption.Domain.Adoption.Aggregate;
using Adoption.Domain.Shared.Adoption;
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
        Task<bool> ChangeAdoptionStatus(Guid id, AdoptionStatus targetStatus, string remarks = "");
    }
}
