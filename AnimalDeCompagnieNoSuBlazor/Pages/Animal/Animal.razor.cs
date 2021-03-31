using System.Collections.Generic;
using System.Threading.Tasks;
using AnimalDeCompagnieNoSuBlazor.Models;
using Microsoft.AspNetCore.Components;
using AntDesign;
using System;
using AnimalDeCompagnieNoSuBlazor.Services;
using System.Text.Json.Serialization;
using System.Linq;
using AnimalDeCompagnieNoSuBlazor.Extensions;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class Animal
    {
        [Inject]
        private IAnimalService AnimalService { get; set; }
        [Inject]
        private IAnimalTypeService AnimalTypeService { get; set; }

        private readonly ListGridType _listGridType = new ListGridType
        {
            Gutter = 24,
            Column = 4
        };

        private List<SelectType> _typeItems = new()
        {
            new SelectType { Text = "cat", Value = "cat" },
            new SelectType { Text = "dog", Value = "dog" },
            new SelectType { Text = "fish", Value = "fish" },
            new SelectType { Text = "dark", Value = "dark" }
        };
        private readonly List<SelectType> _truefalseItems = new()
        {
            new SelectType { Text = "是", Value = "0" },
            new SelectType { Text = "否", Value = "1" }
        };


        private IEnumerable<string> _selectedTypeValues = Array.Empty<string>();
        private PageViewModel page = new PageViewModel();

        private async void OnSelectedTypesChangedHandler(IEnumerable<SelectType> values)
        {
            if (values == null || !values.Any())
            {
                _selectedTypeValues = Array.Empty<string>();
            }
            await FillAnimalPage();
            await InvokeAsync(StateHasChanged);
        }

        private async Task FillAnimalPage()
        {
            var request = new AnimalListSearchModel { Type = string.Join(",", _selectedTypeValues) };
            var list = await AnimalService.GetAnimalList(request, new PageModel { PageSize = page.PageSize, PageIndex = page.PageIndex - 1 });
            _data = list.ToArray();
            var datecount = _data.Count();
            _ = datecount switch
            {
                0 => page.TotalCount = page.PageIndex == 1 ? 0 : (page.PageIndex - 1) * page.PageSize,
                _ when datecount == page.PageSize => page.TotalCount = page.PageSize + page.PageIndex * page.PageSize,
                _ => page.TotalCount = page.PageIndex * page.PageSize,
            };
        }

        private IEnumerable<string> _selectedSterilizationValues;

        private async Task OnPageIndexChange(int pageindex)
        {
            page.PageIndex = pageindex;
            await FillAnimalPage();
        }

        private void OnSelectedSterilizationChangedHandler(IEnumerable<SelectType> values)
        {
            //TODO :filter AnimalList
        }

        private IEnumerable<string> _selectedVaccinValues;
        private void OnSelectedVaccinChangedHandler(IEnumerable<SelectType> values)
        {
            //TODO :filter AnimalList
        }

        private AnimalListViewModel[] _data = { };

        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
            var animalTypes = await AnimalTypeService.GetAllAnimalTypes();
            _typeItems = TypeConvertTools.AnimalTypeToSelectType(animalTypes);
            await FillAnimalPage();
        }
    }
    public class SelectType
    {
        public string Value { get; set; }
        public string Text { get; set; }
        public string Group { get; set; }
    }
}
