using AntDesign;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalType
    {
        public int Id { get; set; }
        public string Type { get; set; }
        public List<AnimalType> SubType { get; set; }
    }
}
