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
        private readonly HttpClient _animalClient;
        public AnimalEventService(IHttpClientFactory httpClientFactory)
        {
            _animalClient = httpClientFactory.CreateClient("AnimalCenter");
        }
        public async Task<List<AnimalEvent>> GetBigEventByAnimalId(string animalId)
        {
            try
            {
                return await _animalClient.GetFromJsonAsync<List<AnimalEvent>>($"api/animal/{animalId}/event");
            }
            catch (Exception ex)
            {
                throw;
            }
        }
    }
}
