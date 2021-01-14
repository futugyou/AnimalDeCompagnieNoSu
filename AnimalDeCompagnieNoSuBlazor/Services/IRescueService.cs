using AnimalDeCompagnieNoSuBlazor.Pages.Dashboard;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    interface IRescueService
    {
        Task<List<ChartDataItem>> GetRescueDataAsync();
    }
}
