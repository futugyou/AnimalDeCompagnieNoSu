using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Values;

namespace Adoption.Domain.Adoption
{
    public class Address : ValueObject
    {
        public string Province { get; private set; }
        public string City { get; private set; }

        public string Street { get; private set; }

        public string Home { get; private set; }

        private Address()
        {

        }

        public Address(
            string province,
            string city,
            string street,
            string home)
        {
            Province = province;
            City = city;
            Street = street;
            Home = home;
        }

        protected override IEnumerable<object> GetAtomicValues()
        {
            yield return Province;
            yield return City;
            yield return Street;
            yield return Home;
        }
    }

}
