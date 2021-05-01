using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.Adoption
{
    public class Adopter : Entity<int>
    {
        protected Adopter()
        {

        }

        public Adopter(string name, string gender, string address, string phone, string idnumber)
        {
            Name = name;
            Gender = gender;
            Address = address;
            Phone = phone;
            IdNumber = idnumber;
        }
        public string Name { get; private set; }
        public string Gender { get; private set; }
        public string Address { get; private set; }
        public string Phone { get; private set; }
        public string IdNumber { get; private set; }
    }
}