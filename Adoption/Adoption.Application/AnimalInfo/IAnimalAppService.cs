using Adoption.Application.Contracts.AnimalInfo;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Application.Services;

namespace Adoption.Application.AnimalInfo
{
    public interface IAnimalAppService : IApplicationService
    {
        Task<List<AnimalDto>> GetAllAnimals();
    }
}
