using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Services;
using AntDesign;
using Microsoft.AspNetCore.Components;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModify
    {
        [Parameter]
        public string Id { get; set; }
        [Inject]
        private IAnimalService AnimalService { get; set; }

        public AnimalUpdateModel AnimalUpdateModel = new();

        private int _current;

        public void Next()
        {
            // todo: Not re-rendered
            _current += 1;
            StateHasChanged();
        }

        public void Prev()
        {
            // todo: Not re-rendered
            if (_current <= 0) return;
            _current -= 1;
            StateHasChanged();
        }

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            AnimalUpdateModel = await AnimalService.GetAnimalForUpdate(Id);
        }
    }
}