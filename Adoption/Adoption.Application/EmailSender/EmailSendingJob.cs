using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp.BackgroundJobs;
using Volo.Abp.DependencyInjection;

namespace Adoption.Application.EmailSender
{
    public class EmailSendingJob : AsyncBackgroundJob<EmailSendingArgs>, ITransientDependency
    { 
        public EmailSendingJob( )
        { 
        }

        public override async Task ExecuteAsync(EmailSendingArgs args)
        {
            // mock a lang running task.
            await Task.Delay(10000);
        }
    }
}
