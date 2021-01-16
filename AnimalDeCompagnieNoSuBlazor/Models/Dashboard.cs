using System;
using System.Collections.Generic;
using System.Linq;
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
}
