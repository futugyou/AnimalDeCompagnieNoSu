using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Application.Dtos;

namespace Adoption.Application.Contracts.AnimalInfo
{
    public class AnimalDto : EntityDto<int>
    {
        public string CardId { get; set; }
        public string Name { get; set; }
    }
}
