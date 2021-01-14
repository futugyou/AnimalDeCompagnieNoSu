using AnimalDeCompagnieNoSuBlazor.Services;
using AntDesign.Charts;
using Microsoft.AspNetCore.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Dashboard
{
    public partial class Rescue
    {
        [Inject]
        private IRescueService RescueService { get; set; }
        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            var data = await RescueService.GetRescueDataAsync();
            await _saleChart.ChangeData(data);
        }

        private readonly ColumnConfig _saleChartConfig = new ColumnConfig
        {
            Title = new AntDesign.Charts.Title
            {
                Visible = true,
                Text = "Stores Sales Trend"
            },
            ForceFit = true,
            Padding = "auto",
            XField = "x",
            YField = "y"
        };
        private IChartComponent _saleChart;
        [Parameter]
        public SaleItem[] Items { get; set; } =
       {
            new SaleItem {Id = 1, Title = "Gongzhuan No.0 shop", Total = "323,234"},
            new SaleItem {Id = 2, Title = "Gongzhuan No.1 shop", Total = "323,234"},
            new SaleItem {Id = 3, Title = "Gongzhuan No.2 shop", Total = "323,234"},
            new SaleItem {Id = 4, Title = "Gongzhuan No.3 shop", Total = "323,234"},
            new SaleItem {Id = 5, Title = "Gongzhuan No.4 shop", Total = "323,234"},
            new SaleItem {Id = 6, Title = "Gongzhuan No.5 shop", Total = "323,234"},
            new SaleItem {Id = 7, Title = "Gongzhuan No.6 shop", Total = "323,234"}
        };
    }
    public class SaleItem
    {
        public int Id { get; set; }
        public string Title { get; set; }
        public string Total { get; set; }
    }
    public class ChartDataItem
    {
        public string X { get; set; }
        public int Y { get; set; }
    }
}
