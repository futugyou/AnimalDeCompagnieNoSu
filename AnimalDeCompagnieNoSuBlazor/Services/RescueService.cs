using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Pages.Dashboard;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Net.Http.Json;
using System.Text.Json;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public class RescueService : IRescueService
    {
        private readonly HttpClient _httpClient;
        private readonly HttpClient _animalClient;

        public RescueService(HttpClient httpClient, IHttpClientFactory httpClientFactory)
        {
            _animalClient = httpClientFactory.CreateClient("AnimalCenter");
            _httpClient = httpClient;
        }

        public async Task<List<ChartFunnelType>> GetFunnelDataAsync()
        {
            return await _httpClient.GetFromJsonAsync<List<ChartFunnelType>>("/data/funnel-data.json");
        }

        public async Task<List<ChartDataItem>> GetRescueDataAsync()
        {
            return await _httpClient.GetFromJsonAsync<List<ChartDataItem>>("/data/rescue.json");
        }

        public async Task<List<ChartPieageType>> GetRescueAgeRangAsync()
        {
            try
            {
                var httpResponse = await _animalClient.GetAsync("api/animalreport");
                List<RescueClassificationResponse> Rescueh = await httpResponse.Content.ReadFromJsonAsync<List<RescueClassificationResponse>>();
                return Rescueh.Select(p => new ChartPieageType { Count = p.Count, Agerang = p.Classification + " year" }).OrderBy(p => p.Agerang).ToList();
            }
            catch (Exception ex)
            {

                throw;
            }
            //return await _httpClient.GetFromJsonAsync<List<ChartPieageType>>("/data/rescue-age.json");
        }

        public async Task<List<ChartPieType>> GetRescueTypeAsync()
        {
            try
            {
                var httpResponse = await _animalClient.GetAsync("api/animalreport?rescue_classification=classic");
                List<RescueClassificationResponse> Rescueh = await httpResponse.Content.ReadFromJsonAsync<List<RescueClassificationResponse>>();
                return Rescueh.Select(p => new ChartPieType { Count = p.Count, Type = p.Classification }).ToList();
            }
            catch (Exception ex)
            {

                throw;
            }
            //return await _httpClient.GetFromJsonAsync<List<ChartPieType>>("/data/rescue-type.json");
        }
    }
}
