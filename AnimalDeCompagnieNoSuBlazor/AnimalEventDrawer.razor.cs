using AntDesign;
using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor
{
    public partial class AnimalEventDrawer : AntDomComponentBase
    {
        [Parameter]
        public bool Visible { get; set; }
        [Parameter]
        public bool MaskClosable { get; set; } 
        private void _onClose()
        {
            Visible = false;
            StateHasChanged();
        }

        protected override void OnInitialized()
        {
            base.OnInitialized();
        }
    }
}
