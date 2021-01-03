using AnimalDeCompagnieNoSuBlazor.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components;
using AntDesign;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModifyResult
    {
        [CascadingParameter]
        public AnimalModify AnimalModifyForm { get; set; }

        [Inject]
        private NavigationManager NavigationManager { get; set; }

        private readonly AnimalUpdateModel AnimalUpdateModel = new();
        private void ReturnToDetail()
        {
            NavigationManager.NavigateTo("/animal/" + AnimalUpdateModel.Id);
        }
    }
}
