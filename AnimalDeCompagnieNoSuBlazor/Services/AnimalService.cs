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
        private readonly HttpClient _animalClient;
        public AnimalService(HttpClient httpClient, IHttpClientFactory httpClientFactory)
        {
            _animalClient = httpClientFactory.CreateClient("AnimalCenter");
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

        public async Task<AnimalUpdateModel> GetAnimalForUpdate(string aid)
        {
            try
            {
                return await _animalClient.GetFromJsonAsync<AnimalUpdateModel>($"api/animal/{aid}");
            }
            catch (Exception ex)
            {
                throw;
            }
        }

        public async Task<List<AnimalListViewModel>> GetAnimalList(AnimalListSearchModel request = null, PageModel pageModel = default)
        {
            try
            {
                string searchuri = "api/animal";
                if (request != null)
                {
                    searchuri += request.ToString();
                }
                if (pageModel != null)
                {
                    searchuri += searchuri.IndexOf("?") > 0 ? "&" + pageModel.ToString() : "?" + pageModel.ToString();
                }
                return await _animalClient.GetFromJsonAsync<List<AnimalListViewModel>>(searchuri);
            }
            catch (Exception ex)
            {
                throw;
            }
        }

        public async Task<AnimalViewModel> UpdateAnimal(AnimalUpdateModel animalUpdateModel)
        {
            try
            {
                var httpResponse = await _animalClient.PutAsJsonAsync("api/animal", animalUpdateModel);
                return await httpResponse.Content.ReadFromJsonAsync<AnimalViewModel>();
            }
            catch (Exception ex)
            {
                throw;
            }
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
