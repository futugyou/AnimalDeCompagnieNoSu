using Adoption.Application.Contracts.Adoption;
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
            //TODO: need validator all filed
            RuleFor(x => x.AdopterIdNumber).Length(3, 20).WithMessage(x => localizer["needid"]); 
        }
    }
}