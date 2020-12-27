using AnimalDeCompagnieNoSuBlazor.Models;
using AntDesign;
using Microsoft.AspNetCore.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModify
    {
        [Parameter]
        public string Id { get; set; }
        [Inject] NavigationManager NavigationManager { get; set; }
        private AnimalRawdata AnimalRawdata { get; set; }

        bool previewVisible = false;
        string previewTitle = string.Empty;
        string imgUrl = string.Empty;

        List<UploadFileItem> fileList = new List<UploadFileItem>();

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
    }
}
