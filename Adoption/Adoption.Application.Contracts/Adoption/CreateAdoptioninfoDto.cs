using Adoption.Domain.Shared.Adoption;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Application.Dtos;

namespace Adoption.Application.Contracts.Adoption
{
    public class CreateAdoptioninfoDto : EntityDto<Guid>
    {
        public string AdopterName { get; set; }
        public Gender AdopterGender { get; set; }
        public string Phone { get; set; }
        public string AdopterIdNumber { get; set; }
        public string Province { get; set; }
        public string City { get; set; }
        public string Street { get; set; }
        public string Home { get; set; }
        public string AdoptionReasons { get; set; }
        public string AnimalCardId { get; set; }
        public DateTime AnimalBirthday { get; set; }
        public string AnimalName { get; set; }
        public AnimalType AnimalType { get; set; }
    }
}
