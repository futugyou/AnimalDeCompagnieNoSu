using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Application.Contracts.Localization;
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
        private readonly IStringLocalizer<AnimalInfoResource> localizer;

        public AnimalAppService(IRepository<Animals> animalRepository, IStringLocalizer<AnimalInfoResource> localizer)
        {
            this.animalRepository = animalRepository;
            this.localizer = localizer;
        }

        public async Task<List<AnimalDto>> GetAllAnimals()
        {
            var animals = await animalRepository.GetListAsync();
            if (animals.Count == 0)
            {
                throw new UserFriendlyException(localizer["nodata"]);
            }
            var animalDtos = ObjectMapper.Map<List<Animals>, List<AnimalDto>>(animals);
            return animalDtos;
        }
    }
}
