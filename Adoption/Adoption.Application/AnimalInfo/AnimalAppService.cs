using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Application.Contracts.Localization;
using Adoption.Application.Contracts.Localization.AnimalInfo;
using Adoption.Application.EmailSender;
using Adoption.Domain.AnimalInfo;
using Microsoft.Extensions.Localization;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp;
using Volo.Abp.Application.Services;
using Volo.Abp.BackgroundJobs;
using Volo.Abp.Domain.Repositories;
using Volo.Abp.EventBus.Distributed;

namespace Adoption.Application.AnimalInfo
{
    public class AnimalAppService : ApplicationService, IAnimalAppService
    {
        private readonly IRepository<Animals> animalRepository;
        private readonly IDistributedEventBus distributedEventBus;
        private readonly IBackgroundJobManager backgroundJobManager;

        public AnimalAppService(
            IRepository<Animals> animalRepository,
            IDistributedEventBus distributedEventBus,
            IBackgroundJobManager backgroundJobManager)
        {
            this.animalRepository = animalRepository;
            this.distributedEventBus = distributedEventBus;
            this.backgroundJobManager = backgroundJobManager;
            LocalizationResource = typeof(AnimalInfoResource);
        }

        public async Task<bool> CreateAnimals(CreateAnimalDto animalDto)
        {
            var animal = ObjectMapper.Map<CreateAnimalDto, Animals>(animalDto);
            var result = await animalRepository.InsertAsync(animal, true);
            await distributedEventBus.PublishAsync(new AnimalCreatedEto { CardId = animalDto.CardId, Name = animalDto.Name });
            var _result = await backgroundJobManager.EnqueueAsync(
                 new EmailSendingArgs
                 {
                     EmailAddress = "this is email address",
                     Subject = "You've successfully registered!",
                     Body = animalDto.ToString(),
                 });

            return result.Id > 0;
        }

        public async Task<List<AnimalDto>> GetAllAnimals()
        {
            var animals = await animalRepository.GetListAsync();
            if (animals.Count == 0)
            {
                throw new UserFriendlyException(L["nodata"]);
            }
            var animalDtos = ObjectMapper.Map<List<Animals>, List<AnimalDto>>(animals);
            return animalDtos;
        }
    }
}
