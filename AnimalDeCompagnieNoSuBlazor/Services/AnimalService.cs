using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
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
                return await _animalClient.GetFromJsonAsync<AnimalViewModel>($"api/animal/{aid}");
            }
            catch (Exception ex)
            {
                throw;
            }
        }

        public async Task<AnimalUpdateModel> GetAnimalForUpdate(int aid)
        {
            return await _httpClient.GetFromJsonAsync<AnimalUpdateModel>("data/animal-for-update.json");
        }

        public async Task<List<AnimalListViewModel>> GetAnimalList()
        {
            try
            {
                return await _animalClient.GetFromJsonAsync<List<AnimalListViewModel>>("api/animal");
            }
            catch (Exception ex)
            {
                throw;
            }
        }

        public async Task<AnimalViewModel> UpdateAnimal(AnimalUpdateModel animalUpdateModel)
        {
            await Task.Delay(3000);
            return await _httpClient.GetFromJsonAsync<AnimalViewModel>("data/animal.json");
        }

        public async Task UpdateAnimalAvatar(AnimalAvatarUploadModel animalAvatarUploadNodel)
        {
            try
            {
                var httpResponse = await _animalClient.PutAsJsonAsync("api/animal", animalAvatarUploadNodel);
                await httpResponse.Content.ReadAsStringAsync();
            }
            catch (Exception ex)
            {
                throw;
            }
        }
    }
}
