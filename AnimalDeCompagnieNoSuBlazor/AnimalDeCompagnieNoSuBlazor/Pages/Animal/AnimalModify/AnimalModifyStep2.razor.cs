using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using AntDesign;
using AnimalDeCompagnieNoSuBlazor.Services;
using AnimalDeCompagnieNoSuBlazor.Extensions;
using Microsoft.Extensions.Options;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModifyStep2
    {
        [CascadingParameter]
        public AnimalModify AnimalModifyForm { get; set; }
        [Parameter]
        public string Id
        {
            get => _id ?? AnimalModifyForm.Id;
            set
            {
                _id = AnimalModifyForm.Id;
            }
        }
        private string _id;

        [Inject]
        private NavigationManager NavigationManager { get; set; }
        [Inject]
        MessageService MessageService { get; set; }

        [Inject]
        private IAnimalService AnimalService { get; set; }
        [Inject]
        private IOptionsMonitor<AnimalCenter> optionsMonitor { get; set; }

        private AnimalUpdateModel AnimalUpdateModel = new();
        private bool previewVisible = false;
        private string previewTitle = string.Empty;
        private string imgUrl = string.Empty;
        private List<UploadFileItem> fileList = new();
        private string with = "80%";
        private bool loading = false;
        private string uploadaction;
        private Dictionary<string, string> Headers;

        private void HandlePreview(UploadFileItem file)
        {
            previewVisible = true;
            previewTitle = file.FileName;
            imgUrl = file.Url;
        }

        private void HandleChange(UploadInfo fileinfo)
        {
            if (fileinfo.File.State == UploadState.Success)
            {
                var result = fileinfo.File.GetResponse<ResponseModel>();
                fileinfo.File.Url = result.url;
                AnimalUpdateModel.Photoes.Add(result.url);
            }
        }

        private Task<bool> HandleRemove(UploadFileItem file)
        {
            AnimalUpdateModel.Photoes.Remove(file.Url);
            return Task.FromResult(true);
        }

        private void HandleModalCancel()
        {
            previewVisible = false;
        }

        private async Task HandleSubmitAsync()
        {
            loading = true;
            string key = $"updatable-{DateTime.Now.Ticks}";
            var config = new MessageConfig()
            {
                Duration = 0,
                Content = "处理中...",
                Key = key
            };
            _ = MessageService.Loading(config);

            try
            {
                var updateResult = await AnimalService.UpdateAnimal(AnimalUpdateModel);
                config.Duration = 3;
                config.Content = "更新信息成功！";
                //config.OnClose += () => NavigationManager.NavigateTo("/animal/" + AnimalUpdateModel.Id, true);
                await MessageService.Success(config).ContinueWith(_ =>
                {
                    AnimalModifyForm.Next();
                });
            }
            catch (Exception)
            {
                config.Duration = 4;
                config.Content = "更新信息失败！";
                await MessageService.Error(config);
            }
            finally
            {
                loading = false;
            }
        }

        private void ReturnToDetail()
        {
            AnimalModifyForm.Prev();
        }

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            var endpoint = optionsMonitor.CurrentValue;
            uploadaction = endpoint.Host + "api/staticfile";
            Headers = new Dictionary<string, string> { { endpoint.HttpHeadKey, endpoint.HttpHeadValue } };
            AnimalUpdateModel = AnimalModifyForm.AnimalUpdateModel;
            MakePhotoeShow(AnimalUpdateModel?.Photoes);
        }

        private void MakePhotoeShow(List<string> photoes)
        {
            if (photoes != null && photoes.Count > 0)
            {
                foreach (var photo in photoes)
                {
                    if (Tools.IsPicture(photo))
                    {
                        var id = Guid.NewGuid().ToString();
                        var file = new UploadFileItem
                        {
                            Id = id,
                            FileName = photo,
                            State = UploadState.Success,
                            Url = photo,
                            Ext = ".jpg",
                            Type = "image/jpeg",
                        };
                        fileList.Add(file);
                    }
                }
            }
        }
    }
}