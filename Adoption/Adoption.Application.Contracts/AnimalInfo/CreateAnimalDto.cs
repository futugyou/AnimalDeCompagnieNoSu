using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Application.Dtos;

namespace Adoption.Application.Contracts.AnimalInfo
{
    public class CreateAnimalDto : EntityDto<int>
    {
        public string CardId { get; set; }
        //[Required]
        //[StringLength(128)]
        public string Name { get; set; }
    }
}
