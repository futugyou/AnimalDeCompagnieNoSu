using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalUpdateModel
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
        [JsonPropertyName("short_describe")]
        public string ShortDescribe { get; set; }
        [JsonPropertyName("photoes")]
        public List<string> Photoes { get; set; } = new List<string>();
        [JsonPropertyName("birthday")]
        public DateTime? Birthday { get; set; }
    }

    public class AnimalAvatarUploadModel
    {
        [JsonPropertyName("id")]
        public string Id { get; set; }
        [JsonPropertyName("avatar")]
        public string Avatar { get; set; }
        [JsonPropertyName("name")]
        public string Name { get; set; }
        [JsonPropertyName("type")]
        public string Type { get; set; }
        [JsonPropertyName("sub_type")]
        public string SubType { get; set; }
    }
}
