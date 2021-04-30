using System.Collections.Generic;
using System.Threading.Tasks;
using Volo.Abp.Application.Services;

namespace Adoption.Application.Contracts.AnimalInfo
{
    public interface IAnimalAppService : IApplicationService
    {
        Task<List<AnimalDto>> GetAllAnimals();
        Task<bool> CreateAnimals(CreateAnimalDto animalDto);
    }
}
