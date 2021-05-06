using Adoption.Application.Contracts.AnimalInfo;
using Adoption.Application.Contracts.Localization.AnimalInfo;
using Adoption.Application.EmailSender;
using Adoption.Domain.Adoption;
using Adoption.Domain.Adoption.DomainEvent;
using System.Collections.Generic;
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
        private readonly IRepository<Animal> _animalRepository;
        private readonly IDistributedEventBus _distributedEventBus;
        private readonly IBackgroundJobManager _backgroundJobManager;

        public AnimalAppService(
            IRepository<Animal> animalRepository,
            IDistributedEventBus distributedEventBus,
            IBackgroundJobManager backgroundJobManager)
        {
            _animalRepository = animalRepository;
            _distributedEventBus = distributedEventBus;
            _backgroundJobManager = backgroundJobManager;
            LocalizationResource = typeof(AnimalInfoResource);
        }

        public async Task<bool> CreateAnimals(CreateAnimalDto animalDto)
        {
            var animal = ObjectMapper.Map<CreateAnimalDto, Animal>(animalDto);
            var result = await _animalRepository.InsertAsync(animal, true);
            await _distributedEventBus.PublishAsync(new AnimalCreatedEto { CardId = animalDto.CardId, Name = animalDto.Name });
            var _result = await _backgroundJobManager.EnqueueAsync(
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
            var animals = await _animalRepository.GetListAsync();
            if (animals.Count == 0)
            {
                throw new UserFriendlyException(L["nodata"]);
            }
            var animalDtos = ObjectMapper.Map<List<Animal>, List<AnimalDto>>(animals);
            return animalDtos;
        }
    }
}
