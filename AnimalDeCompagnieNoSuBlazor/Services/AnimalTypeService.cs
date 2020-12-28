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
        private readonly HttpClient _httpClient;
        public AnimalTypeService(HttpClient httpClient)
        {
            _httpClient = httpClient;
        }
        public async Task<string> GetAllAnimalTypes()
        {
            return await _httpClient.GetStringAsync("/data/animal-type.json");
        }
    }
}
