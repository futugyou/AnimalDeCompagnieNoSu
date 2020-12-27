using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;
using System.Net.Cache;
using System;
using Microsoft.AspNetCore.Components.Web;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalDetail
    {
        [Parameter]
        public string Id { get; set; }
        [Inject] NavigationManager NavigationManager { get; set; }
        private AnimalRawdata AnimalRawdata { get; set; }

        private void GotoUpdateInfoPage(MouseEventArgs e)
        {
            NavigationManager.NavigateTo("/animal/updateinfo/" + AnimalRawdata.Id);
        }

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            if (!int.TryParse(Id, out var aid) || aid == 0)
            {
                NavigationManager.NavigateTo("/animal");
            }
            AnimalRawdata = new AnimalRawdata()
            {
                Id = aid,
                Type = "cat",
                Name = "this is name " + Id,
                Idcard = "cat-" + DateTime.Now.ToString("yyyyMMdd-HHssmm-") + Id,
                Age = aid % 9,
                Photoes = {
                    "/images/cat01.jpg",
                    "/images/cat02.jpg",
                    "/images/cat03.jpg"
                },
            };
        }
    }
}
