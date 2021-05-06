using Adoption.Domain.Shared.Adoption;
using Volo.Abp.Domain.Entities;

namespace Adoption.Domain.Adoption
{
    public class Adopter : Entity<int>
    {
        protected Adopter()
        {

        }

        public Adopter(string name, Gender gender, string phone, string idnumber, Address address)
        {
            Name = name;
            Gender = gender;
            Phone = phone;
            IdNumber = idnumber;
            Address = address;
        }
        public virtual string Name { get; private set; }
        public virtual Gender Gender { get; private set; }
        public virtual string Phone { get; private set; }
        public virtual string IdNumber { get; private set; }
        public virtual Address Address { get; private set; }
        public void ChangeAdress(Address address)
        {
            Address = address;
        }
        public void ChangePhone(string phone)
        {
            Phone = phone;
        }
    }
}