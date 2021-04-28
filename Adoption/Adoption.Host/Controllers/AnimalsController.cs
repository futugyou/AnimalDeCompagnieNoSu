using Adoption.Application.AnimalInfo;
using Adoption.Application.Contracts.AnimalInfo;
using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Adoption.Host.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class AnimalsController : ControllerBase
    {
        private readonly IAnimalAppService animalAppService;

        public AnimalsController(IAnimalAppService animalAppService)
        {
            this.animalAppService = animalAppService;
        }

        [HttpGet]
        public async Task<List<AnimalDto>> Get()
        {
            return await animalAppService.GetAllAnimals();
        }

        [HttpPost]
        public async Task<bool> Post(CreateAnimalDto createAnimalDto)
        {
            return await animalAppService.CreateAnimals(createAnimalDto);
        }
    }
}
