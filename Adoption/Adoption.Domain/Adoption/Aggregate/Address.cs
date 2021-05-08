using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.Domain.Values;

namespace Adoption.Domain.Adoption.Aggregate
{
    public class Address : ValueObject
    {
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
        public virtual string Province { get; private set; }
        public virtual string City { get; private set; }
        public virtual string Street { get; private set; }
        public virtual string Home { get; private set; }

        protected override IEnumerable<object> GetAtomicValues()
        {
            yield return Province;
            yield return City;
            yield return Street;
            yield return Home;
        }
    }

}
