using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Domain.AnimalInfo;
using AutoMapper;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Application
{
    public class AdoptionApplicationAutoMapperProfile : Profile
    {
        public AdoptionApplicationAutoMapperProfile()
        {
            CreateMap<Animals, AnimalDto>();
        }
    }
}
