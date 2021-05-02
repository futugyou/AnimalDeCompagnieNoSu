using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Domain.Shared.Adoption
{
    public enum AdoptionStatus
    {
        Submitted,
        Auditing,
        Audited,
        Reject,
        Cancel,
        Complete,
    }
}
