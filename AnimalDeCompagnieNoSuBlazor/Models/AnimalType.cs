using AntDesign;
using ProtoBuf;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    //public class AnimalType
    //{
    //    [JsonPropertyName("id")]
    //    public string Id { get; set; }
    //    [JsonPropertyName("pid")]
    //    public string Pid { get; set; }
    //    [JsonPropertyName("type")]
    //    public string Type { get; set; }
    //}
    [ProtoContract]
    public class AniamlTypeResponse
    {
        [ProtoMember(1)]
        public List<AnimalType> Item { get; set; }
    }
    [ProtoContract]
    public class AnimalType
    {
        [ProtoMember(1)]
        public string Id { get; set; }

        [ProtoMember(1)]
        public string Pid { get; set; }

        [ProtoMember(1)]
        public string Type { get; set; }
    }
}
