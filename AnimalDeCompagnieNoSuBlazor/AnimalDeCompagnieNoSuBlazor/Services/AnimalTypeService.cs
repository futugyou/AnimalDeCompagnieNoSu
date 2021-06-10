using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public class AnimalTypeService : IAnimalTypeService
    {
        private readonly HttpClient _animalClient;
        public AnimalTypeService(IHttpClientFactory httpClientFactory)
        {
            _animalClient = httpClientFactory.CreateClient("AnimalCenter");
        }
        public async Task<List<AnimalType>> GetAllAnimalTypes()
        {
            return await _animalClient.GetFromJsonAsync<List<AnimalType>>("api/animaltype");
        }

    }
}
