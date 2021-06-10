using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public interface IAnimalEventService
    {
        Task<List<AnimalEvent>> GetBigEventByAnimalId(string animalId);
    }
}
