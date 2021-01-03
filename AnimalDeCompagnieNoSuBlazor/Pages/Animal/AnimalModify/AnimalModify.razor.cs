using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Services;
using AntDesign;
using Microsoft.AspNetCore.Components;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages.Animal
{
    public partial class AnimalModify
    {
        [Parameter]
        public string Id { get; set; }
        [Inject]
        private IAnimalService AnimalService { get; set; }

        public AnimalUpdateModel AnimalUpdateModel = new();

        private int _current;

        public void Next()
        {
            // todo: Not re-rendered
            _current += 1;
            StateHasChanged();
        }

        public void Prev()
        {
            // todo: Not re-rendered
            if (_current <= 0) return;
            _current -= 1;
            StateHasChanged();
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
                return;
            }
            AnimalUpdateModel = await AnimalService.GetAnimalForUpdate(aid);
        }
    }
}