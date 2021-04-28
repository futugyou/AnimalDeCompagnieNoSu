using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Application.Contracts.Localization;
using Adoption.Application.Contracts.Localization.AnimalInfo;
using Adoption.Domain.AnimalInfo;
using Microsoft.Extensions.Localization;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp;
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
            LocalizationResource = typeof(AnimalInfoResource);
        }

        public async Task<bool> CreateAnimals(CreateAnimalDto animalDto)
        {
            var animal = ObjectMapper.Map<CreateAnimalDto, Animals>(animalDto);
            var result = await animalRepository.InsertAsync(animal, true);
            return result.Id > 0;
        }

        public async Task<List<AnimalDto>> GetAllAnimals()
        {
            var animals = await animalRepository.GetListAsync();
            if (animals.Count == 0)
            {
                throw new UserFriendlyException(L["nodata"]);
            }
            var animalDtos = ObjectMapper.Map<List<Animals>, List<AnimalDto>>(animals);
            return animalDtos;
        }
    }
}
