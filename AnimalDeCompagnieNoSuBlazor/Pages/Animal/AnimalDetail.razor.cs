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
using Microsoft.Extensions.Options;

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
        [Inject]
        private IOptionsMonitor<ServiceEndpoint> optionsMonitor { get; set; }

        private AnimalViewModel AnimalViewModel = new();
        private List<AnimalEvent> AnimalEvents = new();
        private bool uploadImageVisable = false;
        private string uploadaction;

        private readonly IList<TabPaneItem> _tabList = new List<TabPaneItem>
        {
            new TabPaneItem {Key = "photoes", Tab = "照片墙"},
            new TabPaneItem {Key = "events", Tab = "重大事件"}
        };
        private readonly Dictionary<string, string> Headers = new Dictionary<string, string> { { "apikey", "apivalue" } };
        private string _avatarUrl = "";

        private void HandleChange(UploadInfo fileinfo)
        {
            if (fileinfo.File.State == UploadState.Success)
            {
                var result = fileinfo.File.GetResponse<ResponseModel>();
                fileinfo.File.Url = result.url;
                _avatarUrl = result.url;
            }
        }

        private void HandleOk(MouseEventArgs e)
        {
            //TODO: update animal data
            AnimalViewModel.Avatar = _avatarUrl;
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
            uploadaction = optionsMonitor.CurrentValue.AnimalCenter + "api/staticfile";
            if (!int.TryParse(Id, out var aid) || aid == 0)
            {
                //NavigationManager.NavigateTo("/animal");
            }
            AnimalViewModel = await AnimalService.GetAnimal(Id);
            AnimalEvents = (await AnimalEventService.GetBigEventByAnimalId(aid)).OrderBy(p => p.EventTime).ToList();
        }
    }
}
