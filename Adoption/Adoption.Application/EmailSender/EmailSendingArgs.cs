using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Application.EmailSender
{
    public class EmailSendingArgs
    {
        public string EmailAddress { get; set; }
        public string Subject { get; set; }
        public string Body { get; set; }
    }
}
