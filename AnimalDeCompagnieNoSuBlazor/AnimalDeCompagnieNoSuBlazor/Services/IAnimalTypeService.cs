﻿using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Services
{
    public interface IAnimalTypeService
    {
        Task<List<AnimalType>> GetAllAnimalTypes();
    }
}
