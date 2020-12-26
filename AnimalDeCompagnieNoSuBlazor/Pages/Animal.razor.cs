using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;
using System.Net.Cache;

namespace AnimalDeCompagnieNoSuBlazor.Pages
{
    public partial class Animal
    {
        private readonly ListGridType _listGridType = new ListGridType
        {
            Gutter = 24,
            Column = 4
        };

        private AnimalRawdata[] _data = { };
        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            var list = new List<AnimalRawdata>();
            for (int i = 0; i < 20; i++)
            {
                var data = new AnimalRawdata()
                {
                    Id = i,
                    Name = "this is name " + i,
                    Age = i % 9,
                    ShortDescribe = "https://gw.alipayobjects.com/zos/rmsportal/JiqGstEfoWAOHiTxclqi.png",
                };
                list.Add(data);
            }
            _data = list.ToArray();
        }
    }
}
