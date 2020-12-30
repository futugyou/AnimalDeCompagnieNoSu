using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Services;
using AntDesign;
using Microsoft.AspNetCore.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModify
    {
        [Parameter]
        public string Id { get; set; }
        [Inject]
        private NavigationManager NavigationManager { get; set; }
        [Inject]
        private IAnimalTypeService AnimalTypeService { get; set; }
        [Inject]
        private IAnimalService AnimalService { get; set; }

        private AnimalUpdateModel AnimalUpdateModel = new();
        private bool previewVisible = false;
        private string previewTitle = string.Empty;
        private string imgUrl = string.Empty;
        private List<UploadFileItem> fileList = new();
        private List<CascaderNode> selectNodes = new();
        private string with = "80%";
        private void HandleChange(UploadInfo fileinfo)
        {
            if (fileinfo.File.State == UploadState.Success)
            {
                fileinfo.File.Url = fileinfo.File.ObjectURL;
                AnimalUpdateModel.Photoes.Add(fileinfo.File.Url);
            }
        }

        private void HandlePreview(UploadFileItem file)
        {
            Console.WriteLine(file.FileName);
            Console.WriteLine(file.Url);
            previewVisible = true;
            previewTitle = file.FileName;
            imgUrl = file.Url;
        }

        private void HandleModalCancel()
        {
            previewVisible = false;
        }

        private Task<bool> HandleRemove(UploadFileItem file)
        {
            AnimalUpdateModel.Photoes.Remove(file.Url);
            return Task.FromResult(true);
        }

        private async Task HandleSubmitAsync()
        {
            AnimalUpdateModel.Type = GetAnimalTypeBySubType(selectNodes, null, AnimalUpdateModel.SubType);
            await AnimalService.UpdateAnimal(AnimalUpdateModel);
            NavigationManager.NavigateTo("/animal/" + AnimalUpdateModel.Id);
        }

        private void ReturnToDetail()
        {
            NavigationManager.NavigateTo("/animal/" + AnimalUpdateModel.Id);
        }

        private void OnAnimalTypeSelected(List<CascaderNode> nodeList, string value, string label)
        {

        }

        private string GetAnimalTypeBySubType(List<CascaderNode> nodeList, string parent, string target)
        {
            if (nodeList == null) return null;
            if (nodeList.Any(p => p.Value == target))
            {
                return parent == null ? target : parent;
            }

            foreach (var item in nodeList)
            {
                var t = GetAnimalTypeBySubType(item.Children?.ToList(), item.Value, target);
                if (t == null)
                {
                    continue;
                }
                return parent == null ? t : parent + "," + t;
            }
            return null;
        }

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();

            if (!int.TryParse(Id, out var aid) || aid == 0)
            {
                NavigationManager.NavigateTo("/animal");
            }
            await GetAnimalTypeData();
            AnimalUpdateModel = await AnimalService.GetAnimalForUpdate(aid);
            MakePhotoeShow(AnimalUpdateModel?.Photoes);
        }

        private void MakePhotoeShow(List<string> photoes)
        {
            if (photoes != null && photoes.Count > 0)
            {
                foreach (var photo in photoes)
                {
                    var id = Guid.NewGuid().ToString();
                    var file = new UploadFileItem
                    {
                        Id = id,
                        FileName = photo,
                        State = UploadState.Success,
                        Url = photo
                    };
                    fileList.Add(file);
                }
            }
        }

        private async Task GetAnimalTypeData()
        {
            var subtypestring = await AnimalTypeService.GetAllAnimalTypes();
            selectNodes = System.Text.Json.JsonSerializer.Deserialize<List<CascaderNode>>(subtypestring);
        }

        public class ResponseModel
        {
            public string name { get; set; }

            public string status { get; set; }

            public string url { get; set; }

            public string thumbUrl { get; set; }
        }
    }
}