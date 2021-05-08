using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Domain.Shared.Localization.Adoption;
using FluentValidation;
using Microsoft.Extensions.Localization;

namespace Adoption.Application.Contracts.FluentValidation
{
    public class CreateAnimalDtoValidator : AbstractValidator<CreateAnimalDto>
    {
        public CreateAnimalDtoValidator(IStringLocalizer<AdoptionInfoResource> localizer)
        {
            RuleFor(x => x.Name).Length(3, 128).WithMessage(x => localizer["needname"]);
            RuleFor(x => x.CardId).Length(3, 128);
        }
    }
}