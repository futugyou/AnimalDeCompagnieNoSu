using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public class AnimalService : IAnimalService
    {
        private readonly HttpClient _httpClient;
        public AnimalService(HttpClient httpClient)
        {
            _httpClient = httpClient;
        }

        public async Task<AnimalViewModel> GetAnimal()
        {
            return await _httpClient.GetFromJsonAsync<AnimalViewModel>("data/animal.json");
        }

        public async Task<List<AnimalListViewModel>> GetAnimalList()
        {
            return await _httpClient.GetFromJsonAsync<List<AnimalListViewModel>>("data/animal-list.json");
        }

        public async Task<AnimalViewModel> UpdateAnimal(AnimalUpdateModel animalUpdateModel)
        {
            return await _httpClient.GetFromJsonAsync<AnimalViewModel>("data/animal.json");
        }
    }
}
