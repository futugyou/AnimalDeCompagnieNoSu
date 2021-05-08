using Adoption.Application.Contracts.Adoption;
using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Domain.Adoption.Aggregate;
using AutoMapper;

namespace Adoption.Application
{
    public class AdoptionApplicationAutoMapperProfile : Profile
    {
        public AdoptionApplicationAutoMapperProfile()
        {
            CreateMap<Animal, AnimalDto>();
            CreateMap<CreateAnimalDto, Animal>();
            //TODO: config this in future
            CreateMap<CreateAdoptioninfoDto, AdoptionInfo>();
        }
    }
}
