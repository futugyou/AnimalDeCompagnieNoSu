using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalViewModel
    {
        [JsonPropertyName("id")]
        public string Id { get; set; }
        [JsonPropertyName("idcard")]
        public string Idcard { get; set; }
        [JsonPropertyName("type")]
        public string Type { get; set; }
        [JsonPropertyName("sub_type")]
        public string SubType { get; set; }
        [JsonPropertyName("name")]
        public string Name { get; set; }
        [JsonPropertyName("avatar")]
        public string Avatar { get; set; }
        [JsonPropertyName("age")]
        public double Age
        {
            get
            {
                if (Birthday.HasValue)
                {
                    return (DateTime.Now - Birthday.Value).Days / 365;
                }
                return 0;
            }
        }
        [JsonPropertyName("short_describe")]
        public string ShortDescribe { get; set; }
        [JsonPropertyName("photoes")]
        public List<string> Photoes { get; set; } = new List<string>();
        [JsonPropertyName("birthday")]
        public DateTime? Birthday { get; set; }
    }
}
