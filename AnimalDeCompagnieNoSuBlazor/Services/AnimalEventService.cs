using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public class AnimalEventService : IAnimalEventService
    {
        private readonly HttpClient _httpClient;
        public AnimalEventService(HttpClient httpClient)
        {
            _httpClient = httpClient;
        }
        public async Task<List<AnimalEvent>> GetBigEventByAnimalId(int animalId)
        {
            return await _httpClient.GetFromJsonAsync<List<AnimalEvent>>("/data/animal-event.json");
        }
    }
}
