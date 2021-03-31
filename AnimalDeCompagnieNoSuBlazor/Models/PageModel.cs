using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class PageModel
    {
        [JsonPropertyName("pagesize")]
        public int PageSize { get; set; } = 8;
        [JsonPropertyName("pageindex")]
        public int PageIndex { get; set; } = 1;

        public override string ToString()
        {
            return $"pagesize={PageSize}&pageindex={PageIndex}";
        }
    }

    public class PageViewModel
    {
        public int PageIndex { get; set; } = 1;
        public int PageSize { get; set; } = 8;
        public int TotalCount { get; set; } = 50;
    }
}