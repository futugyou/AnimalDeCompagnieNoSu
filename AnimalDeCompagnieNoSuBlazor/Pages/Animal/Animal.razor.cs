using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;
using System;
using AnimalDeCompagnieNoSuBlazor.Services;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class Animal
    {
        private readonly ListGridType _listGridType = new ListGridType
        {
            Gutter = 24,
            Column = 4
        };

        private AnimalListViewModel[] _data = { };
        [Inject] private IAnimalService AnimalService { get; set; }
        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            var list = await AnimalService.GetAnimalList();
            _data = list.ToArray();
        }
    }
}
