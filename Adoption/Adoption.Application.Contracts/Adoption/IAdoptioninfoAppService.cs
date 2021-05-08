using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Application.Services;

namespace Adoption.Application.Contracts.Adoption
{
    public interface IAdoptioninfoAppService : IApplicationService
    {
        Task<bool> Create(CreateAdoptioninfoDto adoptioninfoDto);
    }
}
