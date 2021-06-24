using Adoption.Application.Contracts.Adoption;
using Adoption.Domain.Adoption.Aggregate;
using AutoMapper;

namespace Adoption.Application
{
    public class AdoptionApplicationAutoMapperProfile : Profile
    {
        public AdoptionApplicationAutoMapperProfile()
        {
            CreateMap<CreateAdoptioninfoDto, AdoptionInfo>()
                .ForPath(d => d.Adopter.Address.City, o => o.MapFrom(s => s.City))
                .ForPath(d => d.Adopter.Address.Home, o => o.MapFrom(s => s.Home))
                .ForPath(d => d.Adopter.Address.Province, o => o.MapFrom(s => s.Province))
                .ForPath(d => d.Adopter.Address.Street, o => o.MapFrom(s => s.Street))

                .ForPath(d => d.Adopter.Name, o => o.MapFrom(s => s.AdopterName))
                .ForPath(d => d.Adopter.Gender, o => o.MapFrom(s => s.AdopterGender))
                .ForPath(d => d.Adopter.Phone, o => o.MapFrom(s => s.Phone))
                .ForPath(d => d.Adopter.IdNumber, o => o.MapFrom(s => s.AdopterIdNumber))

                .ForPath(d => d.Animal.AnimalType, o => o.MapFrom(s => s.AnimalType))
                .ForPath(d => d.Animal.CardId, o => o.MapFrom(s => s.AnimalCardId))
                .ForPath(d => d.Animal.Birthday, o => o.MapFrom(s => s.AnimalBirthday))
                .ForPath(d => d.Animal.Name, o => o.MapFrom(s => s.AnimalName))

                .ForPath(d => d.AdoptionReasons, o => o.MapFrom(s => s.AdoptionReasons))
                ;
        }
    }
}
