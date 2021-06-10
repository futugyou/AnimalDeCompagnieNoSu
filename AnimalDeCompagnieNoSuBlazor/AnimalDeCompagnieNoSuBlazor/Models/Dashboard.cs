using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class ChartDataItem
    {
        public string X { get; set; }
        public int Y { get; set; }
        public string Type { get; set; }
    }

    public class ChartFunnelType
    {
        public string Action { get; set; }
        public int PV { get; set; }
        public string Quarter { get; set; }
    }

    public class ChartPieType
    {
        public int Count { get; set; }
        public string Type { get; set; }
    }
    public class ChartPieageType
    {
        public int Count { get; set; }
        public string Agerang { get; set; }
    }


    public class RescueClassificationResponse
    {
        [JsonPropertyName("classification")]
        public string Classification { get; set; }
        [JsonPropertyName("count")]
        public int Count { get; set; }
    }     
}
