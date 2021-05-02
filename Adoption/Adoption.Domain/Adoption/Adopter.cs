using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.Adoption
{
    public class Adopter : Entity<int>
    {
        protected Adopter()
        {

        }

        public Adopter(string name, string gender, string phone, string idnumber, Address address)
        {
            Name = name;
            Gender = gender;
            Phone = phone;
            IdNumber = idnumber;
            Address = address;
        }
        public string Name { get; private set; }
        public string Gender { get; private set; }
        public string Phone { get; private set; }
        public string IdNumber { get; private set; }
        public Address Address { get; private set; }
    }
}