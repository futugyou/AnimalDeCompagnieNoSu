﻿using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;
using System;
using Microsoft.AspNetCore.Components.Web;
using AnimalDeCompagnieNoSuBlazor.Services;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalDetail
    {
        [Parameter]
        public string Id { get; set; }
        [Inject] private NavigationManager NavigationManager { get; set; }
        [Inject] private IAnimalService AnimalService { get; set; }
        private AnimalViewModel AnimalViewModel = new AnimalViewModel();
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
            AnimalViewModel = await AnimalService.GetAnimal(aid);
        }
    }
}
