using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Pages.Dashboard;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public class RescueService : IRescueService
    {
        private readonly HttpClient _httpClient;
        public RescueService(HttpClient httpClient) => _httpClient = httpClient;

        public async Task<List<ChartFunnelType>> GetFunnelDataAsync()
        {
            return await _httpClient.GetFromJsonAsync<List<ChartFunnelType>>("/data/funnel-data.json");
        }

        public async Task<List<ChartDataItem>> GetRescueDataAsync()
        {
            return await _httpClient.GetFromJsonAsync<List<ChartDataItem>>("/data/rescue.json");
        }

        public async Task<List<ChartPieType>> GetRescueTypeAsync()
        {
            return await _httpClient.GetFromJsonAsync<List<ChartPieType>>("/data/rescue-type.json");
        }
    }
}
