using Adoption.Application.Contracts.AnimalInfo;
using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Adoption.Host.Controllers
{
    //TODO: remove this in future
    [Route("api/[controller]")]
    [ApiController]
    public class AnimalsController : ControllerBase
    {
        private readonly IAnimalAppService _animalAppService;

        public AnimalsController(IAnimalAppService animalAppService)
        {
            _animalAppService = animalAppService;
        }

        [HttpGet]
        public async Task<List<AnimalDto>> Get()
        {
            return await _animalAppService.GetAllAnimals();
        }

        [HttpPost]
        public async Task<bool> Post(CreateAnimalDto createAnimalDto)
        {
            return await _animalAppService.CreateAnimals(createAnimalDto);
        }
    }
}
