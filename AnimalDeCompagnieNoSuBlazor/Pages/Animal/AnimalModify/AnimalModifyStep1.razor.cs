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

        private void OnAnimalTypeSelected(List<CascaderNode> nodeList, string value, string label)
        {
            AnimalUpdateModel.SubType = value;
            var node = selectNodes.FirstOrDefault(p => p.Value == value);
            if (node != null)
            {
                AnimalUpdateModel.Type = value;
            }
            else
            {
                AnimalUpdateModel.Type = GetAnimalTypeBySubType(value);
            }
        }

        private string GetAnimalTypeBySubType(string target)
        {
            foreach (var item in selectNodes)
            {
                var parent = GetAnimalTypeBySubTypeLoop(item, item.Children, target);
                if (!string.IsNullOrEmpty(parent))
                {
                    return parent;
                }
            }
            return "";
        }

        private string GetAnimalTypeBySubTypeLoop(CascaderNode item, IEnumerable<CascaderNode> children, string target)
        {
            if (!children.Any())
            {
                return "";
            }
            var sublist = children.FirstOrDefault(p => p.Value == target);
            if (sublist != null)
            {
                return item.Value;
            }
            return GetAnimalTypeBySubTypeLoop(item, children.SelectMany(p => p.Children), target);
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
                AnimalUpdateModel.Type = GetAnimalTypeBySubType(selectNodes, null, AnimalUpdateModel.SubType);
                MessageService.Destroy();
                AnimalModifyForm.Next();
            }
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
