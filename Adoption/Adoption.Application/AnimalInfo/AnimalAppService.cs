using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Domain.AnimalInfo;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Application.Services;
using Volo.Abp.Domain.Repositories;

namespace Adoption.Application.AnimalInfo
{
    public class AnimalAppService : ApplicationService, IAnimalAppService
    {
        private readonly IRepository<Animals> animalRepository;

        public AnimalAppService(IRepository<Animals> animalRepository)
        {
            this.animalRepository = animalRepository;
        }

        public async Task<List<AnimalDto>> GetAllAnimals()
        {
            var animals = await animalRepository.GetListAsync();
            var animalDtos = ObjectMapper.Map<List<Animals>, List<AnimalDto>>(animals);
            return animalDtos;
        }
    }
}
