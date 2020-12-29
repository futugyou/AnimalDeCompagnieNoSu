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


        private AnimalUpdateModel AnimalUpdateModel = new AnimalUpdateModel();

        bool previewVisible = false;
        string previewTitle = string.Empty;
        string imgUrl = string.Empty;

        List<UploadFileItem> fileList = new List<UploadFileItem>();
        List<CascaderNode> selectNodes = new List<CascaderNode>();
        void HandleChange(UploadInfo fileinfo)
        {
            if (fileinfo.File.State == UploadState.Success)
            {
                fileinfo.File.Url = fileinfo.File.ObjectURL;
            }
        }

        public class ResponseModel
        {
            public string name { get; set; }

            public string status { get; set; }

            public string url { get; set; }

            public string thumbUrl { get; set; }
        }
        private void HandleSubmit()
        {
        }
        private void OnAnimalTypeSelected(List<CascaderNode> nodeList, string value, string label)
        {
            Console.WriteLine($"label is {label} value is {value}");
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

            fileList = new List<UploadFileItem>
            {
                new UploadFileItem
                {
                    Id = "1",
                    FileName = "cat01.jpg",
                    State = UploadState.Success,
                    Url =  "/images/cat01.jpg",
                },
                new UploadFileItem
                {
                    Id = "2",
                    FileName = "cat02.jpg",
                    State = UploadState.Success,
                    Url = "/images/cat02.jpg",
                },
                new UploadFileItem
                {
                    Id = "3",
                    FileName = "cat03.jpg",
                    State = UploadState.Success,
                    Url =  "/images/cat03.jpg"
                }
            };
        }

        private async Task GetAnimalTypeData()
        {
            var subtypestring = await AnimalTypeService.GetAllAnimalTypes();
            selectNodes = System.Text.Json.JsonSerializer.Deserialize<List<CascaderNode>>(subtypestring);
        }

        //List<CascaderNode> selectNodes = new List<CascaderNode>()
        //{
        //    new CascaderNode()
        //    {
        //        Value = "1",
        //        Label = "cat",
        //        Children = new CascaderNode[] {
        //            new CascaderNode { Value = "11", Label = "british shorthair", },
        //            new CascaderNode { Value = "12", Label = "Ragdoll" },
        //        }
        //    },
        //    new CascaderNode()
        //    {
        //        Value = "2",
        //        Label = "dog",
        //        Children = new CascaderNode[] {
        //            new CascaderNode { Value = "21", Label = "collie" },
        //            new CascaderNode { Value = "22", Label = "Shepherds" },
        //            new CascaderNode { Value = "23", Label = "golden retriever" },
        //        }
        //    }
        //};

    }
}