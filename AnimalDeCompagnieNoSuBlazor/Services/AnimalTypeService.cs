using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Net.Http.Json;
using System.Text.Json;
using System.Text.Json.Serialization;
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
            List<AnimalType> result = new List<AnimalType>();
            try
            {
                var request = new HttpRequestMessage(HttpMethod.Get, "api/animaltype");
                request.Headers.Accept.Add(new MediaTypeWithQualityHeaderValue("application/protobuf"));
                var message = await _animalClient.SendAsync(request);
                var responsestring = await message.Content.ReadAsStringAsync();
                var response = JsonSerializer.Deserialize<AniamlTypeResponse>(responsestring);
                result = response.Item;
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex.Message);
            }
            return result;
        }

    }
}
