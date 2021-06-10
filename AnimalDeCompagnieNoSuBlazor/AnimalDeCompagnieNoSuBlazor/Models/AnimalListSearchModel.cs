using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalListSearchModel
    {
        [JsonPropertyName("name")]
        public string Name { get; set; }
        [JsonPropertyName("type")]
        public string Type { get; set; }
        public override string ToString()
        {
            return string.Format($"?name={Name}&type={Type}");
        }
    }
}
