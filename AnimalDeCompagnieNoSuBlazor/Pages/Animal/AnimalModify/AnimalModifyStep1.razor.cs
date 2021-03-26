using AnimalDeCompagnieNoSuBlazor.Extensions;
using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Services;
using AntDesign;
using Microsoft.AspNetCore.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModifyStep1
    {
        [CascadingParameter]
        public AnimalModify AnimalModifyForm { get; set; }
        [Inject]
        private NavigationManager NavigationManager { get; set; }
        [Inject]
        private ModalService ModalService { get; set; }
        [Inject]
        private MessageService MessageService { get; set; }

        [Inject]
        private IAnimalTypeService AnimalTypeService { get; set; }

        private AnimalUpdateModel AnimalUpdateModel = new();
        private List<CascaderNode> selectNodes = new();
        Form<AnimalUpdateModel> form;


        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            await GetAnimalTypeData();
            AnimalUpdateModel = AnimalModifyForm.AnimalUpdateModel;
        }

        private int AnimalTypeSelectedCount = 0;
        private void OnAnimalTypeSelected(List<CascaderNode> nodeList, string value, string label)
        {
            ///when page load ,the "nodeList" is not complete, it lost parent's path
            ///when choose then option on page ,the "nodeLits" will fix with all path
            ///ant may not fix this ,so i use "AnimalTypeSelectedCount" to record vist times
            if (AnimalTypeSelectedCount > 0 && nodeList != null && nodeList.Any())
            {
                AnimalUpdateModel.SubType = value;
                AnimalUpdateModel.Type = nodeList[0].Value;
            }
            AnimalTypeSelectedCount++;
        } 

        private async Task HandleSubmitAsync()
        {
            string key = $"updatable-{DateTime.Now.Ticks}";
            var config = new MessageConfig()
            {
                Duration = 0,
                Content = "处理中...",
                Key = key
            };
            _ = MessageService.Loading(config);
            customValidator.ClearErrors();

            var errors = new Dictionary<string, List<string>>();

            if (string.IsNullOrEmpty(AnimalUpdateModel.SubType))
            {
                errors.Add(nameof(AnimalUpdateModel.SubType),
                    new List<string>() { "For a 'Defense' ship classification, " +
                "'adnimal subtype' is required." });
            }

            if (errors.Count > 0)
            {
                customValidator.DisplayErrors(errors);
                config.Content = "请填写必要项！";
                config.Duration = 4;
                await MessageService.Warning(config);
            }
            else
            { 
                MessageService.Destroy();
                AnimalModifyForm.Next();
            }
        } 

        private static Task ReturnCancel()
        {
            return Task.CompletedTask;
        }

        private Task ReturnOk()
        {
            NavigationManager.NavigateTo("/animal/" + AnimalUpdateModel.Id);
            return Task.CompletedTask;
        }

        private void ReturnToDetail()
        {
            if (form.IsModified)
            {
                ModalService.Confirm(new ConfirmOptions()
                {
                    Title = "确定放弃现有更改吗？",
                    Icon = icon,
                    OnOk = new Func<ModalClosingEventArgs, Task>(async (e) => await ReturnOk()),
                    OnCancel = new Func<ModalClosingEventArgs, Task>(async (e) => await ReturnCancel()),
                    OkText = "确认",
                    CancelText = "取消",
                    OkType = "danger",
                });
            }
            else
            {
                NavigationManager.NavigateTo("/animal/" + AnimalUpdateModel.Id);
            }
        }

        private async Task GetAnimalTypeData()
        {
            var animalTypes = await AnimalTypeService.GetAllAnimalTypes();
            selectNodes = TypeConvertTools.AnimalTypeToCascaderNode(animalTypes);
        }

    }
}
