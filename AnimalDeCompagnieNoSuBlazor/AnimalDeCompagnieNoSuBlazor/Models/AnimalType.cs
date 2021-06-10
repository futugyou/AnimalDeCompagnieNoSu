using AntDesign;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalType
    {
        [JsonPropertyName("id")]
        public string Id { get; set; }
        [JsonPropertyName("pid")]
        public string Pid { get; set; }
        [JsonPropertyName("type")]
        public string Type { get; set; }
    }
}
