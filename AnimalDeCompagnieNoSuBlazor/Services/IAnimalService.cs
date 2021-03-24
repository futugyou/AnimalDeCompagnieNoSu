using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public interface IAnimalService
    {
        Task<List<AnimalListViewModel>> GetAnimalList();
        Task<AnimalViewModel> GetAnimal(string aid);
        Task<AnimalUpdateModel> GetAnimalForUpdate(int aid);
        Task<AnimalViewModel> UpdateAnimal(AnimalUpdateModel animalUpdateModel);
        Task UpdateAnimalAvatar(AnimalAvatarUploadModel animalAvatarUploadNodel);
    }
}
