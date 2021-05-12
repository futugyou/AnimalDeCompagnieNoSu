using Adoption.Application.Contracts.Adoption;
using Adoption.Domain.Shared.Adoption;
using Adoption.Domain.Shared.Localization.Adoption;
using FluentValidation;
using Microsoft.Extensions.Localization;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Application.Contracts.FluentValidation
{
    public class CreateAdoptioninfoDtoValidator : AbstractValidator<CreateAdoptioninfoDto>
    {
        public CreateAdoptioninfoDtoValidator(IStringLocalizer<AdoptionInfoResource> localizer)
        {
            RuleFor(x => x.AdopterIdNumber).NotEmpty().Length(18).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterId]);
            RuleFor(x => x.AdopterName).NotEmpty().Length(3, 20).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterName]);
            RuleFor(x => x.Phone).NotEmpty().Length(8, 20).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterPhone]);

            RuleFor(x => x.Province).NotEmpty().Length(3, 20).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterAddress]);
            RuleFor(x => x.City).NotEmpty().Length(3, 20).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterAddress]);
            RuleFor(x => x.Street).NotEmpty().Length(3, 40).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterAddress]);
            RuleFor(x => x.Home).NotEmpty().Length(3, 20).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterAddress]);

            RuleFor(x => x.AdoptionReasons).Length(8, 400).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterReason]);
            RuleFor(x => x.AdopterGender).IsInEnum().WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAdopterGender]);

            RuleFor(x => x.AnimalCardId).NotEmpty().Length(3, 28).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAnimalCardId]);
            RuleFor(x => x.AnimalName).NotEmpty().Length(3, 20).WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAnimalName]);
            RuleFor(x => x.AnimalType).IsInEnum().WithMessage(x => localizer[AdoptionValidatorErrorString.InvalidAnimalType]);
        }
    }
}