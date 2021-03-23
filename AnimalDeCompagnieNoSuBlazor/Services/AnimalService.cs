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
        private readonly HttpClient _animalClient;
        public AnimalService(HttpClient httpClient, IHttpClientFactory httpClientFactory)
        {
            _animalClient = httpClientFactory.CreateClient("AnimalCenter");
            _httpClient = httpClient;
        }

        public async Task<AnimalViewModel> GetAnimal(string aid)
        {
            try
            {
                var httpResponse = await _animalClient.GetAsync($"api/animal/{aid}");
                return await httpResponse.Content.ReadFromJsonAsync<AnimalViewModel>();
            }
            catch (Exception ex)
            {
                throw;
            }
            //return await _httpClient.GetFromJsonAsync<AnimalViewModel>("data/animal.json");
        }

        public async Task<AnimalUpdateModel> GetAnimalForUpdate(int aid)
        {
            return await _httpClient.GetFromJsonAsync<AnimalUpdateModel>("data/animal-for-update.json");
        }

        public async Task<List<AnimalListViewModel>> GetAnimalList()
        {
            try
            {
                var httpResponse = await _animalClient.GetAsync("api/animal");
                return await httpResponse.Content.ReadFromJsonAsync<List<AnimalListViewModel>>();
            }
            catch (Exception ex)
            {
                throw;
            }
            //return await _httpClient.GetFromJsonAsync<List<AnimalListViewModel>>("data/animal-list.json");
        }

        public async Task<AnimalViewModel> UpdateAnimal(AnimalUpdateModel animalUpdateModel)
        {
            await Task.Delay(3000);
            return await _httpClient.GetFromJsonAsync<AnimalViewModel>("data/animal.json");
        }
    }
}
