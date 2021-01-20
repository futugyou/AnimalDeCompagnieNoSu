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
        private IChartComponent _liquidhouseChart;
        private IChartComponent _liquidfoodChart;
        private IChartComponent _pieChart;
        private IChartComponent _pieageChart;

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            await OnTabChanged("0");
        }

        private async Task OnTabChanged(string v)
        {
            switch (v)
            {
                case "1":
                    var data = await RescueService.GetRescueDataAsync();
                    await _rescueChart.ChangeData(data);
                    break;
                case "2":
                    var funnelData = await RescueService.GetFunnelDataAsync();
                    await _funnelChart.ChangeData(funnelData);
                    break;
                case "3":
                    _liquidfoodConfig.Value = 5670;
                    var task1 = _liquidfoodChart.UpdateConfig(_liquidfoodConfig);
                    _liquidhouseConfig.Value = 1111;
                    var task2 = _liquidhouseChart.UpdateConfig(_liquidhouseConfig);
                    await Task.WhenAll(task1, task2);
                    break;
                case "4":
                    var piedata = await RescueService.GetRescueTypeAsync();
                    await _pieChart.ChangeData(piedata);
                    var pieagedata = await RescueService.GetRescueAgeRangAsync();
                    await _pieageChart.ChangeData(pieagedata);
                    break;
                default:
                    break;
            }
        }

        private readonly StackedAreaConfig _rescueChartConfig = new StackedAreaConfig
        {
            Title = new Title
            {
                Visible = true,
                Text = "每月救助数量"
            },
            XField = "x",
            YField = "y",
            YAxis = new ValueAxis
            {
                Min = 0
            },
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
            SeriesField = "type",
            Color = new[] { "#1ca9e6", "#f88c24" }
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

        private readonly LiquidConfig _liquidhouseConfig = new LiquidConfig
        {
            Title = new Title
            {
                Visible = true,
                Text = "收容占比"
            },
            Description = new Description
            {
                Visible = true,
                Text = "目前救助站的收容数量"
            },
            Min = 0,
            Max = 10000,
            Value = 2222,
            Statistic = new LiquidStatisticStyle
            {
                //Formatter
            }
        };
        private readonly LiquidConfig _liquidfoodConfig = new LiquidConfig
        {
            Title = new Title
            {
                Visible = true,
                Text = "存粮占比"
            },
            Description = new Description
            {
                Visible = true,
                Text = "目前救助站的饲料数量"
            },
            Min = 0,
            Max = 10000,
            Value = 1000,
            Statistic = new LiquidStatisticStyle
            {
                //Formatter
            }
        };
        private readonly PieConfig _pieageConfig = new PieConfig
        {
            ForceFit = true,
            Title = new Title
            {
                Visible = true,
                Text = "年龄段分类"
            },
            Description = new Description
            {
                Visible = false,
                Text = " "
            },
            Radius = 0.8,
            AngleField = "count",
            ColorField = "agerang",
            Label = new PieLabelConfig
            {
                Visible = true,
                Type = "inner"
            }
        };
        private readonly PieConfig _typeConfig = new PieConfig
        {
            ForceFit = true,
            Title = new Title
            {
                Visible = true,
                Text = "种类分类"
            },
            Description = new Description
            {
                Visible = false,
                Text = " "
            },
            Radius = 0.8,
            AngleField = "count",
            ColorField = "type",
            Label = new PieLabelConfig
            {
                Visible = true,
                Type = "inner"
            }
        };

        private IEnumerable<(string type, string content)> GetListData(DateTime dateTime)
        {
            yield return (dateTime.Month + "", dateTime.Day + "");
        }
    }

}
