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

        private IChartComponent _rescueChart;
        private IChartComponent _funnelChart;
        private IChartComponent _liquidChart;

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            var data = await RescueService.GetRescueDataAsync();
            await _rescueChart.ChangeData(data);
            var funnelData = await RescueService.GetFunnelDataAsync();
            await _funnelChart.ChangeData(funnelData);
            _liquidConfig1.Value = 1111;
            await _liquidChart.UpdateConfig(_liquidConfig1);
        }

        private readonly ColumnConfig _rescueChartConfig = new ColumnConfig
        {
            Title = new Title
            {
                Visible = true,
                Text = "每月救助数量"
            },
            ForceFit = true,
            Padding = "auto",
            XField = "x",
            YField = "y",
            Meta = new
            {
                X = new
                {
                    Alias = "月份"
                },
                Y = new
                {
                    Alias = "救助数量"
                },
            },
        };

        private readonly FunnelConfig _funnelConfig = new FunnelConfig
        {
            Title = new Title
            {
                Visible = true,
                Text = "求助意向转换"
            },
            XField = "action",
            YField = "pv",
            CompareField = "quarter",
            Transpose = true,
        };

        private readonly LiquidConfig _liquidConfig1 = new LiquidConfig
        {
            Title = new Title
            {
                Visible = true,
                Text = "水波图"
            },
            Description = new Description
            {
                Visible = true,
                Text = "水波图 - 收容占比显示"
            },
            Min = 0,
            Max = 10000,
            Value = 4657,
            Statistic = new LiquidStatisticStyle
            {
                //Formatter
            }
        };
    }

}
