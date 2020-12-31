using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalEvent
    {
        public Guid Id { get; set; }
        public int AnimalId { get; set; }
        public string Event { get; set; }
        public DateTime EventTime { get; set; }
    }
}
