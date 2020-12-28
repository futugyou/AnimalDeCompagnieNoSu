using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;
using System;
using Microsoft.AspNetCore.Components.Web;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalDetail
    {
        [Parameter]
        public string Id { get; set; }
        [Inject] private NavigationManager NavigationManager { get; set; }
        private AnimalViewModel AnimalViewModel { get; set; }

        private void GotoUpdateInfoPage(MouseEventArgs e)
        {
            NavigationManager.NavigateTo("/animal/updateinfo/" + AnimalViewModel.Id);
        }
        private readonly Dictionary<string, int> column = new()
        {
            { "xxl", 3 },
            { "xl", 3 },
            { "lg", 2 },
            { "md", 2 },
            { "sm", 1 },
            { "xs", 1 }
        };
        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            if (!int.TryParse(Id, out var aid) || aid == 0)
            {
                NavigationManager.NavigateTo("/animal");
            }
            AnimalViewModel = new AnimalViewModel()
            {
                Id = aid,
                Type = "cat",
                SubType = "British shorthair",
                Name = "this is name " + Id,
                Birthday = DateTime.Now.AddDays(-10),
                Idcard = "cat-" + DateTime.Now.ToString("yyyyMMdd-HHssmm-") + Id,
                Age = 10,
                Photoes = {
                    "/images/cat01.jpg",
                    "/images/cat02.jpg",
                    "/images/cat03.jpg"
                },
            };
        }
    }
}
