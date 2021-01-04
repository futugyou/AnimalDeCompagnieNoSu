using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign.Pro.Layout;
using System;
using Microsoft.AspNetCore.Components.Web;
using AnimalDeCompagnieNoSuBlazor.Services;
using System.Linq;
using AntDesign;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalDetail
    {
        [Parameter]
        public string Id { get; set; }
        [Inject]
        private NavigationManager NavigationManager { get; set; }

        [Inject]
        private IAnimalService AnimalService { get; set; }
        [Inject]
        private IAnimalEventService AnimalEventService { get; set; }

        private AnimalViewModel AnimalViewModel = new();
        private List<AnimalEvent> AnimalEvents = new();
        private bool uploadImageVisable = false;

        private readonly IList<TabPaneItem> _tabList = new List<TabPaneItem>
        {
            new TabPaneItem {Key = "photoes", Tab = "照片墙"},
            new TabPaneItem {Key = "events", Tab = "重大事件"}
        };

        private void HandleChange(UploadInfo fileinfo)
        {
            if (fileinfo.File.State == UploadState.Success)
            {
                fileinfo.File.Url = fileinfo.File.ObjectURL;
                AnimalViewModel.Avatar = "/images/head.jpg";
                //TODO: save data
            }
        }

        private void HandleOk(MouseEventArgs e)
        {
            uploadImageVisable = false;
        }

        private void HandleCancel(MouseEventArgs e)
        {
            uploadImageVisable = false;
        }
        private void GotoUpdateInfoPage(MouseEventArgs e)
        {
            NavigationManager.NavigateTo("/animal/updateinfo/" + AnimalViewModel.Id);
        }
        private void OpenFaceChange()
        {
            uploadImageVisable = true;
        }

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            if (!int.TryParse(Id, out var aid) || aid == 0)
            {
                NavigationManager.NavigateTo("/animal");
            }
            AnimalViewModel = await AnimalService.GetAnimal(aid);
            AnimalEvents = (await AnimalEventService.GetBigEventByAnimalId(aid)).OrderBy(p => p.EventTime).ToList();
        }
    }
}
