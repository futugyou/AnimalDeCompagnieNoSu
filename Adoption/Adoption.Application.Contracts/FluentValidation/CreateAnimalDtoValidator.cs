using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Application.Contracts.Localization.AnimalInfo;
using FluentValidation;
using Microsoft.Extensions.Localization;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Application.Contracts.FluentValidation
{
    public class CreateAnimalDtoValidator : AbstractValidator<CreateAnimalDto>
    {
        public CreateAnimalDtoValidator(IStringLocalizer<AnimalInfoResource> localizer)
        {
            RuleFor(x => x.Name).Length(3, 128).WithMessage(x => localizer["needname"]);
            RuleFor(x => x.CardId).Length(3, 128);
        }
    }
}