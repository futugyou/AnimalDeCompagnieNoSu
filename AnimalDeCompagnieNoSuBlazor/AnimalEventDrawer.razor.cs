using AnimalDeCompagnieNoSuBlazor.Models;
using AntDesign;
using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor
{
    public partial class AnimalEventDrawer : FeedbackComponent<string, AnimalEvent>
    {
        private List<SelectType> _eventItems = new()
        {
            new SelectType { Text = "daily", Value = "daily" },
            new SelectType { Text = "born", Value = "born" },
            new SelectType { Text = "ill", Value = "ill" },
            new SelectType { Text = "other", Value = "other" },
        };
        public AnimalEvent AnimalEvent { get; set; } = new AnimalEvent();
        protected override void OnInitialized()
        {
            base.OnInitialized();
        }

        private async void AddNewEvent()
        {
            await ((DrawerRef<AnimalEvent>)base.FeedbackRef)?.CloseAsync(AnimalEvent);
        }
    }
}
