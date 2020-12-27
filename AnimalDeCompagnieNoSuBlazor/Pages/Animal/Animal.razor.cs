using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
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
            for (int i = 1; i <= 20; i++)
            {
                var data = new AnimalRawdata()
                {
                    Id = i,
                    Type = "cat",
                    Name = "this is name " + i,
                    Age = i % 9,
                    Photoes = { "/images/cat01.jpg" },
                };
                list.Add(data);
            }
            _data = list.ToArray();
        }
    }
}
