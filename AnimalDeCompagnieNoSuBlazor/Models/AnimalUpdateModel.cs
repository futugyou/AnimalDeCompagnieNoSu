using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Models
{
    public class AnimalUpdateModel
    {
        public int Id { get; set; }
        public string Idcard { get; set; }
        public string Type { get; set; }
        public string SubType { get; set; }
        [Required]
        public string Name { get; set; }
        public double Age { get; set; }
        public string ShortDescribe { get; set; }
        public List<string> Photoes { get; set; } = new List<string>();
        public DateTime? Birthday { get; set; }
    }
}
