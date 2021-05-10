using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Domain.Shared.Adoption
{
    public static class AdoptionDomainErrorCodes
    {
        public const string AnimalHaveBeenAdoptioned = "Adoption:A00001";
        public const string AuditeNotFinish = "Adoption:A00002";
        public const string AdoptionNotFound = "Adoption:A00003";
        public const string AdoptionFinished = "Adoption:A00004";
        public const string AdoptionRejected = "Adoption:A00005";
        public const string AdoptionCanceled = "Adoption:A00006";
        public const string TargetStatusNotSupport = "Adoption:A00007";
    }
}
