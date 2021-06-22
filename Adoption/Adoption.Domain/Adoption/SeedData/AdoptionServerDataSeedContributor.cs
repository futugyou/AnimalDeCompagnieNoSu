using Microsoft.Extensions.Logging;
using System.Threading.Tasks;
using Volo.Abp.Data;
using Volo.Abp.DependencyInjection;

namespace Adoption.Domain.Adoption.SeedData
{
    public class AdoptionServerDataSeedContributor : IDataSeedContributor, ITransientDependency
    {
        private readonly ILogger<AdoptionServerDataSeedContributor> _logger;
        public AdoptionServerDataSeedContributor(ILogger<AdoptionServerDataSeedContributor> logger)
        {
            _logger = logger;
        }

        public Task SeedAsync(DataSeedContext context)
        {
            _logger.LogInformation("this is adoption dataseed");
            return Task.CompletedTask;
        }
    }
}
